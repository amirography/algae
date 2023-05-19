{
  description = "Description for the project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    devenv.url = "github:cachix/devenv";
    nix2container.url = "github:nlewo/nix2container";
    nix2container.inputs.nixpkgs.follows = "nixpkgs";
    mk-shell-bin.url = "github:rrbutani/nix-mk-shell-bin";
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [
        inputs.devenv.flakeModule
      ];
      systems = ["x86_64-linux"]; # other possible options:"i686-linux" "x86_64-darwin" "aarch64-linux" "aarch64-darwin"

      perSystem = {
        config,
        # , self'
        # , inputs'
        pkgs,
        # system,
        ...
      }: {
        # Per-system attributes can be defined here. The self' and inputs'
        # module parameters provide easy access to attributes of the same
        # system.

        packages.hello = pkgs.cowsay;

        devenv.shells.default = {
          name = "algae default shell";

          # https://devenv.sh/reference/options/
          packages = [config.packages.hello];
          difftastic.enable = true;

          # environmental variables
          env = {};

          # shell command to execute on startup
          enterShell = ''
            cowsay hi
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

              cargo-verify = {
                enable = true;

                # The name of the hook (appears on the report table):
                name = "Cargo Verify";

                # The command to execute (mandatory):
                entry = "cargo verify-project";

                # The pattern of files to run on (default: "" (all))
                # see also https://pre-commit.com/#hooks-files
                # files = "\\.rs$";

                # List of file types to run on (default: [ "file" ] (all files))
                # see also https://pre-commit.com/#filtering-files-with-types
                # You probably only need to specify one of `files` or `types`:
                types = ["file" "non-executable" "text" "rust"];

                # Exclude files that were matched by these patterns (default: [ ] (none)):
                # excludes = ["irrelevant\\.c"];

                # The language of the hook - tells pre-commit
                # how to install the hook (default: "system")
                # see also https://pre-commit.com/#supported-languages
                language = "rust";

                # Set this to false to not pass the changed files
                # to the command (default: true):
                pass_filenames = false;
              };
            };
            settings = {
              clippy.denyWarnings = true;
            };
          };
        };
      };
      flake = {
        # The usual flake attributes can be defined here, including system-
        # agnostic ones like nixosModule and system-enumerating ones, although
        # those are more easily expressed in perSystem.
      };
    };
}
