# Flake specifying both how to package a production binary, and create a
# development environment, all fully declaratively and reproducibly.
{
  description = "Find and randomize cryptographic hashes in text files";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        rustFilterSource = builtins.filterSource (path: _: builtins.match regex path != null);
        regex = ".*/Cargo\.(lock|toml)|.*\.1|.*/src($|/.*)"; # Just include: Cargo.toml, Cargo.lock, *.1, src/**
      in
      {
        # Production package
        packages.hashmash =
          pkgs.rustPlatform.buildRustPackage {
            # Package the binary
            pname = "hashmash";
            version = "1.0.1"; # Keep in sync with Cargo.toml and src/opts.rs
            src = rustFilterSource ./.;
            cargoSha256 = "sha256-P+V1dwzzvCVErhxPIZhU4WSIfxSyvk/7wfNq3UjJe+4=";
            verifyCargoDeps = true;

            # Package the man page
            nativeBuildInputs = [ pkgs.installShellFiles ];
            postInstall = '' installManPage hashmash.1 '';

            # Metadata
            meta = {
              description = "Find and randomize cryptographic hashes in text files";
              homepage = "https://github.com/simonchatts/hashmash";
              license = pkgs.lib.licenses.mit;
              maintainers = [ pkgs.maintainers.simonchatts ];
            };
          };

        defaultPackage = self.packages.${system}.hashmash;

        # Development environment
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustc
            cargo
            clippy
            rust-analyzer
            rustfmt
          ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
            libiconv
          ];
        };
      });
}
