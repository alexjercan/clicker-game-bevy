{
  description = "Clicker Game Bevy";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-flake.url = "github:juspay/rust-flake";
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [
        inputs.rust-flake.flakeModules.default
        inputs.rust-flake.flakeModules.nixpkgs
      ];

      systems = ["x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin"];

      perSystem = {
        config,
        lib,
        pkgs,
        ...
      }: let
        rustToolchain = pkgs.rust-bin.nightly."2026-07-03".default.override {
          extensions = ["rust-src" "clippy" "rustfmt"];
          targets = ["wasm32-unknown-unknown"];
        };

        gameLibs = lib.optionals pkgs.stdenv.hostPlatform.isLinux (with pkgs; [
          udev
          alsa-lib-with-plugins
          vulkan-loader
          libx11
          libxcursor
          libxi
          libxrandr
          libxkbcommon
          wayland
        ]);

        unwrapped = config.rust-project.crates.clicker.crane.outputs.drv.crate;

        assets = builtins.path {
          path = ./assets;
          name = "clicker-assets";
        };

        credits = builtins.path {
          path = ./credits;
          name = "clicker-credits";
        };

        clicker = pkgs.stdenvNoCC.mkDerivation {
          pname = "clicker";
          inherit (unwrapped) version;
          dontUnpack = true;
          nativeBuildInputs = [pkgs.makeWrapper];

          installPhase = ''
            runHook preInstall
            mkdir -p $out/bin $out/share/clicker
            ln -s ${assets} $out/share/clicker/assets
            ln -s ${credits} $out/share/clicker/credits
            makeWrapper ${unwrapped}/bin/clicker $out/bin/clicker \
              --set-default BEVY_ASSET_ROOT $out/share/clicker \
              --prefix LD_LIBRARY_PATH : ${lib.makeLibraryPath gameLibs}
            runHook postInstall
          '';

          meta = {
            description = "Clicker game where you collect resources";
            homepage = "https://github.com/alexjercan/clicker-game-bevy";
            license = lib.licenses.mit;
            mainProgram = "clicker";
            platforms = lib.platforms.linux;
          };
        };
      in {
        rust-project = {
          toolchain = rustToolchain;
          crates = lib.mkForce {
            clicker = {
              path = ./.;
              autoWire = [];
              crane.args = {
                buildInputs = gameLibs;
                CARGO_PROFILE = "dist";
                doCheck = false;
              };
              crane.extraBuildArgs.cargoExtraArgs = "--locked --no-default-features -p clicker";
            };
          };
        };

        packages = lib.optionalAttrs pkgs.stdenv.hostPlatform.isLinux {
          inherit clicker;
          clicker-unwrapped = unwrapped;
          default = clicker;
        };

        apps = lib.optionalAttrs pkgs.stdenv.hostPlatform.isLinux {
          default = {
            type = "app";
            program = lib.getExe clicker;
            meta.description = "Launch Clicker";
          };
        };

        checks = lib.optionalAttrs pkgs.stdenv.hostPlatform.isLinux {
          package = clicker;
        };

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            rustToolchain
            rust-analyzer
            cargo-about
            pkg-config
            llvmPackages.bintools
            trunk
            wasm-pack
          ];
          buildInputs = gameLibs;
          LD_LIBRARY_PATH = lib.makeLibraryPath gameLibs;
          RUST_BACKTRACE = "1";
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
        };
      };
    };
}
