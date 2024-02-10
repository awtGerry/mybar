{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  inputsFrom = [ (pkgs.callPackage ./default.nix { }) ];
  buildInputs = with pkgs; [
    dbus
    gdk-pixbuf
    libnotify
    pkg-config
    xorg.libX11

    rust-analyzer
    rustup
    rustc
    cargo
    gcc
    glib
    glibc
  ];
}
