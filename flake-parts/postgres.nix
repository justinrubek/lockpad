{...} @ part-inputs: {
  imports = [];

  perSystem = {
    pkgs,
    inputs',
    self',
    ...
  }: let
    init-database = pkgs.writeScriptBin "init-database" ''
      set -euo pipefail

      ${self'.packages.postgresql}/bin/initdb -D .tmp/test-db
      ${self'.packages.postgresql}/bin/pg_ctl -D .tmp/test-db -l .tmp/test-db.log -o "--unix_socket_directories='$PWD'" start
      ${self'.packages.postgresql}/bin/createdb test-db -h $PWD
    '';

    start-database = pkgs.writeScriptBin "start-database" ''
      set -euo pipefail

      ${self'.packages.postgresql}/bin/pg_ctl -D .tmp/test-db -l .tmp/test-db.log -o "--unix_socket_directories='$PWD'" start
    '';

    stop-database = pkgs.writeScriptBin "stop-database" ''
      set -euo pipefail

      ${self'.packages.postgresql}/bin/pg_ctl -D .tmp/test-db stop
    '';
  in rec {
    packages = {
      postgresql = pkgs.postgresql_15.withPackages (ps: with ps; [inputs'.pgx-ulid.packages.pgx_ulid]);

      "scripts/init-database" = init-database;
      "scripts/start-database" = start-database;
      "scripts/stop-database" = stop-database;
    };
  };
}
