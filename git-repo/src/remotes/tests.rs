use super::*;
use std::fs;

#[test]
fn correctly_gets_path() {
    let path = get_path("git@github.com:some/path.git").unwrap_or("error");

    assert_eq!(path, "some/path")
}

#[test]
fn correctly_gets_remote_host() {
    let remote_host = get_remote_host("git@github.com:some/path.git").unwrap_or("error");

    assert_eq!(remote_host, "git@github.com")
}

#[test]
fn creates_github_host() {
    let host = Host::new("git@github.com").unwrap();

    assert_eq!(host, Host::GitHub)
}

#[test]
fn returns_correct_github_url() {
    let host = Host::new("git@github.com").unwrap();

    assert_eq!(host.get_remote_url("test"), "https://github.com/test")
}

#[test]
fn creates_gitlab_host() {
    let host = Host::new("git@gitlab.com").unwrap();

    assert_eq!(host, Host::GitLab)
}

#[test]
fn returns_correct_gitlab_url() {
    let host = Host::new("git@gitlab.com").unwrap();

    assert_eq!(host.get_remote_url("test"), "https://gitlab.com/test")
}

#[test]
fn unsupported_host_causes_error() {
    let host = Host::new("git@invalid.com").map_err(|error| error.to_string());

    assert_eq!(
        host.unwrap_err(),
        "Unsupported remote host. Supported hosts: GitHub, GitLab. Got: git@invalid.com"
    )
}

#[test]
fn gets_correct_remotes() {
    let repository = Repository::init("tmp/test").unwrap();
    let mut expected = HashMap::new();

    if let Err(e) = repository.remote_set_url("origin", "git@github.com:path/test") {
        panic!("{}", e.to_string())
    }

    let remotes = get_remotes(&repository);

    expected.insert(
        String::from("origin"),
        String::from("git@github.com:path/test"),
    );

    // Clean up
    if let Err(e) = fs::remove_dir_all("tmp/test") {
        panic!("{}", e)
    }

    assert_eq!(remotes.unwrap(), expected)
}
