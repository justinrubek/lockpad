{...} @ part-inputs: {
  imports = [];

  perSystem = {
    pkgs,
    inputs',
    self',
    ...
  }: {
    packages = {
      postgresql = inputs'.nix-postgres.packages."psql_15/bin";
    };
  };
}
