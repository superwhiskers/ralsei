{ sources ? import ./sources.nix }:

let
  mozilla-overlay = import sources.mozilla-overlay.outPath;
in
import sources.nixpkgs {
  overlays = [
    mozilla-overlay
  ];
  config = {};
}
