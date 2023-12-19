#include "day5.h"
#include <deque>
#include <fstream>
#include <iomanip>
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
    if (input >= map[1] && input < map[1] + map[2]) {
      return map[0] + input - map[1];
    }
  }
  return input;
}

int64_t RecipeMap::reverseMap(const int64_t &input) const {
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
    if (started) {
      std::smatch matches;
      if (std::regex_search(input[i], matches, digits)) {
        current_map.addMap({std::stoll(matches[1].str()),
                            std::stoll(matches[2].str()),
                            std::stoll(matches[3].str())});
      }
    }
    if (input[i].find(map_name) != std::string::npos || i == input.size() - 1) {
      if (started) {
        recipe_maps.push_back(current_map);
        current_map.clear();
        continue;
      } else {
        started = true;
      }
    }
  }
}

int64_t runSeeds(const std::vector<int64_t> &seeds,
                 const std::vector<RecipeMap> &recipe_maps, bool verbose) {
  int64_t min = -1;
  int64_t count = 0;
  for (auto &seed : seeds) {
    if (verbose) {
      std::cout << "\nSeed trail: Seed: " << std::to_string(seed);
    }
    int64_t res = seed;
    for (auto &map : recipe_maps) {
      res = map.map(res);
      if (verbose) {
        std::cout << " Next: " << std::to_string(res);
      }
    }
    if (min == -1 || res < min) {
      min = res;
    }
    count++;
    if (count % 1000000 == 0) {
      double percentage = (double)count / seeds.size() * 100;
      std::cout << std::fixed << std::setprecision(2);
      std::cout << "\nProgress: " << percentage << "%\n";
    }
  }
  return min;
}

int64_t runReversedSeed(const std::vector<int64_t> &seeds,
                        const std::vector<RecipeMap> &recipe_maps,
                        bool verbose) {
  int64_t max_seed = 0;
  for (int i = 0; i < seeds.size() - 1; i += 2) {
    for (int j = 0; j < seeds[i + 1]; j++) {
      if (seeds[i] + j > max_seed) {
        max_seed = seeds[i] + j;
      }
    }
  }

  for (int64_t res = 0; res < max_seed; res++) {
    int64_t seed = res;
    if (verbose) {
      std::cout << "\nSeed trail: Seed: " << std::to_string(res);
    }
    for (int i = recipe_maps.size() - 1; i >= 0; i--) {
      seed = recipe_maps[i].reverseMap(seed);
      if (verbose) {
        std::cout << " Next: " << std::to_string(seed);
      }
    }
    for (int i = 0; i < seeds.size() - 1; i += 2) {
      if (seed >= seeds[i] && seed < seeds[i] + seeds[i + 1]) {
        return res;
      }
    }

    if (res % 1000000 == 0) {
      std::cout << "\nProgress: " << res << "\n";
    }
  }
  return 0;
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
  std::vector<int64_t> seeds_ranges;
  std::vector<int64_t> seeds;
  std::vector<RecipeMap> recipe_maps;
  parseInput(input, seeds_ranges, recipe_maps);
  int64_t result = runReversedSeed(seeds_ranges, recipe_maps, false);

  /**
  for (int64_t i = 0; i < seeds_ranges.size() - 1; i += 2) {
    for (int64_t j = 0; j < seeds_ranges[i + 1]; j++) {
      seeds.push_back(seeds_ranges[i] + j);
    }
  }
  int64_t result = runSeeds(seeds, recipe_maps);
  **/
  std::cout << "\nResult: " << std::to_string(result) << "\n";
}
} // namespace day5
