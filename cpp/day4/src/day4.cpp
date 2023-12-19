#include "day4.h"
#include <deque>
#include <fstream>
#include <iostream>
#include <math.h>
#include <regex>
#include <string>

namespace day4 {

StringVector getInput(const std::string &file_path) {
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

void parseInputString(const std::string &input_string,
                      std::vector<int> &ticket_numbers,
                      std::set<int> &winning_numbers) {
  std::regex cregex("(\\d+)|(\\|)");
  std::smatch matches;

  auto start = input_string.cbegin();

  int count = 0;
  bool before = true;
  while (std::regex_search(start, input_string.cend(), matches, cregex)) {
    if (matches[2].str() == "|") {
      before = false;
      count++;
      start = matches.suffix().first;
      continue;
    }
    int number = std::stoi(matches[1].str());
    if (count > 0 && before) {
      ticket_numbers.push_back(number);
    } else if (!before) {
      winning_numbers.insert(number);
    }
    count++;
    start = matches.suffix().first;
  }
}

int getScore(const std::vector<int> &ticket_numbers,
             const std::set<int> &winning_numbers) {
  int count = 0;
  for (auto el : ticket_numbers) {
    if (!winning_numbers.count(el)) {
      continue;
    }
    count++;
  }
  return 1 * pow(2, count - 1);
}

int getNumber(const std::vector<int> &ticket_numbers,
              const std::set<int> &winning_numbers) {
  int count = 0;
  for (auto el : ticket_numbers) {
    if (!winning_numbers.count(el)) {
      continue;
    }
    count++;
  }
  return count;
}

int walkBackVector(const StringVector &input) {
  std::deque<int> number_scratchcards;
  int total = input.size();

  for (int i = input.size() - 1; i >= 0; i--) {
    std::vector<int> ticket_numbers;
    std::set<int> winning_numbers;
    parseInputString(input[i], ticket_numbers, winning_numbers);
    int sub_total = 0;
    int result = getNumber(ticket_numbers, winning_numbers);
    std::cout << "\nSubtotal: " << std::to_string(result);
    sub_total += result;
    for (int j = 0; j < result; j++) {
      std::cout << " + " << std::to_string(number_scratchcards[j]);
      sub_total += number_scratchcards[j];
    }
    number_scratchcards.push_front(sub_total);
    total += sub_total;
  }

  return total;
}

void runPartOne(const std::string &input_path) {
  std::vector<std::string> input = getInput(input_path);

  int result = 0;
  for (auto &input_string : input) {
    std::vector<int> ticket_numbers;
    std::set<int> winning_numbers;
    parseInputString(input_string, ticket_numbers, winning_numbers);
    result += getScore(ticket_numbers, winning_numbers);
  }

  std::cout << "\nResult: " << std::to_string(result) << "\n";
}

void runPartTwo(const std::string &input_path) {
  std::vector<std::string> input = getInput(input_path);
  int total = walkBackVector(input);

  std::cout << "\nResult: " << std::to_string(total) << " \n";
}
} // namespace day4
