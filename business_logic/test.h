typedef struct UserInput {
    int left_rotary_val;
    int right_rotary_val;
    int left_button_pressed;
    int right_button_pressed;
} UserInput;

void user_input(UserInput input);
void get_matrix_cmd_word(char* buf);
