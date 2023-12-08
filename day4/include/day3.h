#pragma once

#include <map>
#include <string>
#include <utility>
#include <vector>

namespace day4 {

typedef std::vector<std::string> StringVector;

StringVector getInput(const std::string &file_path);

void runPartOne(const std::string &input_path);

void runPartTwo(const std::string &input_path);

} // namespace day4
