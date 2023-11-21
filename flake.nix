{
  description = "NAHO's Advent of Code solutions";

  inputs = {
    flakeUtils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    preCommitHooks = {
      inputs = {
        flake-utils.follows = "flakeUtils";
        nixpkgs-stable.follows = "preCommitHooks/nixpkgs";
        nixpkgs.follows = "nixpkgs";
      };

      url = "github:trueNAHO/pre-commit-hooks.nix/fix-typos-force-exclude";
    };
  };

  outputs = {
    self,
    flakeUtils,
    nixpkgs,
    preCommitHooks,
    ...
  }:
    flakeUtils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
      in {
        checks.preCommitHooks = preCommitHooks.lib.${system}.run {
          hooks = {
            alejandra.enable = true;
            convco.enable = true;
            rustfmt.enable = true;
            typos.enable = true;
            yamllint.enable = true;
          };

          settings = {
            alejandra.verbosity = "quiet";
            typos.exclude = "*.txt";
          };

          src = ./.;
        };

        devShells.default = pkgs.mkShell {
          inherit (self.checks.${system}.preCommitHooks) shellHook;
          packages = with pkgs; [cargo-watch rustup];
        };
      }
    );
}
