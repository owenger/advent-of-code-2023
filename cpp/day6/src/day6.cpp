#include "day6.h"
#include <deque>
#include <fstream>
#include <iostream>
#include <math.h>
#include <regex>
#include <string>

namespace day6 {

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

std::vector<int64_t> getInts(const std::string &input) {
  std::vector<int64_t> ints;
  std::regex pattern{"\\s*(\\d+)\\s*(\\d+)\\s*(\\d+)\\s*(\\d+)\\s*"};
  std::smatch matches;

  if (std::regex_search(input, matches, pattern)) {
    for (int i = 1; i < matches.size(); i++) {
      ints.push_back(std::stoi(matches[i].str()));
    }
  }
  return ints;
}

int numberOfBeats(int64_t time, int64_t distance) {
  int64_t count = 0;
  for (int64_t i = 1; i < time; i++) {
    if (i * (time - i) > distance) {
      count++;
    }
  }
  return count;
}

void runPartOne(const std::string &input_path) {
  std::vector<std::string> input = getInput(input_path);
  std::vector<int64_t> times = getInts(input[0]);
  std::vector<int64_t> distances = getInts(input[1]);
  int64_t total = 1;
  for (int i = 0; i < times.size(); i++) {
    total *= numberOfBeats(times[i], distances[i]);
  }
  std::cout << "\nResult: " << std::to_string(total) << "\n";
}

void runPartTwo(const std::string &input_path) {
  std::vector<std::string> input = getInput(input_path);
  std::vector<int64_t> times = getInts(input[0]);
  std::vector<int64_t> distances = getInts(input[1]);
  std::string time_str = std::to_string(times[0]) + std::to_string(times[1]) +
                         std::to_string(times[2]) + std::to_string(times[3]);
  std::string distance_str =
      std::to_string(distances[0]) + std::to_string(distances[1]) +
      std::to_string(distances[2]) + std::to_string(distances[3]);
  int64_t time = std::stoll(time_str);
  int64_t distance = std::stoll(distance_str);
  int64_t total = numberOfBeats(time, distance);
  printf("\nResult: %ld \n", total);
}
} // namespace day6
