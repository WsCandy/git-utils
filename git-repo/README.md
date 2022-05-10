# git repo
[![Test](https://github.com/WsCandy/git-utils/actions/workflows/rust.yml/badge.svg)](https://github.com/WsCandy/git-utils/actions/workflows/rust.yml)

The git-repo subcommand is simple, it will attempt to open the location of your remote repository in your web browser. For example if your `origin` remote is located on GitHub then your browser will open the repository on GitHub.

### Usage:
```
git repo [remote]
```
`[remote]` is optional, if `[remote]` is not specified then `origin` will be used by default.

#### Examples:

```
git repo
git repo origin
```

### Supports
Supports repositories with remotes located on GitHub and GitLab. Plans to add more at a later date.