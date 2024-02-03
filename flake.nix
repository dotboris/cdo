{
  description = "Lets you run other commands in different directories.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};
      craneLib = crane.mkLib pkgs;

      src = craneLib.cleanCargoSource (craneLib.path ./.);
      commonArgs = {
        inherit src;
        strictDeps = true;
      };
      cargoArtifacts = craneLib.buildDepsOnly commonArgs;
      cdo = craneLib.buildPackage (commonArgs
        // {
          inherit cargoArtifacts;
        });
    in {
      formatter = pkgs.alejandra;

      packages.default = cdo;

      checks = {
        inherit cdo;
        clippy = craneLib.cargoClippy (commonArgs // {inherit cargoArtifacts;});
        test = craneLib.cargoTest (commonArgs // {inherit cargoArtifacts;});
        rustfmt = craneLib.cargoFmt {inherit src;};
        alejandra =
          pkgs.runCommand "alejandra" {
            buildInputs = [pkgs.alejandra];
          } ''
            alejandra -c ${./.}
            mkdir $out
          '';
      };

      devShells.default = craneLib.devShell {
        checks = self.checks.${system};
        # Ensure `rust-analyzer` has access to the rust source code.
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };
    });
}
