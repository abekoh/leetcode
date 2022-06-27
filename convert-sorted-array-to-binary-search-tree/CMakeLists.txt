add_executable(convert-sorted-array-to-binary-search-tree src/main.cpp)

target_link_libraries(convert-sorted-array-to-binary-search-tree GTest::gtest_main)
set(convert-sorted-array-to-binary-search-tree_SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR})
set_tests_properties(${convert-sorted-array-to-binary-search-tree} PROPERTIES TIMEOUT 10)
add_test(NAME convert-sorted-array-to-binary-search-tree COMMAND convert-sorted-array-to-binary-search-tree)
