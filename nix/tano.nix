{
  lib,
  stdenv,
  makeRustPlatform,
  inputs,
  ...
}: let
  toolchain = inputs.fenix.packages.${stdenv.hostPlatform.system}.default.toolchain;
in
  (makeRustPlatform {
    cargo = toolchain;
    rustc = toolchain;
  }).buildRustPackage rec {
    pname = "tano";
    version = "0.1.0";

    src = builtins.path {
      path = ../.;
      name = pname;
    };

    cargoLock.lockFile = ../Cargo.lock;

    meta = {
      description = "A terminal music player";
      homepage = "https://github.com/Ev357/tano";
      license = lib.licenses.mit;
      mainProgram = "tano";
    };
  }
