{
  description = "A Nix-flake-based Rust development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
    }:
    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forEachSupportedSystem =
        f:
        nixpkgs.lib.genAttrs supportedSystems (
          system:
          f {
            pkgs = import nixpkgs {
              inherit system;
              overlays = [
                rust-overlay.overlays.default
                self.overlays.default
              ];
            };
          }
        );
    in
    {
      overlays.default = final: prev: {
        rustToolchain =
          let
            rust = prev.rust-bin;
          in
          # rust.stable.latest.default.override {
          #   extensions = [ "rust-src" ];
          #   targets = [ ];
          # };
          rust.nightly."2025-10-29".default.override {
            extensions = [ "rust-src" ];
            targets = [ "wasm32-unknown-unknown" ];
          };
      };

      devShells = forEachSupportedSystem (
        { pkgs }:
        {
          default = pkgs.mkShellNoCC {
            # The Nix packages installed in the dev environment.
            packages = with pkgs; [
              # --- rust --- #
              rustToolchain
              cargo-edit # managing cargo dependencies
              cargo-deny # linting dependencies
              bacon # background code checker

              # --- common --- #
              just # just a command runner

              # --- bevy linux --- #
              pkg-config # build tools
              vulkan-loader # vulkan
              alsa-lib # for sound
              udev # device manager

              # --- bevy wayland --- #
              wayland # wayland client library
              libxkbcommon # keyboard input on Wayland

              # # --- bevy x11 --- #
              # xorg.libX11
              # xorg.libXcursor
              # xorg.libXi
              # xorg.libXrandr
            ];

            env = {
              LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (
                with pkgs;
                [
                  vulkan-loader
                  libxkbcommon
                  # xorg.libX11
                  # xorg.libXi
                  # xorg.libXcursor
                ]
              );
            };

            # The shell script executed when the environment is activated.
            shellHook = ''
              # Print the last modified date of "flake.lock".
              git log -1 --format="%cd" --date=format:"%Y-%m-%d" -- flake.lock |
                awk '{printf "\"flake.lock\" last modified on: %s", $1}' &&
                echo " ($((($(date +%s) - $(git log -1 --format="%ct" -- flake.lock)) / 86400)) days ago)"
            '';
          };
        }
      );
    };
}
