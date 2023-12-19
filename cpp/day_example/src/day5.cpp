#include "day5.h"
#include <deque>
#include <fstream>
#include <iostream>
#include <math.h>
#include <regex>
#include <string>

namespace day5 {

StringVector getInput(const std::string &file_path) {
  std::fstream input_file;
  input_file.open(file_path, std::ios::in);
  std::vector<std::string> input_vector;
  if (input_file.is_open()) {
    std::string input;
    while (getline(input_file, input)) {
      input_vector.push_back(input);
    }
  }
  return input_vector;
}

void runPartOne(const std::string &input_path) {
  std::vector<std::string> input = getInput(input_path);
}

void runPartTwo(const std::string &input_path) {
  std::vector<std::string> input = getInput(input_path);
}
} // namespace day5
