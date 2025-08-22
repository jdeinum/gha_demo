{
  description = "Rust CI project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      system = "x86_64-linux";
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs {
        inherit system overlays;
      };

      rustToolchain = pkgs.rust-bin.stable.latest.default;
      
      # Read package metadata from Cargo.toml
      cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
      packageName = cargoToml.package.name;
      packageVersion = cargoToml.package.version;

      app = pkgs.rustPlatform.buildRustPackage {
        pname = packageName;
        version = packageVersion;
        
        src = ./.;
        
        cargoLock = {
          lockFile = ./Cargo.lock;
        };

        buildInputs = with pkgs; [
        ];

        nativeBuildInputs = with pkgs; [
        ];
        
        # set env variables
        # PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
        
        # if we require any resources for our tests, set to false, otherwise
        # true
        doCheck = false;
      };

    in
    {
      packages.${system}.default = app;

      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          rustToolchain
          # Optional dev tools
          # rust-analyzer
          # sqlx-cli      # For database migrations
          # postgresql    # For local postgres server
        ];
      };
    };
}
