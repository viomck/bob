// bob - Docker image build agent
// Copyright (C) 2022 Violet McKinney <opensource@viomck.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use crate::discord_log;
use futures::stream::iter;
use futures::FutureExt;
use octocrab::models::repos::{Commit, FileUpdate};
use octocrab::models::{Repository, User};
use octocrab::Octocrab;
use serde::de::IgnoredAny;
use std::any::Any;
use std::collections::HashMap;

pub(crate) struct GitHub {
    gh: Octocrab,
}

impl GitHub {
    pub(crate) fn init() -> Self {
        Self {
            gh: Octocrab::builder()
                .personal_token(std::env::var("GITHUB_TOKEN").unwrap())
                .build()
                .unwrap(),
        }
    }

    pub(crate) async fn ensure_config(&self) {
        for user in Self::get_watch_users() {
            self.get_repos(user, 1).await.expect("User not found");
        }
    }

    pub(crate) async fn get_watched_repo_info(&self) -> Vec<GitHubInfo> {
        let repo_futs = Self::get_watch_users()
            .into_iter()
            .map(|u| self.get_repos(u, 100))
            .collect::<Vec<_>>();

        let repo_info_futs = futures::future::join_all(repo_futs)
            .await
            .into_iter()
            .map(|r| r.unwrap())
            .flatten()
            .map(|r| self.get_repo_info(r))
            .collect::<Vec<_>>();

        futures::future::join_all(repo_info_futs)
            .await
            .into_iter()
            .filter_map(|i| i)
            .collect()
    }

    async fn get_repo_info(&self, repo: Repository) -> Option<GitHubInfo> {
        let gh = &self.gh;

        let owner_name = repo
            .owner
            .map(|o| o.login)
            .unwrap_or("<unknown>".to_string());

        let content = gh
            .repos(&owner_name, &repo.name)
            .get_content()
            .path("bob_tag.txt")
            .send()
            .await;

        if content.is_err() {
            return None;
        }

        let content = content.unwrap();

        if content.items.len() != 1 {
            discord_log::log(&format!(
                "WARN {}/{}'s bob_tag.txt has {} items (should be 1) - skipping",
                owner_name,
                repo.name,
                content.items.len()
            ))
            .await;
            return None;
        }

        let content = content.items[0].decoded_content();

        if content.is_none() {
            discord_log::log(&format!(
                "WARN {}/{}'s bob_tag.txt has no content",
                owner_name, repo.name
            ))
            .await;
            return None;
        }

        let content = content.unwrap();

        let mut last_commit_params = HashMap::new();
        last_commit_params.insert("per_page", "1");

        let default_branch = repo.default_branch.unwrap_or("main".to_string());

        let last_commit: octocrab::Result<ApiCommitResponse> = gh
            .get(
                gh.absolute_url(format!(
                    "/repos/{}/{}/commits/{}",
                    owner_name, repo.name, default_branch,
                ))
                .unwrap(),
                Some(&last_commit_params),
            )
            .await;

        match last_commit {
            Err(err) => {
                discord_log::log(&format!(
                    "ERROR Could not get last commit for {}/{}/{}: {:?}",
                    owner_name, repo.name, default_branch, err
                ))
                .await;
                None
            }
            Ok(last_commit) => Some(GitHubInfo {
                repo_name: repo.name,
                repo_owner: owner_name,
                bob_tag: content.trim().to_string(),
                latest_commit_sha: last_commit.sha,
            }),
        }
    }

    async fn get_repos(&self, user: String, limit: usize) -> octocrab::Result<Vec<Repository>> {
        let gh = &self.gh;
        let mut params = HashMap::new();
        params.insert("per_page", limit);

        gh.get(
            gh.absolute_url(format!("/users/{}/repos", user)).unwrap(),
            Some(&params),
        )
        .await
    }

    fn get_watch_users() -> Vec<String> {
        std::env::var("WATCH_USERS")
            .expect("WATCH_USERS not set")
            .split(",")
            .map(String::from)
            .collect()
    }
}

#[derive(Debug)]
pub(crate) struct GitHubInfo {
    pub repo_name: String,
    pub repo_owner: String,
    pub bob_tag: String,
    pub latest_commit_sha: String,
}

#[derive(serde::Deserialize)]
struct ApiCommitResponse {
    sha: String,
    node_id: String,
    commit: IgnoredAny,
    url: String,
    html_url: String,
    comments_url: String,
    author: IgnoredAny,
    committer: IgnoredAny,
    files: IgnoredAny,
}
