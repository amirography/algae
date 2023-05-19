{
  description = "Description for the project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default";
    devenv.url = "github:cachix/devenv";
  };

  outputs = {
    nixpkgs,
    devenv,
    systems,
    ...
  } @ inputs: let
    forEachSystem = nixpkgs.lib.genAttrs (import systems);
  in {
    devShells =
      forEachSystem
      (system: let
        pkgs = nixpkgs.legacyPackages.${system};
      in {
        default = devenv.lib.mkShell {
          inherit inputs pkgs;
          modules = [
            {
              # https://devenv.sh/reference/options/
              packages = [pkgs.cowsay];

              # environmental variables
              env = {};

              enterShell = ''
                cowsay Hello!
              '';

              languages.rust = {
                enable = true;
                packages = {
                  cargo = pkgs.cargo;
                  clippy = pkgs.clippy;
                  rust-analyzer = pkgs.rust-analyzer;
                  rust-src = pkgs.rustPlatform.rustLibSrc;
                  rustc = pkgs.rustc;
                  rustfmt = pkgs.rustfmt;
                };
                version = "stable";
              };
              pre-commit = {
                hooks = {
                  # for nix
                  deadnix.enable = true;
                  alejandra.enable = true;
                  nil.enable = true;

                  # for markdown
                  markdownlint.enable = true;

                  # for github
                  actionlint.enable = true;

                  # for git
                  commitizen.enable = true;

                  # for docker
                  hadolint.enable = true;

                  # for rust
                  cargo-check.enable = true;
                  clippy.enable = true;
                  rustfmt.enable = true;

                  # for toml
                  taplo.enable = true;

                  cargo-deny = {
                    enable = true;
                    name = "Cargo Deny check";
                    entry = "cargo deny check";
                    types = ["file" "non-executable" "text" "rust"];
                    language = "rust";
                    pass_filenames = false;
                  };

                  cargo-verify = {
                    enable = true;
                    name = "Cargo Verify";
                    entry = "cargo verify-project";
                    types = ["file" "non-executable" "text" "rust"];
                    language = "rust";
                    pass_filenames = false;
                  };

                  cargo-outdated = {
                    enable = true;
                    name = "Cargo outdated";
                    entry = "cargo outdated --exit-code 1";
                    types = ["file" "non-executable" "text" "rust"];
                    language = "rust";
                    pass_filenames = false;
                  };
                  # problem with runnig a nightly compiler!
                  # cargo-udeps = {
                  #   enable = true;
                  #   name = "Cargo unused dependencies";
                  #   entry = "rustup run nightly cargo udeps";
                  #   types = ["file" "non-executable" "text" "rust"];
                  #   language = "rust";
                  #   pass_filenames = false;
                  # };

                  cargo-audit = {
                    enable = true;
                    name = "Cargo audit";
                    entry = "cargo audit";
                    types = ["file" "non-executable" "text" "rust"];
                    language = "rust";
                    pass_filenames = false;
                  };

                  cargo-pants = {
                    enable = true;
                    name = "Cargo pants";
                    entry = "cargo pants --dev";
                    types = ["file" "non-executable" "text" "rust"];
                    language = "rust";
                    pass_filenames = false;
                  };

                  cargo-test = {
                    enable = true;
                    name = "Cargo test";
                    entry = "cargo test";
                    types = ["file" "non-executable" "text" "rust"];
                    language = "rust";
                    pass_filenames = false;
                  };
                };
                settings = {
                  clippy.denyWarnings = true;
                };
              };
            }
          ];
        };
      });
  };
}
