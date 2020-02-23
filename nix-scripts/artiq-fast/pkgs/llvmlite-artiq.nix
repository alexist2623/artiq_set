{ stdenv, fetchFromGitHub, llvm-or1k, makeWrapper, python3, ncurses, zlib, python3Packages }:
python3Packages.buildPythonPackage rec {
  name = "llvmlite-artiq";
  src = fetchFromGitHub {
    rev = "158f9d3a898dbf055ca513d69505df288c681fea";
    owner = "m-labs";
    repo = "llvmlite";
    sha256 = "1anniwya5jhhr2sxfdnwrsjy17yrk3x61i9hsm1rljsb8zvh68d5";
  };

  buildInputs = [ makeWrapper python3 ncurses zlib llvm-or1k python3Packages.setuptools ];

  preBuild = "export LLVM_CONFIG=${llvm-or1k}/bin/llvm-config";

  meta = with stdenv.lib; {
      description = "A lightweight LLVM python binding for writing JIT compilers";
      homepage    = "http://llvmlite.pydata.org/";
      maintainers = with maintainers; [ sb0 ];
      license     = licenses.bsd2;
      platforms   = platforms.unix;
  };
}
