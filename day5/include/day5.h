#pragma once

#include <map>
#include <set>
#include <string>
#include <utility>
#include <vector>

namespace day5 {

typedef std::vector<std::string> StringVector;

class RecipeMap {
public:
  void addMap(std::vector<int64_t> map);

  void clear();

  int getLength();

  int64_t map(const int64_t &input) const;

  std::vector<std::vector<int64_t>> map_vec_;
};

StringVector getInput(const std::string &file_path);

void parseInput(const StringVector &input, std::vector<int64_t> &seeds,
                std::vector<RecipeMap> &recipe_maps);

void getSeeds(const std::string &line, std::vector<int64_t> &seeds);

void getRecipeMaps(const StringVector &input,
                   std::vector<RecipeMap> &recipe_maps);

int64_t runSeeds(const std::vector<int64_t> &seeds,
                 const std::vector<RecipeMap> &recipe_maps);

void runPartOne(const std::string &input_path);

void runPartTwo(const std::string &input_path);

} // namespace day5
