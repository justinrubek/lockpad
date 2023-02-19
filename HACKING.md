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
