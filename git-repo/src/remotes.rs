use git2::{Remote, Repository};
use std::collections::HashMap;
use std::error::Error;
use webbrowser;

#[cfg(test)]
mod tests;

#[derive(PartialEq, Debug)]
pub enum Host {
    GitHub,
    GitLab,
}

impl Host {
    pub fn new(remote_host: &str) -> Result<Host, Box<dyn Error>> {
        if remote_host.contains("github.com") {
            return Ok(Host::GitHub);
        }

        if remote_host.contains("gitlab.com") {
            return Ok(Host::GitLab);
        }

        Err(Box::from(format!(
            "Unsupported remote host. Supported hosts: GitHub, GitLab. Got: {}",
            remote_host
        )))
    }

    pub fn get_remote_url(&self, path: &str) -> String {
        match *self {
            Host::GitHub => String::from(format!("https://github.com/{}", path)),
            Host::GitLab => String::from(format!("https://gitlab.com/{}", path)),
        }
    }
}

pub fn get_remotes(repository: &Repository) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let remotes = repository.remotes()?;
    let remotes: Result<Vec<Remote>, _> = remotes
        .iter()
        .filter(|name| name.is_some())
        .map(|name| name.unwrap())
        .map(|name| repository.find_remote(name))
        .collect();

    match remotes {
        Ok(v) => {
            let hash_map: HashMap<String, String> = v
                .into_iter()
                .map(|remote| {
                    let name = String::from(remote.name().unwrap());
                    let url = String::from(remote.url().unwrap());

                    (name, url)
                })
                .collect();

            Ok(hash_map)
        }
        Err(e) => Err(Box::from(e)),
    }
}

pub fn open_remote(repo_url: &str) -> Result<(), Box<dyn Error>> {
    let remote_host = get_remote_host(repo_url)?;
    let remote_path = get_path(repo_url)?;
    let remote_host = Host::new(remote_host)?;

    if let Err(e) = webbrowser::open(&remote_host.get_remote_url(remote_path)) {
        return Err(Box::from(e));
    }

    Ok(())
}

fn get_path(repo_url: &str) -> Result<&str, Box<dyn Error>> {
    match repo_url.split(":").last() {
        Some(value) => Ok(value.trim_end_matches(".git")),
        None => Err(Box::from("Unable to get remote repository path")),
    }
}

fn get_remote_host(repo_url: &str) -> Result<&str, Box<dyn Error>> {
    match repo_url.split(":").next() {
        Some(value) => Ok(value),
        None => Err(Box::from("Unable to get remote repository host")),
    }
}
