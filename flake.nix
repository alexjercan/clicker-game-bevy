{
  description = "A basic flake for my Bevy Game";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    rust-overlay = {
        url = "github:oxalica/rust-overlay";
        inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils = {
        url = "github:numtide/flake-utils";
        inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
  }: (
    flake-utils.lib.eachDefaultSystem
    (system: let
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs {
        inherit system overlays;

        config = {
          allowUnfree = true;
        };
      };
      manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
    in {
      packages.default = pkgs.rustPlatform.buildRustPackage {
        pname = manifest.name;
        version = manifest.version;
        cargoLock.lockFile = ./Cargo.lock;
        src = pkgs.lib.cleanSource ./.;
      };

      devShells.default = pkgs.mkShell rec {
        nativeBuildInputs = with pkgs; [
          # Rust Compiler
          # cargo
          # rustc
          # rustfmt
          # clippy
          (rust-bin.beta.latest.default.override {
                targets = [ "wasm32-unknown-unknown" ];
            })
          pkg-config
          llvmPackages.bintools
          # Tools
          blender
          # Web
          trunk
          wasm-pack
        ];

        buildInputs = with pkgs; [
          udev
          alsa-lib-with-plugins
          vulkan-loader
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr # To use the x11 feature
          libxkbcommon
          wayland # To use the wayland feature
        ];

        RUST_BACKTRACE = 1;
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };
    })
  );
}
