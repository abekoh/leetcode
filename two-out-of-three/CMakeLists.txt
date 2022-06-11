add_executable(two-out-of-three src/main.cpp)

target_link_libraries(two-out-of-three GTest::gtest_main)
set(two-out-of-three_SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR})
set_tests_properties(${two-out-of-three} PROPERTIES TIMEOUT 10)
add_test(NAME two-out-of-three COMMAND two-out-of-three)
