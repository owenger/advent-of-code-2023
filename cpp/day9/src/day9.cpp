#include "day9.h"
#include <deque>
#include <fstream>
#include <iostream>
#include <math.h>
#include <regex>
#include <sstream>
#include <string>

namespace day9 {

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

void parseInput(const StringVector &input, IntVectorVector &histories) {
  for (auto &input_str : input) {
    std::vector<int64_t> history;
    std::istringstream iss(input_str);
    int64_t number;
    while (iss >> number) {
      history.push_back(number);
    }
    histories.push_back(history);
  }
}

int64_t walkHistory(const std::vector<int64_t> &history, bool part2) {
  IntVectorVector walked_history = {history};
  std::vector<int64_t> additions;

  while (true) {
    bool all_zeros = true;
    std::vector<int64_t> next_history;
    if (walked_history.back().size() == 1) {
      walked_history.push_back({0});
      break;
    }
    for (int i = 0; i < walked_history.back().size() - 1; i++) {
      int dif = walked_history.back()[i + 1] - walked_history.back()[i];
      next_history.push_back(dif);
      all_zeros = all_zeros && dif == 0;
    }
    walked_history.push_back(next_history);
    if (all_zeros) {
      break;
    }
  }
  int64_t total = 0;
  if (!part2) {
    for (auto &hist : walked_history) {
      total += hist.back();
    }
  } else {
    for (int i = 0; i < walked_history.size(); i++) {
      int64_t a = walked_history[i][0] * pow(-1, i - 1);
      total += walked_history[i][0] * pow(-1, i);
    }
  }
  return total;
}

void runPartOne(const std::string &input_path) {
  std::vector<std::string> input = getInput(input_path);
  IntVectorVector histories;
  parseInput(input, histories);
  int64_t res = 0;
  for (auto &history : histories) {
    res += walkHistory(history);
  }
  printf("\nResult: %ld\n", res);
}

void runPartTwo(const std::string &input_path) {
  std::vector<std::string> input = getInput(input_path);
  IntVectorVector histories;
  parseInput(input, histories);
  int64_t res = 0;
  for (auto &history : histories) {
    int64_t a = walkHistory(history, true);
    res += a;
  }
  printf("\nResult: %ld\n", res);
}
} // namespace day9
