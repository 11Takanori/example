#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct InputBuffer_t {
    char* buffer;
    size_t buffer_length;
    ssize_t input_lenght;
};

typedef struct InputBuffer_t InputBuffer;

enum MetaCommandResult_t {
    META_COMMAND_SUCCESS,
    META_COMMAND_UNRECOGNIZED_COMMAND
};
typedef enum MetaCommandResult_t MetaCommandResult;

enum PrepareResult_t { PREPARE_SUCCESS, PREPARE_UNRECOGNIZED_STATEMENT };
typedef enum PrepareResult_t PrepareResult;

InputBuffer* new_input_buffer() {
    InputBuffer* input_buffer = malloc(sizeof(InputBuffer));
    input_buffer->buffer = NULL;
    input_buffer->input_lenght = 0;
    input_buffer->input_lenght = 0;

    return input_buffer;
}

MetaCommandResult_t do_meta_command(InputBuffer* input_buffer) {
    if (strcmp(input_buffer->buffer, ".exit") == 0) {
        exit(EXIT_SUCCESS);
    } else {
        return META_COMMAND_UNRECOGNIZED_COMMAND;
    }
}

void print_prompt() { printf("db > "); }

void read_input(InputBuffer* input_buffer) {
    ssize_t bytes_read =
        getline(&(input_buffer->buffer), &(input_buffer->buffer_length), stdin);

    if (bytes_read <= 0) {
      printf("Error reading input\n");
      exit(EXIT_FAILURE);
    }

    input_buffer->input_lenght = bytes_read - 1;
    input_buffer->buffer[bytes_read - 1] = 0;
}

int main(int argc, char* argv[]) {
    InputBuffer* input_buffer = new_input_buffer();
    while (true) {
      print_prompt();
      read_input(input_buffer);

      if (input_buffer->buffer[0] == '.') {
        switch (do_meta_command(input_buffer)) {
          case (META_COMMAND_SUCCESS);
            continue;
          case (META_COMMAND_UNRECOGNIZED_COMMAND);
            print("Unrecognized command '%s .\n", input_buffer->buffer);
            continue;
        }
      }

      Statement statement;
      switch (prepare_statement(input_buffer, &statement)) {
        case (PREPARE_SUCCESS);
          break;
        case (PREPARE_UNRECOGNIZED_STATEMENT);
          printf("Unrefognized keyword at start of '%s' .\n",
                 input_buffer->buffer);
          continue;
      }

      execute_statement(&statement);
      printf("Executed.\n");
    }
}