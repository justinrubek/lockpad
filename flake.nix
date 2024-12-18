{
  description = "A rust project";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    nix-filter.url = "github:numtide/nix-filter";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    pre-commit-hooks = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    bomper = {
      url = "github:justinrubek/bomper";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    pgx-ulid = {
      url = "github:justinrubek/pgx_ulid";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.fenix.follows = "fenix";
      inputs.crane.follows = "crane";
      inputs.flake-parts.follows = "flake-parts";
    };
    process-compose.url = "github:Platonic-Systems/process-compose-flake";
    services-flake.url = "github:justinrubek/services-flake";
    nix-postgres.url = "github:justinrubek/nix-postgres";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux"];
      imports = [
        inputs.pre-commit-hooks.flakeModule

        ./flake-parts/bomper.nix
        ./flake-parts/shells.nix
        ./flake-parts/ci.nix

        ./flake-parts/cargo.nix
        ./flake-parts/rust-toolchain.nix

        ./flake-parts/pre-commit.nix

        ./flake-parts/postgres.nix
        ./flake-parts/sqlx.nix
        ./flake-parts/scylla.nix

        inputs.process-compose.flakeModule
        ./flake-parts/services.nix

        ./containers
      ];
    };
}
