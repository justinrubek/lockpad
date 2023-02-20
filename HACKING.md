# Development guidelines

## Postgres

Postgres is included in the flake's devShell so that you may have a project-specific database.
The shell has scripts to make this easier: `init-database`, `start-database`, and `stop-database`.
These will create a database in `./tmp`, and let you start and stop running it as your local user.
You may get started like this:

```
init-database
psql -h $PWD -d test-db
```

When you are done: `stop-database`.
When you need the database again: `start-database`.

## ScyllaDB

ScyllaDB doesn't build in nix at the time of writing so it has been included in the form of a script which runs a docker container.
After running the script, `run-scylla` or `nix run .#run-scylla`, a container will be launched and should be managed manually using docker.
You may connect to Alternator through `"http://localhost:8100"`.
