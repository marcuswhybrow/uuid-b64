{
  description = "UUIDs with Base64 Display and serialization";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk.url = "github:nix-community/naersk";
  };

  outputs = inputs: let 
    pkgs = inputs.nixpkgs.legacyPackages.x86_64-linux;
    toolchain = inputs.fenix.packages.x86_64-linux.combine [
      inputs.fenix.packages.x86_64-linux.stable.toolchain
    ];
    naersk = pkgs.callPackage inputs.naersk {
      cargo = toolchain;
      rustc = toolchain;
    };
    cargo = (pkgs.lib.importTOML ./Cargo.toml).package;
  in {
    packages.x86_64-linux.stacks.uuid-b64 = naersk.buildPackage {
      name = cargo.name;
      version = cargo.version;
      src = pkgs.lib.cleanSource ./.;
    };
    packages.x86_64-linux.default = inputs.self.packages.x86_64-linux.uuid-b64;
    devShells.x86_64-linux.default = pkgs.mkShell {
      nativeBuildInputs = [ 
        pkgs.rust-analyzer
        toolchain
      ];
    };
  };
}


