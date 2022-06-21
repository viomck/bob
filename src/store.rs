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

use crate::{discord_log, GitHubInfo};
use std::fs;
use std::path::Path;
use std::ptr::replace;

pub(crate) fn is_newer_than_stored_sha(ghi: &GitHubInfo) -> bool {
    match get_sha(&ghi) {
        Some(sha) => ghi.latest_commit_sha != sha,
        None => true,
    }
}

pub(crate) fn ensure_store_exists() {
    let path = Path::new("../shas.dat");

    if !path.exists() {
        fs::write(path, "").unwrap();
    }
}

pub(crate) fn store_new_sha(ghi: &GitHubInfo) {
    match get_sha(ghi) {
        Some(old_sha) => replace_sha(&old_sha, &ghi.latest_commit_sha),
        None => append_sha(&ghi.repo_owner, &ghi.repo_name, &ghi.latest_commit_sha),
    }
}

fn replace_sha(old_sha: &str, new_sha: &str) {
    mutate_shas(|old| old.replace(old_sha, new_sha));
}

fn append_sha(repo_owner: &str, repo_name: &str, sha: &str) {
    mutate_shas(|old| format!("{}{}/{} {}\n", &old, &repo_owner, &repo_name, &sha))
}

fn mutate_shas(mutator: impl FnOnce(&str) -> String) {
    fs::write("../shas.dat", mutator(&read_shas())).unwrap()
}

fn read_shas() -> String {
    fs::read_to_string("../shas.dat").unwrap()
}

fn get_sha(ghi: &GitHubInfo) -> Option<String> {
    let res = read_shas();

    for line in res.split("\n").into_iter() {
        let split = line.split_once(" ");

        if split.is_none() {
            continue;
        }

        let (k, v) = split.unwrap();

        if k == format!("{}/{}", ghi.repo_owner, ghi.repo_name) {
            return Some(v.to_string());
        }
    }

    None
}
