{
  description = "EVE PI Solver";

  inputs = {
    nixpkgs.url = "nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crane = {
      url = "github:ipetkov/crane";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, crane, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        # Setup overlays and pkgs
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        # Node.js setup
        nodejs = pkgs.nodejs_22;
        
        nodeEnv = {
          inherit nodejs;
          npm = nodejs.pkgs.npm;
        };

        # Rust setup
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          targets = [ "wasm32-unknown-unknown" ];
          extensions = [ "rust-src" ];
        };
        
        # Setup crane with toolchain
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
      in
      {
        # Development shell
        devShells.default = pkgs.mkShell {
          buildInputs = [
            # Node.js
            nodeEnv.nodejs
            nodeEnv.npm
            
            # Rust
            rustToolchain
            pkgs.wasm-pack
            pkgs.pkg-config
          ];
          
          # Environment variables
          shellHook = ''
            echo "Node.js $(${nodeEnv.nodejs}/bin/node --version)"
            echo "Rust $(${rustToolchain}/bin/rustc --version)"
            echo "WASM development environment ready"
          '';
        };

        # Build the React application
        packages.default = pkgs.stdenv.mkDerivation {
          name = "eve-pi-solver";
          src = ./.;
          
          buildInputs = [
            # Node.js
            nodeEnv.nodejs
            nodeEnv.npm
            
            # Rust
            rustToolchain
            pkgs.wasm-pack
            pkgs.pkg-config
          ];
          
          buildPhase = ''
            # Ensure npm doesn't try to go online
            export HOME=$TMPDIR
            
            # Install dependencies
            npm install
            
            # Build the application
            npm run build 
          '';
          
          installPhase = ''
            # Copy the build output to the Nix store
            mkdir -p $out
            cp -r build/dist/* $out/
          '';
        };
      }
    );
}