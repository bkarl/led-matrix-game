#include "main.h"
#include "hardware.h"
#include "stm32l4xx_hal.h"
#include "test.h"

int main(void)
{ 
  uint8_t cmd_word[3*67] = {0};
  UserInput ui = {0};
  init_hardware();

  user_input(ui);
  get_matrix_cmd_word(cmd_word);
  transmit_matrix_data(cmd_word);

  while(1) 
  {
    if (is_user_input_available(&ui)) {
      user_input(ui);
      get_matrix_cmd_word(cmd_word);
      transmit_matrix_data(cmd_word);
    }
  }
  return 0;
}