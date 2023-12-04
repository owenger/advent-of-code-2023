#pragma once

#include <string>
#include <vector>

namespace day2 {

enum COLOR { RED, BLUE, GREEN };

void getColorValues(const std::string &input_string, int &blue, int &red,
                    int &green);

void runPartOne();

void runPartTwo();

} // namespace day2
