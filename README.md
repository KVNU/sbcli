# About

`sbcli` is a command line interface for the SmartBeans[^sb_gitlab] application.

We communicate mainly with the [SmartApe](^https://gitlab.gwdg.de/smart/smartape-dokumentation/) system via its [API](https://gitlab.gwdg.de/smart/smartape-dokumentation/-/wikis/api).

# Design

## High Level Goals

### Core functionality

- Allow users to authenticate on the command line
  - Allow login via username/password or OAuth/LTI
  - Allow login via an auth token -- this should be preferred. The CLI will get a session token from the SmartBeans API by authenticating via the auth token, and store it in memory. This session token will be used for all subsequent requests and will be refreshed when it expires.
- Facilitate a user's ability to interact with the SmartBeans applications exercise directory via the command line.
  - Allow downloading exercises from SmartBeans via the command line
  - Manage a local directory structure for downloaded exercises
  - Allow running tests for local exercises
  - Allow users to submit local exercises to SmartBeans

### Optional functionality

- Facilitate a user's ability to interact with other parts of SmartBeans locally on the CLI, e.g. viewing the leaderboard.

[^sb_gitlab]: https://gitlab.gwdg.de/smart/smartbeans
