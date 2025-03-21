{
  description = "RustBookEs flake";

  inputs = {
    crane.url = "github:ipetkov/crane";
    fenix.url = "github:nix-community/fenix";
  };

  outputs = { nixpkgs, fenix, ... }@inputs: 
  let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
    lib = pkgs.lib;
    crane = inputs.crane.mkLib pkgs;
    
    toolchain = fenix.packages.${system}.fromToolchainFile {
	file = ./rust-toolchain;
	sha256 = "sha256-opUgs6ckUQCyDxcB9Wy51pqhd0MPGHUVbwRKKPGiwZU=";
    };

    craneLib = crane.overrideToolchain toolchain;

    commonArgs = {
        doCheck = false;
        src = lib.cleanSourceWith {
          src = craneLib.path ./..;
        };
    };

    
    book = craneLib.buildPackage (commonArgs // {
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
    });
  in
  {
    devShells.${system}.default = craneLib.devShell {
       packages = with pkgs; [ toolchain mdbook mdbook-epub];
    };
  };
}
