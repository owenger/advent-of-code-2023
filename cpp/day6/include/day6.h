#pragma once

#include <map>
#include <set>
#include <string>
#include <utility>
#include <vector>

namespace day6 {

typedef std::vector<std::string> StringVector;

StringVector getInput(const std::string &file_path);

std::vector<int64_t> getInts(const std::string &input);

int numberOfBeats(int64_t time, int64_t distance);

void runPartOne(const std::string &input_path);

void runPartTwo(const std::string &input_path);

} // namespace day6
