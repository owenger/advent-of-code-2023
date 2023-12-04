#include "day2.h"
#include <fstream>
#include <iostream>
#include <regex>
#include <string>

namespace day2 {

void getColorValues(const std::string &input_string, int &blue, int &red,
                    int &green) {
  std::regex pattern("(\\w+):\\s*(\\d+)");
  std::smatch matches;

  std::string::const_iterator search_start(input_string.cbegin());
  std::cout << "Matches: "
            << "\n";
  while (
      std::regex_search(search_start, input_string.cend(), matches, pattern)) {
    for (int i = 0; i < matches.size(); i++) {
      std::cout << matches[i].str() << "\n";
    }
  }
}

void runPartOne() {}

void runPartTwo() {}

} // namespace day2

int main() {

  // day1::runPartOne();
  // day1::runPartTwo(false);
  return 0;
}
