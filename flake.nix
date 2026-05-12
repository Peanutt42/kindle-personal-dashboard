{
  description = "Kindle Personal Dashboard";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          targets = [ "arm-unknown-linux-gnueabi" ];
        };
      in
      {
        devShells.default = pkgs.mkShell.override { stdenv = pkgs.gccStdenv; } {
          name = "kindle-personal-dashboard";

          nativeBuildInputs = with pkgs; [
            rustToolchain

            pkg-config
            cmake
            ninja

            just

            clang-tools # clang-tidy

            gtk2
            gtk2.dev
          ];
        };
      }
    );
}
