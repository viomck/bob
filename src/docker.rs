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

// docker implemented thru CLI because docker API suuucks

use std::io;
use std::io::{Error, ErrorKind};
use std::process::{Command, ExitStatus};

pub(crate) fn login() {
    let username = std::env::var("DOCKER_USERNAME").unwrap();
    let password = std::env::var("DOCKER_TOKEN").unwrap();

    if !Command::new("docker")
        .arg("login")
        .arg("--username")
        .arg(username)
        .arg("--password")
        .arg(password)
        .status()
        .unwrap()
        .success()
    {
        panic!("docker login failed")
    }
}

pub(crate) fn build_and_push(image_dir: &str, image_tag: &str) -> io::Result<()> {
    if !Command::new("docker")
        .arg("build")
        .arg(&image_dir)
        .arg("-t")
        .arg(&image_tag)
        .status()?
        .success()
    {
        return Err(Error::from(ErrorKind::Other));
    }

    if !Command::new("docker")
        .arg("push")
        .arg(&image_tag)
        .status()?
        .success()
    {
        Err(Error::from(ErrorKind::Other))
    } else {
        Ok(())
    }
}
