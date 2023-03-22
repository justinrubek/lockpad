{
  inputs,
  self,
  ...
}: {
  perSystem = {
    pkgs,
    self',
    ...
  }: {
    pre-commit = {
      check.enable = false;

      settings = {
        src = ../.;
        hooks = {
          alejandra.enable = true;
          rustfmt.enable = true;

          # sqlx check custom hook
          sql-prepare = {
            enable = true;
            entry = "cargo sqlx prepare --merged";
            # add `--check` to check only. Without it the file will be updated when the hook is run
            # entry = "cargo sqlx prepare --merged --check";
            pass_filenames = false;
          };
        };
      };
    };
  };
}
