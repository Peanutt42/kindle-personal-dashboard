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
          extensions = [ "rust-src" ];
        };
      in
      {
        devShells.default = pkgs.mkShell.override { stdenv = pkgs.gccStdenv; } {
          name = "kindle-personal-dashboard";

          nativeBuildInputs = with pkgs; [
            rustToolchain
            rust-analyzer

            pkg-config
            cmake
            ninja
            ccache

            just

            clang-tools # clang-tidy

            # gtk2 libs for native host
            gtk2.dev
            libsysprof-capture
            pcre2.dev
            util-linux.dev # mount package
            libselinux.dev
            glib.dev
            libsepol.dev
            fribidi.dev
            libthai.dev
            libdatrie.dev
            expat.dev
            libxdmcp.dev
            libdeflate
            lerc.dev
            xz.dev
            zstd.dev
            libwebp
          ];

          buildInputs = with pkgs; [
            libclang.lib # for bindgen (see ./core/build.rs)
          ];

          shellHook = "export LIBCLANG_PATH=${pkgs.libclang.lib}/lib";
        };
      }
    );
}
