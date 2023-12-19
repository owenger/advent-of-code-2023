#pragma once

#include <map>
#include <string>
#include <vector>

namespace day2 {

enum COLOUR { RED, BLUE, GREEN };

std::vector<std::string> getInput(const std::string &file_path);

inline std::string getColour(const COLOUR &color) {
  switch (color) {
  case RED:
    return "red";
  case BLUE:
    return "blue";
  case GREEN:
    return "green";
  default:
    return "Unknown color";
  }
}

void getColourValues(const std::string &input_string, int &blue, int &red,
                     int &green);

int getSimpleColourValues(const std::string &input_string, int &blue, int &red,
                          int &green);

void runPartOne(const std::string &input_path);

void runPartTwo(const std::string &input_path);

} // namespace day2
