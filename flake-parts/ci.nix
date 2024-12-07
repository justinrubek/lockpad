_: {
  perSystem = {
    config,
    pkgs,
    ...
  }: {
    devShells = {
      ci = pkgs.mkShell rec {
        packages = [config.bomper.wrappedBomper];

        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath packages;
      };
    };
  };
}
