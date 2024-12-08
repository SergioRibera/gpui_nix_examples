{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    crane.url = "github:ipetkov/crane";
    fenix.url = "github:nix-community/fenix";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    flake-utils,
    ...
  } @ inputs: let
      fenix = inputs.fenix.packages;
    in
    # Iterate over Arm, x86 for MacOs 🍎 and Linux 🐧
    (flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
        crane = inputs.crane.mkLib pkgs;
        # Toolchain
        toolchain = fenix.${system}.fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-yMuSb5eQPO/bHv+Bcf/US8LVMbf/G/0MSfiPwBhiPpk=";
        };
        craneLib = crane.overrideToolchain toolchain;

        buildInputs = with pkgs; [
          openssl.dev
          pkg-config
          wayland
        ];

        src = pkgs.lib.cleanSourceWith {
          src = craneLib.path ./.;
          filter = path: type:
            (pkgs.lib.hasInfix "/assets" path)
            || (craneLib.filterCargoSources path type);
        };
        commonArgs = {
          doCheck = false;
          inherit src buildInputs;
          nativeBuildInputs = libraries;
        };

        libraries = with pkgs; [
          vulkan-loader
          libxkbcommon
          wayland
          xorg.libX11
        ];
        # Compile all artifacts
        appDeps = craneLib.buildDepsOnly commonArgs;

        exampleNames = map (file: pkgs.lib.removeSuffix ".rs" file) (builtins.attrNames (builtins.readDir ./examples));

        # Compile
        pkg = name: craneLib.buildPackage (commonArgs// {
          cargoExtraArgs = "--example ${name}";
          cargoArtifacts = appDeps;
        });
        app = name: flake-utils.lib.mkApp {
          drv = pkg name;
        };
      in {
        # nix build
        packages = {
          default = pkg "help_world";
        } // builtins.listToAttrs (map (name: {
          name = name;
          value = pkg name;
        }) exampleNames);

        # nix run
        apps = {
          default = app "help_world";
        } // builtins.listToAttrs (map (name: {
          name = name;
          value = app name;
        }) exampleNames);

        # nix develop
        devShells.default = craneLib.devShell {
          inherit buildInputs;

          packages = [ toolchain ] ++ libraries;

          LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath libraries}";
        };
      }
    ));
}
