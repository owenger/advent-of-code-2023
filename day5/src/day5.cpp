#include "day5.h"
#include <deque>
#include <fstream>
#include <iostream>
#include <math.h>
#include <regex>
#include <string>

namespace day5 {

void RecipeMap::addMap(std::vector<int64_t> map) {
  if (map.size() != 3) {
    std::cout << "ERROR: Can only add vectors of length 3.";
    return;
  }
  map_vec_.push_back(map);
}

void RecipeMap::clear() { map_vec_.clear(); }

int RecipeMap::getLength() { return map_vec_.size(); }

int64_t RecipeMap::map(const int64_t &input) const {
  for (auto &map : map_vec_) {
    if (input >= map[0] && input < map[0] + map[2]) {
      return map[1] + input - map[0];
    }
  }
  return input;
}

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

void parseInput(const StringVector &input, std::vector<int64_t> &seeds,
                std::vector<RecipeMap> &recipe_maps) {
  getSeeds(input[0], seeds);
  getRecipeMaps(input, recipe_maps);
}

void getSeeds(const std::string &line, std::vector<int64_t> &seeds) {
  seeds.clear();
  std::regex pattern{"(\\d+)"};
  std::smatch matches;
  auto start = line.cbegin();
  while (std::regex_search(start, line.cend(), matches, pattern)) {
    try {
      seeds.push_back(std::stoll(matches[1].str()));
    } catch (const std::out_of_range &e) {
      std::cerr << "Error: Input string is out of range for integer "
                   "conversion. String: <"
                << matches[1].str() << ">" << std::endl;
      // Handle the out-of-range case here, if needed.
    }
    start = matches.suffix().first;
  }
}

void getRecipeMaps(const StringVector &input,
                   std::vector<RecipeMap> &recipe_maps) {
  bool started = false;
  RecipeMap current_map;
  std::string map_name = "-to-";
  std::regex digits{"(\\d+)\\s+(\\d+)\\s+(\\d+)"};
  for (int i = 0; i < input.size(); i++) {
    if (input[i].find(map_name) != std::string::npos && started) {
      recipe_maps.push_back(current_map);
      current_map.clear();
      continue;
    } else if (!started) {
      started = true;
    }
    std::smatch matches;
    if (std::regex_search(input[i], matches, digits)) {
      current_map.addMap({std::stoll(matches[1].str()),
                          std::stoll(matches[2].str()),
                          std::stoll(matches[3].str())});
    }
  }
}

int64_t runSeeds(const std::vector<int64_t> &seeds,
                 const std::vector<RecipeMap> &recipe_maps) {
  int64_t min = -1;
  for (auto &seed : seeds) {
    int64_t res = seed;
    for (auto &map : recipe_maps) {
      res = map.map(res);
    }
    if (min == -1 || res < min) {
      min = res;
    }
  }
  return min;
}

void runPartOne(const std::string &input_path) {
  std::vector<std::string> input = getInput(input_path);
  std::vector<int64_t> seeds;
  std::vector<RecipeMap> recipe_maps;
  parseInput(input, seeds, recipe_maps);
  int64_t result = runSeeds(seeds, recipe_maps);
  std::cout << "\nResult: " << std::to_string(result) << "\n";
}

void runPartTwo(const std::string &input_path) {
  std::vector<std::string> input = getInput(input_path);
}
} // namespace day5
