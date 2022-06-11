add_executable(REPLACE_ME src/main.cpp)

target_link_libraries(REPLACE_ME GTest::gtest_main)
set(REPLACE_ME_SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR})
set_tests_properties(${REPLACE_ME} PROPERTIES TIMEOUT 10)
add_test(NAME REPLACE_ME COMMAND REPLACE_ME)