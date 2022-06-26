add_executable(count-number-of-texts src/main.cpp)

target_link_libraries(count-number-of-texts GTest::gtest_main)
set(count-number-of-texts_SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR})
set_tests_properties(${count-number-of-texts} PROPERTIES TIMEOUT 10)
add_test(NAME count-number-of-texts COMMAND count-number-of-texts)
