cmake_minimum_required(VERSION 3.2)

project(part1)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_FLAGS "-Wall -O3")

set(SOURCES main.cpp)

add_executable(part1 ${SOURCES})
