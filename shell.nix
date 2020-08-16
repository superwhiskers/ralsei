{ sources ? import ./nix/sources.nix
, pkgs ? import ./nix { inherit sources; } }:

pkgs.mkShell {
  name = "ralsei-shell";

  buildInputs = with pkgs; [
    latest.rustChannels.nightly.rust
    openssl
    pkg-config
    niv
  ];
}
