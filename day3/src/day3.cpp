#include "day3.h"
#include <algorithm>
#include <fstream>
#include <iostream>
#include <map>
#include <math.h>
#include <regex>
#include <stdio.h>
#include <string>

namespace day3 {

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

BoolBitmap getBitMap(const std::vector<std::string> &input) {
  BoolBitmap bitmap;

  for (auto &line : input) {
    std::vector<bool> bitMapPart;
    for (auto &character : line) {
      if (character != '.' && !std::isdigit(character)) {
        bitMapPart.push_back(true);
      } else {
        bitMapPart.push_back(false);
      }
    }
    bitmap.push_back(bitMapPart);
  }
  return bitmap;
}

PairBitmap getPairBitMap(const StringVector &input) {
  PairBitmap bitmap;

  for (auto &line : input) {
    std::vector<std::pair<int, int>> bitMapPart;
    for (auto &character : line) {
      if (character == '*') {
        bitMapPart.push_back(std::pair<int, int>(1, 1));
      } else {
        bitMapPart.push_back(std::pair<int, int>(0, 0));
      }
    }
    bitmap.push_back(bitMapPart);
  }
  return bitmap;
}

int walkVector(const StringVector &input, const BoolBitmap &bitmap) {
  int sum = 0;
  for (int i = 0; i < input.size(); i++) {
    sum += walkString(input[i], i, bitmap);
  }
  return sum;
}

void walkPairVector(const StringVector &input, PairBitmap &bitmap) {
  for (int i = 0; i < input.size(); i++) {
    walkPairString(input[i], i, bitmap);
  }
}

void walkPairString(const std::string &input_string, const int &row_number,
                    PairBitmap &bitmap) {
  int sum = 0;
  int power = 0;
  bool number_mode = false;
  int start_index, end_index;

  for (int i = input_string.size() - 1; i >= 0; i--) {
    if (number_mode) {
      if (std::isdigit(input_string[i])) {
        int addition = (input_string[i] - '0') * pow(10, power);
        sum += addition;
        power++;
      }
      if (!std::isdigit(input_string[i]) || i == 0) {
        start_index = i + 1;
        checkIfPairPart(bitmap, sum, row_number, start_index, end_index);
        number_mode = false;
        power = 0;
        sum = 0;
      }
    } else {
      if (std::isdigit(input_string[i])) {
        number_mode = true;
        end_index = i;
        int addition = (input_string[i] - '0') * pow(10, power);
        sum += addition;
        power++;
      }
    }
  }
}

int walkString(const std::string &input_string, const int &row_number,
               const BoolBitmap &bitmap) {
  int part_sum = 0;

  int sum = 0;
  int power = 0;
  bool number_mode = false;
  int start_index, end_index;

  for (int i = input_string.size() - 1; i >= 0; i--) {
    if (number_mode) {
      if (std::isdigit(input_string[i])) {
        int addition = (input_string[i] - '0') * pow(10, power);
        sum += addition;
        power++;
      }
      if (!std::isdigit(input_string[i]) || i == 0) {
        start_index = i + 1;
        if (checkIfPart(bitmap, row_number, start_index, end_index)) {
          printf("\nAdding %d.", sum);
          part_sum += sum;
        } else {
          printf("\nNot adding %d.", sum);
        }
        number_mode = false;
        power = 0;
        sum = 0;
      }
    } else {
      if (std::isdigit(input_string[i])) {
        number_mode = true;
        end_index = i;
        int addition = (input_string[i] - '0') * pow(10, power);
        sum += addition;
        power++;
      }
    }
  }

  return part_sum;
}

bool checkIfPart(const BoolBitmap &bitmap, const int &row_number,
                 const int &start_index, const int &end_index) {
  int actual_start = std::max(0, start_index - 1);
  int actual_end = std::min(static_cast<int>(bitmap[0].size()), end_index + 1);
  std::vector<int> rows = {row_number};
  if (row_number > 0) {
    rows.push_back(row_number - 1);
  }
  if (row_number + 1 < static_cast<int>(bitmap.size())) {
    rows.push_back(row_number + 1);
  }

  for (int i = actual_start; i <= actual_end; i++) {
    for (auto &row : rows) {
      if (bitmap[row][i]) {
        return true;
      }
    }
  }
  return false;
}

void checkIfPairPart(PairBitmap &bitmap, const int &value,
                     const int &row_number, const int &start_index,
                     const int &end_index) {
  int actual_start = std::max(0, start_index - 1);
  int actual_end = std::min(static_cast<int>(bitmap[0].size()), end_index + 1);
  std::vector<int> rows = {row_number};
  if (row_number > 0) {
    rows.push_back(row_number - 1);
  }
  if (row_number + 1 < static_cast<int>(bitmap.size())) {
    rows.push_back(row_number + 1);
  }

  for (int i = actual_start; i <= actual_end; i++) {
    for (auto &row : rows) {
      if (bitmap[row][i].first == 1 || bitmap[row][i].first == 2) {
        bitmap[row][i].first += 1;
        bitmap[row][i].second *= value;

        if (bitmap[row][i].first == 3) {
          printf("\nRow: %d, Col: %d, Value %d", row, i, bitmap[row][i].second);
        }
      }
    }
  }
}

int sumPairBitmap(PairBitmap &bitmap) {
  int sum = 0;

  for (int i = 0; i < bitmap.size(); i++) {
    for (int j = 0; j < bitmap[i].size(); j++) {
      if (bitmap[i][j].first == 3) {
        sum += bitmap[i][j].second;
      }
    }
  }

  return sum;
}

void runPartOne(const std::string &input_path) {
  StringVector input = getInput(input_path);
  BoolBitmap bitmap = getBitMap(input);
  int result = walkVector(input, bitmap);
  std::cout << "\nResult: " << result << "\n";
}

void runPartTwo(const std::string &input_path) {
  StringVector input = getInput(input_path);
  PairBitmap bitmap = getPairBitMap(input);
  walkPairVector(input, bitmap);
  int sum = sumPairBitmap(bitmap);
  printf("\nResult: %d", sum);
}

} // namespace day3
