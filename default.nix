let
  sources = import ./nix/sources.nix {};
  pkgs = import sources.nixpkgs {};

in {
  pnpm = pkgs.nodePackages.pnpm;
}
