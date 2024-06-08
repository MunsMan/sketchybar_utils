{
  description = "Rust development flake";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rust = pkgs.rust-bin.selectLatestNightlyWith (toolchain:
          toolchain.default.override { extensions = [ "rust-src" ]; });
        darwinInputs = with pkgs;
          lib.optionals stdenv.isDarwin
            (with pkgs.darwin.apple_sdk.framework; [ ]);

      in
      with pkgs; {
        devShells.default = mkShell
          {
            packages = [ rust rust-analyzer-unwrapped ] ++ darwinInputs;
            shellHook = ''
              export COLOR_SCHEMES_DIR=~/.config/color/
              export COLOR_SCHEME=onedark
            '';
          };
        packages = rec {
          color_parser = rustPlatform.buildRustPackage {
            pname = "color_parser";
            version = "0.1.0";
            src = ./color_parser;
            cargoSha256 = "sha256-B/khcqX1COj56ch284g805qZsjDP1+d/uFA6gtxta48=";
          };
          default = pkgs.symlinkJoin
            {
              name = "sketchybar_utils";
              paths = [ color_parser ];
            };
        };
      });
}



