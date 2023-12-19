#pragma once

#include <map>
#include <set>
#include <string>
#include <utility>
#include <vector>

namespace day7 {

typedef std::vector<std::string> StringVector;

enum HandType { a5 = 6, a4 = 5, fh = 4, a3 = 3, p2 = 2, a2 = 1, hc = 0 };

class Hand {
public:
  Hand(std::string hand, int bidding, bool part2 = false);

  void evaluateValue();

  bool isHigherThan(const Hand &other_hand) const;

  std::string hand_;
  HandType hand_type_;
  int bidding_;
  bool part2_;
};

int getCharValue(char input);

int getCharValue2(char input);

StringVector getInput(const std::string &file_path);

void parseInput(const StringVector &input, StringVector &hands,
                std::vector<int> &biddings);

void sortHands(const StringVector &hand_strs, const std::vector<int> &biddings,
               std::vector<Hand> &hands, bool part2 = false);

int scoreHands(const std::vector<Hand> &hands);

void runPartOne(const std::string &input_path);

void runPartTwo(const std::string &input_path);

} // namespace day7
