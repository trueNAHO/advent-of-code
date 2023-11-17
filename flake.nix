{
  description = "NAHO's Advent of Code solutions";

  inputs = {
    cargo2nix = {
      inputs = {
        flake-compat.follows = "flakeCompat";
        flake-utils.follows = "flakeUtils";
        nixpkgs.follows = "nixpkgs";
      };

      url = "github:cargo2nix/cargo2nix";
    };

    flakeCompat.url = "github:edolstra/flake-compat";
    flakeUtils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    preCommitHooks = {
      inputs = {
        flake-compat.follows = "flakeCompat";
        flake-utils.follows = "flakeUtils";
        nixpkgs-stable.follows = "preCommitHooks/nixpkgs";
        nixpkgs.follows = "nixpkgs";
      };

      url = "github:cachix/pre-commit-hooks.nix";
    };
  };

  outputs = {
    self,
    cargo2nix,
    flakeUtils,
    nixpkgs,
    preCommitHooks,
    ...
  }:
    flakeUtils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [cargo2nix.overlays.default];
        };

        rustPkgs = pkgs.rustBuilder.makePackageSet {
          rustVersion = "1.73.0";
          packageFun = import ./Cargo.nix;
        };
      in {
        checks = {
          packagesTest = self.packages.${system}.test;

          preCommitHooks = preCommitHooks.lib.${system}.run {
            hooks = {
              alejandra.enable = true;
              clippy.enable = true;
              convco.enable = true;
              rustfmt.enable = true;
              typos.enable = true;
              yamllint.enable = true;
            };

            settings.alejandra.verbosity = "quiet";
            src = ./.;
          };
        };

        devShells.default = pkgs.mkShell {
          inherit (self.checks.${system}.preCommitHooks) shellHook;
        };

        packages = {
          build = rustPkgs.workspace.advent_of_code {};
          default = self.packages.${system}.build;
          test = rustPkgs.workspace.advent_of_code {compileMode = "test";};
        };
      }
    );
}
