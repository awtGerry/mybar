{
  description = "Rust dependency management";

  inputs = {
    nixpkgs.url = "nixpkgs";
  };

  outputs = { self, nixpkgs, ... } @inputs:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
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
      };
    };
}
