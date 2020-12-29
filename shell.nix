{ sources ? import ./nix/sources.nix, pkgs ? import ./nix { inherit sources; }
}:

pkgs.mkShell {
  name = "ralsei-shell";

  buildInputs = with pkgs; [
    ((rustChannelOf {
      date = "2020-11-25";
      channel = "nightly";
    }).rust.override { extensions = [ "rustfmt-preview" "clippy-preview" ]; })
    openssl
    pkg-config
    niv
  ];
}
