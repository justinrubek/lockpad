{
  inputs,
  self,
  ...
} @ part-inputs: {
  imports = [];

  perSystem = {
    config,
    pkgs,
    lib,
    system,
    inputs',
    self',
    ...
  }: let
    run-scylla = pkgs.writeScriptBin "run-scylla" ''
      docker run \
      -p 8100:8100 \
      --name test-scylla \
      --hostname test-scylla \
      -d scylladb/scylla \
      --smp 1 \
      --developer-mode 1 \
      --alternator-port 8100 \
      --alternator-address 0.0.0.0 \
      --alternator-write-isolation always
    '';
  in rec {
    packages = {
      "scripts/run-scylla" = run-scylla;
    };

    apps = {
      scylla = {
        type = "app";
        program = "${run-scylla}/bin/run-scylla";
      };
    };
  };
}
