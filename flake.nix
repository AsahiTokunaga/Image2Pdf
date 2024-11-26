{
  description = "Image2Pdf cargo project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }: 
    flake-utils.lib.eachDefaultSystem (system:
      let
        inherit (nixpkgs.lib) optional;
        pkgs = import nixpkgs { inherit system; };
        rustup = pkgs.rustup;
        dav1d = pkgs.dav1d;
        nasm = pkgs.nasm;
        pkgconf = pkgs.pkg-config;
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            rustup
            dav1d
            nasm
            pkgconf
          ];
        };
      }
    );
}
