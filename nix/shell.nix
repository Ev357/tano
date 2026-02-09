{
  pkgs,
  inputs,
  ...
}:
pkgs.mkShell {
  packages = with pkgs.extend inputs.fenix.overlays.default; [
    inputs.fenix.packages.${stdenv.hostPlatform.system}.default.toolchain
    rust-analyzer-nightly
    sqlx-cli
  ];

  shellHook =
    # bash
    ''
      export DATABASE_URL="sqlite://$XDG_DATA_HOME/tano/database.db"
      export PATH="$PATH:$HOME/.cargo/bin"
    '';

  env = {
    TANO_LOG = "tano=debug";
  };
}
