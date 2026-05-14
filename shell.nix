{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.gcc-arm-embedded
    (pkgs.python3.withPackages (ps: [
      ps.ar
      ps.pyelftools
    ]))
  ];

  shellHook = ''
    echo "Rust Natmod Development Shell"
    arm-none-eabi-gcc --version
  '';
}
