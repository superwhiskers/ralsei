{
  description = "development shell for ralsei";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages."${system}";
      in {
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            # needed for ralsei itself
            openssl
            pkg-config
            rustup
            libxml2

            # needed to compile documentation
            (texlive.combined.scheme-small)
            pandoc
          ];
        };
      });
}
