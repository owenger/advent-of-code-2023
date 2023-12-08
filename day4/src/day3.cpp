#include "day4.h"
#include <algorithm>
#include <fstream>
#include <iostream>
#include <map>
#include <math.h>
#include <regex>
#include <stdio.h>
#include <string>

namespace day4 {

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
  StringVector input = getInput(input_path);
  BoolBitmap bitmap = getBitMap(input);
  int result = walkVector(input, bitmap);
  std::cout << "\nResult: " << result << "\n";
}

void runPartTwo(const std::string &input_path) {
  StringVector input = getInput(input_path);
  PairBitmap bitmap = getPairBitMap(input);
  walkPairVector(input, bitmap);
  int sum = sumPairBitmap(bitmap);
  printf("\nResult: %d", sum);
}

} // namespace day4
