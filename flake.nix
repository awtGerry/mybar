{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, flake-utils, naersk, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        naersk' = pkgs.callPackage naersk {};

      in rec {
        # For `nix build` & `nix run`:
        defaultPackage = naersk'.buildPackage {
          nativeBuildInputs = with pkgs; [
            dbus
            gdk-pixbuf
            libnotify
            pkg-config
            xorg.libX11
          ];
          src = ./.;
        };

        # For `nix develop`:
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            dbus
            gdk-pixbuf
            libnotify
            pkg-config
            xorg.libX11

            rust-analyzer
            # rustup
            # rustc
            # cargo
            gcc
            glib
            glibc
          ];
        };
      }
    );
}
