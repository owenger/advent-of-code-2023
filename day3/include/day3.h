#pragma once

#include <map>
#include <string>
#include <utility>
#include <vector>

namespace day3 {

typedef std::vector<std::string> StringVector;
typedef std::vector<std::vector<bool>> BoolBitmap;
typedef std::vector<std::vector<std::pair<int, int>>> PairBitmap;

StringVector getInput(const std::string &file_path);

int walkVector(const StringVector &input, const BoolBitmap &bitmap);

void walkPairVector(const StringVector &input, PairBitmap &bitmap);

int walkString(const std::string &input_string, const int &row_number,
               const BoolBitmap &bitmap);

void walkPairString(const std::string &input_string, const int &row_number,
                    PairBitmap &bitmap);

BoolBitmap getBitMap(const StringVector &input);

PairBitmap getPairBitMap(const StringVector &input);

bool checkIfPart(const BoolBitmap &bitmap, const int &row_number,
                 const int &start_index, const int &end_index);

void checkIfPairPart(PairBitmap &bitmap, const int &value,
                     const int &row_number, const int &start_index,
                     const int &end_index);

int sumPairBitmap(PairBitmap &bitmap);

void runPartOne(const std::string &input_path);

void runPartTwo(const std::string &input_path);

} // namespace day3
