{
  description = "Media tools flake";

  inputs = { nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable"; };

  outputs = { self, nixpkgs }:
    let
      supportedSystems = [ "x86_64-linux" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      nixpkgsFor = forAllSystems (system: import nixpkgs { inherit system; });
      version = self.rev or self.dirtyRev;
    in {
      packages = forAllSystems (system:
        let pkgs = nixpkgsFor.${system};
        in {
          default = pkgs.rustPlatform.buildRustPackage {
            inherit version;
            pname = "media_tools";

            src = ./.;

            cargoLock = { lockFile = ./Cargo.lock; };
          };
        });

      devShells = forAllSystems (system:
        let pkgs = nixpkgsFor.${system};
        in {
          default = pkgs.mkShell { buildInputs = with pkgs; [ cargo rustc ]; };
        });
    };
}
