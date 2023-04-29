#pragma once

#include "stm32l4xx_hal.h"
#include "stdbool.h"
#include "test.h"

#define COLORDUINO_ADDRESS 5
#define START_OF_DATA 0x10       //data markers
#define END_OF_DATA 0x20         //data markers
#define CMD_WORD_SIZE 67

extern TIM_HandleTypeDef htim1;
extern TIM_HandleTypeDef htim2;
extern TIM_HandleTypeDef htim6;
extern I2C_HandleTypeDef hi2c1;

void init_hardware();
void setup_led();
void setup_encoder();
void setup_i2c();
void TIM6_Init();
void setup_buttons();
void SystemClock_Config(void);
void Error_Handler(void);
bool is_user_input_available(UserInput *ui);
void transmit_matrix_data(uint8_t cmd_word[3*67]);