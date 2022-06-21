add_executable(binary-tree-vertical-order-traversal src/main.cpp)

target_link_libraries(binary-tree-vertical-order-traversal GTest::gtest_main)
set(binary-tree-vertical-order-traversal_SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR})
set_tests_properties(${binary-tree-vertical-order-traversal} PROPERTIES TIMEOUT 10)
add_test(NAME binary-tree-vertical-order-traversal COMMAND binary-tree-vertical-order-traversal)
