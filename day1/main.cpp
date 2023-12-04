#include <iostream>
#include <fstream>
#include <string>
#include "include/day1.h"

namespace day1 {

std::vector<std::string> getInput(const std::string &file_path) {
  std::fstream input_file;
  input_file.open(file_path, std::ios::in);
  std::vector<std::string> input_vector;
  if (input_file.is_open()) {
    std::string input;
    while(getline(input_file, input)) {
      input_vector.push_back(input);
    }
  }
  return input_vector;
}

int getValueFromInt(const std::string &input_string) {
  int left_value = -1;
  int right_value = -1;
  int half_string_size = static_cast<int>(input_string.size() / 2);
  for(auto& character: input_string) {
    if(isdigit(character)) {
      right_value = character - '0';
      if (left_value == -1) {
        left_value = character - '0';
      }
    }
  }
  
  int total_value = 10 * left_value + right_value;
  std::cout << "\n Input string: " << input_string << "\n right value: " << std::to_string(right_value) << "; left value: " << std::to_string(left_value) << "; total value: " << std::to_string(total_value) << "; \n";
  return total_value;

}

bool getIntIndexes(const std::string &input_string, int &left_value, int &left_index, int &right_value, int &right_index) {
  bool has_occurrence{false};
  left_value = -1;
  left_index = -1;
  right_value = -1;
  right_index = -1;
  for(int i=0; i<input_string.size(); i++) {
    if(isdigit(input_string[i])) {
      has_occurrence = true;
      right_value = input_string[i] - '0';
      right_index = i;
      if (left_value == -1) {
        left_value = input_string[i] - '0';
        left_index = i;
      }
    }
  }
  return has_occurrence;
}

bool getStringIndexes(const std::string &input_string, int &left_value, int &left_index, int &right_value, int &right_index) {
  bool has_occurrence{false};
  left_value = -1;
  left_index = -1;
  right_value = -1;
  right_index = -1;

  int cur_first_index, cur_last_index;
  for(int i=0; i<DIGITS.size(); i++) {
    cur_first_index = input_string.find(DIGITS[i]);
    cur_last_index = input_string.rfind(DIGITS[i]);

    if (cur_first_index != std::string::npos && (cur_first_index < left_index || left_index == -1)) {
      has_occurrence = true;
      left_index = cur_first_index;
      left_value = i;
    }

    if (cur_last_index != std::string::npos && (cur_last_index > right_index || right_index == -1)) {
      has_occurrence = true;
      right_index = cur_last_index;
      right_value = i;
    }
  }

  //std::cout << "String: " << input_string << "; Left index: " << left_index << "; Right index: " << right_index << "; Left value: " << left_value << "; Right value: " << right_value << "\n";

  return has_occurrence;
}

void runPartOne() {
  std::string input_path = "/home/oli/Documents/advent-of-code-2023/day1/input_1.txt";
  std::vector<std::string> input_vector = getInput(input_path);
  
  int sum = 0;

  for (auto& input_string: input_vector) {
    sum += getValueFromInt(input_string);
  }
  std::cout << "Result value: " << std::to_string(sum) << "\n";

}

void runPartTwo(bool debug) {
  std::vector<std::string> input_vector;
  if (debug) {
    input_vector = TEST_SET;
  } else {
  std::string input_path = "/home/oli/Documents/advent-of-code-2023/day1/input_1.txt";
  input_vector = getInput(input_path);
  }
  
  int sum = 0;

  for (auto &input_string: input_vector) {
    int left_value, right_value, left_value_int, left_index_int, right_value_int, right_index_int, left_value_str, left_index_str, right_value_str, right_index_str = -1;
    bool has_int = getIntIndexes(input_string, left_value_int, left_index_int, right_value_int, right_index_int);
    bool has_str = getStringIndexes(input_string, left_value_str, left_index_str, right_value_str, right_index_str);

    if (has_int) {
      left_value = left_value_int;
      right_value = right_value_int;
    }

    if (has_str) {
      if (left_index_str < left_index_int || left_index_int == -1) {
        left_value = left_value_str;
      }
      if (right_index_str > right_index_int || right_index_int == -1) {
        right_value = right_value_str;
      }
    }


    sum += left_value * 10 + right_value;

    std::cout << "String: " << input_string << " ; Left value int: " << std::to_string(left_value_int) << " ; Left value str: " << std::to_string(left_value_str) << "; Right value int: " << std::to_string(right_value_int) << "; Right value str: " << std::to_string(right_value_str) << "\n";
    std::cout << "String: " << input_string << " ; Left index int: " << std::to_string(left_index_int) << " ; Left index str: " << std::to_string(left_index_str) << "; Right index int: " << std::to_string(right_index_int) << "; Right index str: " << std::to_string(right_index_str) << "\n";

    std::cout << "Sum: " << std::to_string(left_value * 10 + right_value) << "\n";
   
  }
  std::cout << "Result value: " << std::to_string(sum) << "\n";

}

} // namespace day1

int main() {

  //day1::runPartOne();
  day1::runPartTwo(false);
  return 0;
}
