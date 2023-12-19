#pragma once

#include <map>
#include <set>
#include <string>
#include <utility>
#include <vector>

namespace day9 {

typedef std::vector<std::string> StringVector;
typedef std::vector<std::vector<int64_t>> IntVectorVector;

StringVector getInput(const std::string &file_path);

void parseInput(const StringVector &input, IntVectorVector &histories);

int64_t walkHistory(const std::vector<int64_t> &history, bool part2 = false);

void runPartOne(const std::string &input_path);

void runPartTwo(const std::string &input_path);

} // namespace day9
