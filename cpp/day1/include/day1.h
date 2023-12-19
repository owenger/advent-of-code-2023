#include <vector>

namespace day1 {

void runPartOne();

void runPartTwo(bool debug);

std::vector<std::string> getInput(const std::string &file_path);

bool getIntIndexes(const std::string &input_string, int &left_value, int &left_index, int &right_value, int &right_index);

bool getStringIndexes(const std::string &input_string, int &left_value, int &left_index, int &right_value, int &right_index);

int getValueFromInt(const std::string &input_string);

int getValueFromString(const std::string &input_string);

std::vector<std::string> DIGITS = {"zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"};

std::vector<std::string> TEST_SET = {"two1nine", "eightwothree", "abcone2threexyz", "xtwone3four", "4nineeightseven2", "zoneight234", "7pqrstsixteen"};


} //namespace day1
