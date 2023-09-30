{inputs, ...} @ part-inputs: {
  imports = [];

  perSystem = {
    pkgs,
    inputs',
    self',
    ...
  }: let
    # define some values that are used in multiple places.
    # this is used to ensure that all services data is kept in the same directory, and that
    # each service has its own directory within the main one.
    # if there were flake input variables we could determine the dataDir from those, but
    # since there aren't we will allow for an environment variable and a sensible default.
    ports = {
      pg1 = "\${PG1_PORT}";
    };

    global-imports = [
      ({name, ...}: {
        dataDir = "\$PRJ_DATA_HOME/${name}";
      })
    ];
  in rec {
    process-compose = {
      services = {
        imports = [
          inputs.services-flake.processComposeModules.default
        ];

        services.postgres."pg1" = {
          imports = global-imports;

          enable = true;
          package = self'.packages.postgresql;

          listen_addresses = "127.0.0.1";
          port = ports."pg1";
          initialDatabases = [
            {
              name = "lockpad-local";
            }
          ];
        };
      };
    };
  };
}
