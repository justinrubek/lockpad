{self, ...} @ part-inputs: {
  imports = [];

  perSystem = {
    pkgs,
    inputs',
    self',
    ...
  }: let
    inherit (pkgs) sqlx-cli;
  in {
    packages = {
      inherit sqlx-cli;
      # copy the migrations directory into a derivation
      sqlx-migrations = pkgs.runCommand "sqlx-migrations" {} ''
        mkdir -p $out
        cp -r ${self}/migrations $out
      '';
    };
  };
}
