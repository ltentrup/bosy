cmake_minimum_required(VERSION 3.2)

# This file contains the binary dependencies of BoSy
project(bosy-external-dependencies)

include(ExternalProject)

ExternalProject_Add(spot
    INSTALL_DIR .
    URL http://www.lrde.epita.fr/dload/spot/spot-2.6.2.tar.gz
    URL_HASH SHA256=caf5398def7adc30369d46915499bad88abff0a32e4429113122b6eb19f91146
    CONFIGURE_COMMAND ./configure --disable-python --enable-static --disable-shared --prefix <INSTALL_DIR>
    BUILD_COMMAND make
    BUILD_IN_SOURCE 1
    INSTALL_COMMAND make install
)
