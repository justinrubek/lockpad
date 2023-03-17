# Development guidelines

Documentation will assume that you are operating from within the flake's devShell.
[nix-direnv](https://github.com/nix-community/nix-direnv) is highly recommended to use when working with this codebase.

## environment variables

To load environment variables, `.envrc` has been configured to the variables from the  `.direnv/env` directory.
Create this directory and place individual files per variable into it.
Name each file after the corresponding variable name, and the file contents will be loaded and watched by direnv.
The configuration matches the `Config` struct in `crates/cli/src/config.rs`, with `LOCKPAD_` being prefixed to the names.

## Postgres

The flake's devShell includes postgres, allowing you to have a project-specific database for development.
The shell has scripts to make it easier to manage the database: `init-database`, `start-database`, and `stop-database`.
The scripts will create a database in `./tmp`, allowing you start and stop running it as your local user.

To get started, run `init-database` to create a new database.
You can connect to it using `psql -h $PWD -d test-db`.

To stop the database, run `stop-database`.
To start it again: `start-database`.

## ScyllaDB

ScyllaDB doesn't build in nix at the time of writing so it has been included in the form of a script which runs a docker container.
After running the script (`run-scylla` or `nix run .#run-scylla), a container will be launched and should be managed manually using docker.
You may connect to Alternator through `"http://localhost:8100"`.
