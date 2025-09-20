{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, nixpkgs }: 
  let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};

    nativeBuildInputs = with pkgs; [ rustc cargo live-server lld ]; 
    buildInputs = with pkgs; [ pkg-config alsa-lib xorg.libX11 xorg.libXi libxkbcommon libGL ]; 
    
    # read project metadata from cargo.toml
    cargoTOML = builtins.fromTOML (builtins.readFile ./Cargo.toml);
    pname = cargoTOML.package.name;
    version = cargoTOML.package.version;

    lib = pkgs.lib;
  in
  {

    # declare development environment
    devShells.${system}.default = pkgs.mkShell {
      inherit nativeBuildInputs buildInputs;  
      LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
    };

    packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
      inherit nativeBuildInputs buildInputs pname version;
      LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
      
      cargoLock.lockFile = ./Cargo.lock;
      src = ./.;
    };

  };
}
