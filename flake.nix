{
  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, fenix, flake-utils, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        rust-toolchain = with fenix.packages.${system}; fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-KdTsTZ0EqztXHw1wXt89wAdFvWbVL/WArSjQr1hdYUU=";
        };
      in {
        devShell = pkgs.mkShell rec {
          buildInputs = with pkgs; [
            lld
            rust-toolchain
            pkg-config
            rust-analyzer
            mold-wrapped
            clang
          ];
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;



          # shelHook = ''
          #   cargo install mdbook-echarts
          # '';
        };
      }
    );
}