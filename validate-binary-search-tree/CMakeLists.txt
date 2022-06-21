add_executable(validate-binary-search-tree src/main.cpp)

target_link_libraries(validate-binary-search-tree GTest::gtest_main)
set(validate-binary-search-tree_SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR})
set_tests_properties(${validate-binary-search-tree} PROPERTIES TIMEOUT 10)
add_test(NAME validate-binary-search-tree COMMAND validate-binary-search-tree)
