#include "day8.h"
#include <cstdint>
#include <deque>
#include <fstream>
#include <iostream>
#include <numeric>
#include <regex>
#include <string>

namespace day8 {

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

void parseInput(const StringVector &input, std::vector<bool> &command,
                std::map<std::string, StringPair> &commandMap) {
  for (auto str : input[0]) {
    command.push_back(str == 'L');
  }
  std::regex pattern{"(\\w+) = \\((\\w+), (\\w+)\\)"};
  std::smatch matches;

  for (int i = 2; i < input.size(); i++) {
    if (std::regex_search(input[i], matches, pattern)) {
      commandMap[matches[1].str()] = {matches[2].str(), matches[3].str()};
    }
  }
}

int walkMap(const std::map<std::string, StringPair> &map,
            const std::vector<bool> &command) {
  std::string cur = "AAA";
  int count = 0;
  int max_count = 1000000000;

  while (true) {
    for (const auto &com : command) {
      count++;
      if (com) {
        cur = map.at(cur).first;
      } else {
        cur = map.at(cur).second;
      }
      if (cur == "ZZZ") {
        return count;
      }
      if (count > max_count) {
        return -1;
      }
    }
  }
}

int walkMaps(const std::map<std::string, StringPair> &map,
             const std::vector<bool> &command, const StringVector &starts) {
  StringVector curs = starts;
  uint64_t count = 0;

  std::vector<uint64_t> counts(curs.size(), 0);

  bool doit = true;
  while (doit) {
    for (const auto &com : command) {
      count++;
      // if (count % 1000000 == 0) {
      //   std::cout << "\nCount: " << std::to_string(count);
      // }
      for (int i = 0; i < curs.size(); i++) {
        if (com) {
          curs[i] = map.at(curs[i]).first;
        } else {
          curs[i] = map.at(curs[i]).second;
        }
        if (curs[i][curs[i].size() - 1] == 'Z') {
          counts[i] = count;
        }
      }
      bool all_values = true;
      for (auto &c : counts) {
        if (c == 0) {
          all_values = false;
        }
      }
      if (all_values) {
        doit = false;
        break;
      }
    }
  }
  uint64_t res = 1;
  for (auto &c : counts) {
    std::cout << std::to_string(c) << ", ";
    res = std::lcm(res, c);
  }
  return res;
}

void runPartOne(const std::string &input_path) {
  std::vector<std::string> input = getInput(input_path);
  std::vector<bool> command;
  std::map<std::string, StringPair> command_map;
  parseInput(input, command, command_map);
  int res = walkMap(command_map, command);
  printf("\nResult: %d\n", res);
}

void runPartTwo(const std::string &input_path) {
  std::vector<std::string> input = getInput(input_path);
  std::vector<bool> command;
  std::map<std::string, StringPair> command_map;
  StringVector starts;
  parseInput(input, command, command_map);
  for (auto &key : command_map) {
    if (key.first[key.first.size() - 1] == 'A') {
      starts.push_back(key.first);
    }
  }
  uint64_t res = walkMaps(command_map, command, starts);
  printf("\nResult: %lu\n", res);
}
} // namespace day8
