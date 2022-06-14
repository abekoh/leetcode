add_executable(maximum-width-of-binary-tree src/main.cpp)

target_link_libraries(maximum-width-of-binary-tree GTest::gtest_main)
set(maximum-width-of-binary-tree_SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR})
set_tests_properties(${maximum-width-of-binary-tree} PROPERTIES TIMEOUT 10)
add_test(NAME maximum-width-of-binary-tree COMMAND maximum-width-of-binary-tree)
