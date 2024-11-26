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
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            rustup
            dav1d
            nasm
            pkg-config
          ];
        };
      }
    );
}
