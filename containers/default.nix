{
  inputs,
  self,
  lib,
  ...
}: {
  imports = [];

  perSystem = {
    self',
    pkgs,
    lib,
    system,
    inputs',
    ...
  }: let
    skopeo-push = pkgs.writeShellScriptBin "skopeo-push" ''
      set -euo pipefail
      # copy an image to a docker registry
      # 1. image - Given as a path to an image archive
      # 2. registry - The registry to push to
      ${pkgs.skopeo}/bin/skopeo copy --insecure-policy "docker-archive:$1" "docker://$2"
    '';
  in {
    apps = {
      skopeo-push = {
        type = "app";
        program = "${skopeo-push}/bin/skopeo-push";
      };
    };
    packages = {
      "scripts/skopeo-push" = skopeo-push;

      "postgres-init" = pkgs.writeScriptBin "postgres_init" ''
        #!${pkgs.runtimeShell}
        sudo -u postgres /bin/initdb --locale=C -D /data
        sudo -u postgres ln -s /etc/postgresql.conf /data/postgresql.conf
        sudo -u postgres /bin/postgres -p 5432 -D /data
      '';

      "postgres/docker" = let
        postgres-config = pkgs.runCommand "default-postgresql.conf" {} ''
          mkdir -p $out/etc
          cp ${./postgres/postgres.conf} $out/etc/postgresql.conf
        '';

        postgres = self'.packages.postgresql;
      in
        pkgs.dockerTools.buildImage {
          name = "postgres-lockpad";
          tag = "latest";

          runAsRoot = ''
            $!${pkgs.runtimeShell}
            ${pkgs.dockerTools.shadowSetup}
            groupadd -r postgres
            useradd -r -g postgres postgres
            mkdir -p /data /run/postgresql
            chown postgres:postgres /data /run/postgresql
          '';

          copyToRoot = pkgs.buildEnv {
            name = "image-root";
            paths = [postgres pkgs.coreutils pkgs.bash pkgs.lsd pkgs.sudo self'.packages."postgres-init" pkgs.dockerTools.binSh];
            pathsToLink = ["/bin" "/etc" "/share" "/var"];
          };

          config = {
            Cmd = ["/bin/postgres_init"];
            ExposedPorts = {"5432/tcp" = {};};
            WorkingDir = "/data";
            Volumes = {"/data" = {};};
          };
        };

      "lockpad/docker" = pkgs.dockerTools.buildImage {
        name = "lockpad";
        tag = self.rev or "dirty";

        copyToRoot = pkgs.buildEnv {
          name = "image-root";
          paths = [
            self'.packages.cli
          ];
          pathsToLink = ["/bin"];
        };

        config = {
          Cmd = ["/bin/lockpad-cli"];
          WorkingDir = "/app";
        };
      };
    };
  };
}
