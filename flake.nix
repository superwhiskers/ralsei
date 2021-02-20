{
  description = "development shell for ralsei";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    nixpkgs-mozilla = {
      url = "github:mozilla/nixpkgs-mozilla";
      flake = false;
    };
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, mozilla, utils }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages."${system}";
        mozilla = import "${nixpkgs-mozilla}/package-set.nix" pkgs;
      in {
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs;
            [ openssl pkg-config ] ++ [
              ((mozilla.rustChannelOf { channel = "nightly"; }).rust.override {
                extensions = [ "rustfmt-preview" "clippy-preview" ];
              })
            ];
        };
      });
}
