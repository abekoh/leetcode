add_executable(max-consecutive-ones-ii src/main.cpp)

target_link_libraries(max-consecutive-ones-ii GTest::gtest_main)
set(max-consecutive-ones-ii_SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR})
set_tests_properties(${max-consecutive-ones-ii} PROPERTIES TIMEOUT 10)
add_test(NAME max-consecutive-ones-ii COMMAND max-consecutive-ones-ii)
