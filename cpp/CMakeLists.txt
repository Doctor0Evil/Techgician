cmake_minimum_required(VERSION 3.16)
project(techgician_core_cpp LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(techgician_core STATIC
    src/aln_did_stamp.cpp
    src/validation.cpp
)

target_include_directories(techgician_core PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/include
)

find_package(nlohmann_json 3.10.5 REQUIRED)

target_link_libraries(techgician_core PUBLIC nlohmann_json::nlohmann_json)
