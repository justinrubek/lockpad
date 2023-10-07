{...} @ part-inputs: {
  imports = [];

  perSystem = {
    pkgs,
    inputs',
    self',
    ...
  }: {
    packages = {
      postgresql = pkgs.postgresql_15.withPackages (ps: [inputs'.pgx-ulid.packages.pgx_ulid]);
    };
  };
}
