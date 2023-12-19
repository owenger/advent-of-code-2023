#pragma once

#include <map>
#include <set>
#include <string>
#include <utility>
#include <vector>

namespace day4 {

typedef std::vector<std::string> StringVector;

StringVector getInput(const std::string &file_path);

void parseInputString(const std::string &input_string,
                      std::vector<int> &ticket_numbers,
                      std::set<int> &winning_numbers);

int getScore(const std::vector<int> &ticket_numbers,
             const std::set<int> &winning_numbers);

int walkBackVector(const StringVector &input);

void runPartOne(const std::string &input_path);

void runPartTwo(const std::string &input_path);

} // namespace day4
