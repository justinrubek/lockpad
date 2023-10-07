# Development guidelines

Documentation will assume that you are operating from within the flake's devShell.
[nix-direnv](https://github.com/nix-community/nix-direnv) is highly recommended to use when working with this codebase.

## environment variables

To load environment variables, `.envrc` has been configured to the variables from the  `.direnv/env` directory.
Create this directory and place individual files per variable into it.
Name each file after the corresponding variable name, and the file contents will be loaded and watched by direnv.
The configuration matches the `Config` struct in `crates/cli/src/config.rs`, with `LOCKPAD_` being prefixed to the names.

## services

As a convenience the flake is equipped with [services-flake](https://github.com/justinrubek/services-flake) to provide an interface for the services needed for local development.
To launch services use `nix run .#services` and [process-compose](https://github.com/F1bonacc1/process-compose) will start.

### Postgres

The flake's package output includes postgres, allowing you to access the pinned version directly if needed.

Migrations are managed using sqlx, see the [sqlx-cli usage page](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md#usage=) for information on how to use it.

Common commands:
- `sqlx migrate add <name>`
- `sqlx migrate run`
- `sqlx migrate revert`


### pre-commit-hooks

The devShell has been equipped with a pre-commit hook that prepares sqlx to ensure type-checking can be performed.
This is especially important to do because we don't have a database available to check against in the build environment.
You'll need to ensure you have the `DATABASE_URL` environment variable set. You can make it a symlink to `LOCKPAD_POSTGRES_URL` or manually keep them in sync.
