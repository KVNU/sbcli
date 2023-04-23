# About

Current `MVP` version: `0.1.0`
Some features are still missing, but the core functionality is there.

`sbcli` is a command line interface for the SmartBeans[^sb_gitlab] application.

We communicate mainly with the [SmartApe](^https://gitlab.gwdg.de/smart/smartape-dokumentation/) system via its [API](https://gitlab.gwdg.de/smart/smartape-dokumentation/-/wikis/api).

# Installation

We don't have a release yet, so you'll have to build from source.

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

OpenSSL is required for the `reqwest` crate, which we use for HTTP requests. See [here](https://docs.rs/reqwest/latest/reqwest/index.html#tls) for more information.

### Building

`cargo build --release`

# Usage

### Prerequisites

A SmartBeans account which has password authentication enabled. Token login is wip.

### Commands

The CLI is organized into subcommands. To see a list of available subcommands, run `sbcli --help`.
You'll need to configure the CLI before you can use it. Run
```
sbcli config -u USERNAME -c COURSE --host HOST
```
 with all required arguments to do so.

If you're not logged in, it'll ask if you want to do so. Otherwise, run `sbcli login` to log in.

Then you'll want to run `sbcli sync` to download the exercise directory from SmartBeans. This will create a directory called `tasks` in the current working directory. You can change this by setting the `SBCLI_EXERCISE_DIR` environment variable(wip).

Next, you can run `sbcli start` to start a new exercise. By default, this will open the next exercise in order using your default editor.

Once you're done, you can run `sbcli submit PATH_TO_SOLUTION` to submit your solution to SmartBeans.

It's currently **not** possible to submit files which have been **moved or renamed**. This will be fixed in a future release.

Run `sbcli list` to see a list of all exercises in the exercise directory and their current status.

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
- Gamification features

[^sb_gitlab]: https://gitlab.gwdg.de/smart/smartbeans
