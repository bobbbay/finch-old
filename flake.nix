{
  inputs.utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, utils} @ inputs: utils.lib.simpleFlake {
    inherit self nixpkgs;
    name = "finch";
    shell = ./shell.nix;
  };
}
