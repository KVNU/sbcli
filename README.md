# About

`sbcli` is a command line interface for the SmartBeans[^sb_gitlab] application.

# Design

## High Level Goals

### Core functionality

- Allow users to authenticate on the command line
  - Allow login via username/password or OAuth
- Facilitate a user's ability to interact with the SmartBeans applications exercise directory via the command line.
  - Allow downloading exercises from SmartBeans via the command line
  - Manage a local directory structure for downloaded exercises
  - Allow running tests for local exercises
  - Allow users to submit local exercises to SmartBeans

### Optional functionality

- Facilitate a user's ability to interact with other parts of SmartBeans locally on the CLI, e.g. viewing the leaderboard.

[^sb_gitlab]: https://gitlab.gwdg.de/smart/smartbeans
