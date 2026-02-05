cmake_minimum_required(VERSION 3.10)
project(calculator C)

# Set C standard
set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)

# Build static library
add_library(calculator STATIC
        c_lib/calculator.c
        c_lib/analytics.c
)

# Include directories
target_include_directories(calculator PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}/c_lib
)
