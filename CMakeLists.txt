cmake_minimum_required(VERSION 3.22)
project(leetcode)

set(CMAKE_CXX_STANDARD 20)

include(conan.cmake)

option(RUN_TESTS "Build the tests" ON)
if(RUN_TESTS)
    enable_testing()
    find_package(GTest)
endif()

add_subdirectory(merge-two-sorted-lists)
add_subdirectory(number-of-equal-count-substrings)
add_subdirectory(two-out-of-three)
add_subdirectory(max-consecutive-ones-ii)
add_subdirectory(fibonacci-number)
