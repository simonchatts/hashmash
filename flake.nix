# hashmash flake
{
  description = "Find and randomize cryptographic hashes in text files";

  outputs = { self, nixpkgs, flake-lib }:
    let
      flib = flake-lib.outputs;
      name = (flib.readCargoToml ./.).name;
      forAllSystems = flib.forAllSystemsWith [ self.overlay ];
    in
    {
      # Overlay and default build artefacts
      overlay = final: prev: { "${name}" = final.callPackage ./. { inherit flib; }; };
      packages = forAllSystems (pkgs: { "${name}" = pkgs."${name}"; });
      defaultPackage = forAllSystems (pkgs: pkgs."${name}");

      # Development environment
      devShell = forAllSystems (pkgs: with pkgs; mkShell {
        # Host development environment
        nativeBuildInputs = [
          rustc
          cargo
          clippy
          rust-analyzer
          rustfmt
          nixpkgs-fmt
        ];

        # Build inputs
        buildInputs = lib.optionals stdenv.isDarwin [ libiconv ];
      });

      # Basic CI checks
      checks = forAllSystems (pkgs: {
        format = flib.checkRustFormat ./. pkgs;
      });
    };

  inputs = {
    flake-lib.url = "git+ssh://git@github.com/simonchatts/flake-lib?ref=main";
    flake-lib.inputs.nixpkgs.follows = "nixpkgs";
  };
}
