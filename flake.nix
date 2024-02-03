{
  description = "Lets you run other commands in different directories.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay/stable";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
  };

  # Flake outputs
  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        # Overlays enable you to customize the Nixpkgs attribute set
        overlays = [
          # Makes a `rust-bin` attribute available in Nixpkgs
          (import rust-overlay)
          # Provides a `rustToolchain` attribute for Nixpkgs that we can use to
          # create a Rust environment
          (self: super: {
            rustToolchain = super.rust-bin.stable.latest.default;
          })
        ];
        pkgs = import nixpkgs { inherit system overlays; };
      in
      {
        packages = {
          default = pkgs.rustPlatform.buildRustPackage {
            name = "cdo";
            src = ./.;
            cargoLock = { lockFile = ./Cargo.lock; };
          };
        };

        # Development environment output
        devShells = {
          default = pkgs.mkShell {
            # The Nix packages provided in the environment
            packages = (with pkgs; [
              # The usual suite for rust tools including cargo, Clippy,
              # cargo-fmt rustdoc, rustfmt, and other tools.
              rustToolchain
              # To format this file (how meta)
              nixpkgs-fmt
            ]) ++ pkgs.lib.optionals pkgs.stdenv.isDarwin
              (with pkgs; [ libiconv ]);
          };
        };
      });
}
