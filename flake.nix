# hashmash flake
#
# Considerable inspiration taken from
# https://hoverbear.org/blog/a-flake-for-your-crate/
{
  description = "Find and randomize cryptographic hashes in text files";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      # Admin
      cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
      name = cargoToml.package.name;
      systems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forAllSystems = f: nixpkgs.lib.genAttrs systems (system:
        let pkgs = import nixpkgs {
          inherit system;
          overlays = [ self.overlay ];
        }; in f pkgs);
    in
    {
      # Overlay
      overlay = final: prev: { "${name}" = final.callPackage ./. { }; };

      # Packages (built by `nix build .#<name>`)
      packages = forAllSystems (pkgs: { "${name}" = pkgs."${name}"; });

      # Default Package (built by `nix build .`)
      defaultPackage = forAllSystems (pkgs: pkgs."${name}");

      # Development environment
      devShell = forAllSystems (pkgs: import ./shell.nix { inherit pkgs; });

      # Basic CI checks
      checks = forAllSystems (pkgs: {
        "${name}" = pkgs."${name}";

        # Source code formatting.
        format = pkgs.runCommand "check-format"
          { buildInputs = [ pkgs.cargo pkgs.rustfmt pkgs.nixpkgs-fmt ]; }
          ''
            ${pkgs.cargo}/bin/cargo fmt --manifest-path ${./.}/Cargo.toml -- --check
            ${pkgs.nixpkgs-fmt}/bin/nixpkgs-fmt --check ${./.}
            touch $out # success
          '';

        # Doesn't work yet, and is also slow (re-downloads crates.io)
        #
        # clippy = pkgs.runCommand "clippy"
        #   { buildInputs = [ pkgs.cargo pkgs.clippy ]; }
        #   ''
        #     CARGO_HOME=. ${pkgs.cargo}/bin/cargo clippy --manifest-path ${./.}/Cargo.toml -- -D warnings
        #     touch $out # success
        #   '';

      });
    };
}
