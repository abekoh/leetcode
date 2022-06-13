add_executable(fibonacci-number src/main.cpp)

target_link_libraries(fibonacci-number GTest::gtest_main)
set(fibonacci-number_SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR})
set_tests_properties(${fibonacci-number} PROPERTIES TIMEOUT 10)
add_test(NAME fibonacci-number COMMAND fibonacci-number)
