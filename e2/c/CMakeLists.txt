cmake_minimum_required(VERSION 3.0 FATAL_ERROR)
project (threading)

add_executable(atomic atomic.c)
add_executable(mutex mutex.c)
