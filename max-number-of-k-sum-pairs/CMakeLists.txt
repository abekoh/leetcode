add_executable(max-number-of-k-sum-pairs src/main.cpp)

target_link_libraries(max-number-of-k-sum-pairs GTest::gtest_main)
set(max-number-of-k-sum-pairs_SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR})
set_tests_properties(${max-number-of-k-sum-pairs} PROPERTIES TIMEOUT 10)
add_test(NAME max-number-of-k-sum-pairs COMMAND max-number-of-k-sum-pairs)
