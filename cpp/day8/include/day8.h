#pragma once

#include <map>
#include <set>
#include <string>
#include <utility>
#include <vector>

namespace day8 {

typedef std::vector<std::string> StringVector;
typedef std::pair<std::string, std::string> StringPair;

StringVector getInput(const std::string &file_path);

void parseInput(const StringVector &input, std::vector<bool> &command,
                std::map<std::string, StringPair> &commandMap);

int walkMap(const std::map<std::string, StringPair> &map,
            const std::vector<bool> &command);

int walkMaps(const std::map<std::string, StringPair> &map,
             const std::vector<bool> &command, const StringVector &starts);

void runPartOne(const std::string &input_path);

void runPartTwo(const std::string &input_path);

} // namespace day8
