cmake_minimum_required(VERSION 3.13)
set(CMAKE_TOOLCHAIN_FILE /opt/stm32-cmake/cmake/stm32_gcc.cmake)
set(STM32_CUBE_L4_PATH /opt/cubemx/STM32CubeL4)

project(klarimaus C ASM)

find_package(CMSIS COMPONENTS STM32L412 REQUIRED)
find_package(HAL COMPONENTS STM32L4 REQUIRED)
set(CMAKE_INCLUDE_CURRENT_DIR TRUE)
 

set(PROJECT_SOURCES
    main.c
    hardware.c
    stm32l4xx_it.c
)

add_executable(klarimaus ${PROJECT_SOURCES})
link_directories(lib)

target_link_libraries(klarimaus
    HAL::STM32::L4::RCC
    HAL::STM32::L4::RCCEx
    HAL::STM32::L4::GPIO
    HAL::STM32::L4::I2C
    HAL::STM32::L4::I2CEx
    HAL::STM32::L4::CORTEX
    HAL::STM32::L4::PWR
    HAL::STM32::L4::PWREx
    HAL::STM32::L4::TIM
    HAL::STM32::L4::TIMEx
    CMSIS::STM32::L412KB
    ${CMAKE_SOURCE_DIR}/lib/libstatic_lib_stm32.a
    STM32::NoSys
    )
