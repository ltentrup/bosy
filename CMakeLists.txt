cmake_minimum_required(VERSION 3.2)

# This file contains the binary dependencies of BoSy
project(bosy-external-dependencies)

include(ExternalProject)

ExternalProject_Add(spot
    INSTALL_DIR .
    URL http://www.lrde.epita.fr/dload/spot/spot-2.8.7.tar.gz
    URL_HASH SHA256=bdbeb4454f0244b6b77e1b04458558ab62471aaa21918e104602e6da99969c1f
    CONFIGURE_COMMAND ./configure --disable-python --enable-static --disable-shared --prefix <INSTALL_DIR>
    BUILD_COMMAND make
    BUILD_IN_SOURCE 1
    INSTALL_COMMAND make install
)

ExternalProject_Add(z3
    INSTALL_DIR .
    GIT_REPOSITORY https://github.com/Z3Prover/z3.git
    GIT_TAG z3-4.8.7
    BUILD_IN_SOURCE 1
    CONFIGURE_COMMAND python scripts/mk_make.py --prefix=<INSTALL_DIR>
    BUILD_COMMAND make -C build
    INSTALL_COMMAND cp <SOURCE_DIR>/build/z3 <INSTALL_DIR>/bin/
)
