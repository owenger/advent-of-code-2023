#include "day2.h"
#include <fstream>
#include <iostream>
#include <map>
#include <regex>
#include <string>

namespace day2 {

std::vector<std::string> getInput(const std::string &file_path) {
  std::fstream input_file;
  input_file.open(file_path, std::ios::in);
  std::vector<std::string> input_vector;
  if (input_file.is_open()) {
    std::string input;
    while (getline(input_file, input)) {
      input_vector.push_back(input);
    }
  }
  return input_vector;
}

void getColourValues(const std::string &input_string, int &blue, int &red,
                     int &green) {
  std::string delimiter = "; ";
  int last = 0;
  int next = 0;
  std::vector<std::string> tokens;

  while ((next = input_string.find(delimiter, last)) != std::string::npos) {
    tokens.push_back(input_string.substr(last, next - last));
    last = next + delimiter.size();
  }

  std::regex cregex("(\\d+)\\s+(\\w+)");

  for (auto &token : tokens) {
    std::smatch matches;
    auto start = token.cbegin();
    while (std::regex_search(start, token.cend(), matches, cregex)) {
      std::string match_colour = matches[2].str();
      int match_number = std::stoi(matches[1].str());
      if (match_colour == "blue" && blue < match_number) {
        blue = match_number;
      } else if (match_colour == "red" && red < match_number) {
        red = match_number;
      } else if (match_colour == "green" && green < match_number) {
        green = match_number;
      } else if (match_colour != "blue" && match_colour != "red" &&
                 match_colour != "green") {
        std::cout << "\nError colour not known: " << match_colour << "\n";
      }
      start = matches.suffix().first;
    }
  }
}

int getSimpleColourValues(const std::string &input_string, int &blue, int &red,
                          int &green) {
  std::smatch matches;
  auto start = input_string.cbegin();
  std::regex cregex("(\\d+)\\s+(\\w+)");

  while (std::regex_search(start, input_string.cend(), matches, cregex)) {
    std::string match_colour = matches[2].str();
    int match_number = std::stoi(matches[1].str());
    if (match_colour == "blue" && blue < match_number) {
      blue = match_number;
    } else if (match_colour == "red" && red < match_number) {
      red = match_number;
    } else if (match_colour == "green" && green < match_number) {
      green = match_number;
    } else if (match_colour != "blue" && match_colour != "red" &&
               match_colour != "green") {
      std::cout << "\nError colour not known: " << match_colour << "\n";
    }

    start = matches.suffix().first;
  }

  std::regex game_regex("Game (\\d+):");
  std::smatch game_matches;

  std::regex_search(input_string, game_matches, game_regex);
  return std::stoi(game_matches[1]);
}

void runPartOne(const std::string &input_path) {
  std::vector input = getInput(input_path);
  int total = 0;

  for (int i = 0; i < input.size(); i++) {
    int blue = 0;
    int red = 0;
    int green = 0;
    int match = getSimpleColourValues(input[i], blue, red, green);
    std::cout << "\nMatch: " << match << " Blue: " << blue << " Red: " << red
              << " Green: " << green;
    if (blue <= 14 && red <= 12 && green <= 13) {
      total += match;
    }
  }

  std::cout << "\nResult: " << std::to_string(total) << "\n";
}

void runPartTwo(const std::string &input_path) {
  std::vector input = getInput(input_path);
  int total = 0;

  for (int i = 0; i < input.size(); i++) {
    int blue = 0;
    int red = 0;
    int green = 0;
    int match = getSimpleColourValues(input[i], blue, red, green);
    std::cout << "\nMatch: " << match << " Blue: " << blue << " Red: " << red
              << " Green: " << green;
    total += blue * red * green;
  }

  std::cout << "\nResult: " << std::to_string(total) << "\n";
}

} // namespace day2
