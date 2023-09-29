cmake_minimum_required(VERSION 3.8)

project(verona-lib CXX)


set(VERONA_PATH "../external/verona-rt")

add_subdirectory(${VERONA_PATH} verona-runtime)

set(CMAKE_CXX_STANDARD 17)

find_package(Threads REQUIRED)
add_library(verona STATIC verona.cc)
target_link_libraries(verona verona_rt)
target_compile_options(verona PRIVATE -fpie)
