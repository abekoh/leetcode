add_executable(merge-two-sorted-lists src/main.cpp)

target_link_libraries(merge-two-sorted-lists GTest::gtest_main)
set(merge-two-sorted-lists_SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR})
set_tests_properties(${merge-two-sorted-lists} PROPERTIES TIMEOUT 10)
add_test(NAME merge-two-sorted-lists COMMAND merge-two-sorted-lists)