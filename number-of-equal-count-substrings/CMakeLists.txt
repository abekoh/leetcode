add_executable(number-of-equal-count-substrings src/main.cpp)

target_link_libraries(number-of-equal-count-substrings GTest::gtest_main)
set(number-of-equal-count-substrings_SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR})
set_tests_properties(${number-of-equal-count-substrings} PROPERTIES TIMEOUT 10)
add_test(NAME number-of-equal-count-substrings COMMAND number-of-equal-count-substrings)