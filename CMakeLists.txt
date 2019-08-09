cmake_minimum_required(VERSION 3.2)

# This file contains the binary dependencies of BoSy
project(bosy-external-dependencies)

include(ExternalProject)

ExternalProject_Add(spot
    INSTALL_DIR .
    URL http://www.lrde.epita.fr/dload/spot/spot-2.8.1.tar.gz
    URL_HASH SHA256=dcb7aa684725304afb3d435f26f25b51fbd6e9a6ef610e16163cc0030ad5eab4
    CONFIGURE_COMMAND ./configure --disable-python --enable-static --disable-shared --prefix <INSTALL_DIR>
    BUILD_COMMAND make
    BUILD_IN_SOURCE 1
    INSTALL_COMMAND make install
)
