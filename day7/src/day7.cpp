#include "day7.h"
#include <deque>
#include <fstream>
#include <iostream>
#include <math.h>
#include <regex>
#include <string>

namespace day7 {

Hand::Hand(std::string hand, int bidding, bool part2)
    : hand_(hand), bidding_(bidding), part2_(part2) {}

void Hand::evaluateValue() {
  std::vector<int> counts(13, 0);
  for (auto &ch : hand_) {
    if (ch == 'J' && part2_) {
      for (auto &count : counts) {
        count += 1;
      }
    }
    counts[getCharValue(ch)] += 1;
  }
  int most = 0;
  int second_most = 0;

  for (auto &count : counts) {
    if (count > most) {
      second_most = most;
      most = count;
      continue;
    }
    if (count > second_most) {
      second_most = count;
    }
  }
  most += getCharValue('J');

  switch (most) {
  case (5):
    hand_type_ = HandType::a5;
    break;
  case (4):
    hand_type_ = HandType::a4;
    break;
  case (3):
    if (second_most == 2) {
      hand_type_ = HandType::fh;
    } else {
      hand_type_ = HandType::a3;
    }
    break;
  case (2):
    if (second_most == 2) {
      hand_type_ = HandType::p2;
    } else {
      hand_type_ = HandType::a2;
    }
    break;
  case (1):
    hand_type_ = HandType::hc;
    break;
  default:
    hand_type_ = HandType::hc;
    break;
  }
}

bool Hand::isHigherThan(const Hand &other_hand) const {
  if (hand_type_ > other_hand.hand_type_) {
    return true;
  } else if (hand_type_ < other_hand.hand_type_) {
    return false;
  }
  for (int i = 0; i < hand_.size(); i++) {
    if (part2_) {
      if (getCharValue2(hand_[i]) == getCharValue2(other_hand.hand_[i])) {
        continue;
      }
      return getCharValue2(hand_[i]) > getCharValue2(other_hand.hand_[i]);
    }
    if (getCharValue(hand_[i]) == getCharValue(other_hand.hand_[i])) {
      continue;
    }
    return getCharValue(hand_[i]) > getCharValue(other_hand.hand_[i]);
  }
  return false;
}

int getCharValue(char input) {
  switch (input) {
  case ('A'):
    return 12;
  case ('K'):
    return 11;
  case ('Q'):
    return 10;
  case ('J'):
    return 9;
  case ('T'):
    return 8;
  case ('9'):
    return 7;
  case ('8'):
    return 6;
  case ('7'):
    return 5;
  case ('6'):
    return 4;
  case ('5'):
    return 3;
  case ('4'):
    return 2;
  case ('3'):
    return 1;
  case ('2'):
    return 0;
  default:
    return 0;
  }
}

int getCharValue2(char input) {
  switch (input) {
  case ('A'):
    return 12;
  case ('K'):
    return 11;
  case ('Q'):
    return 10;
  case ('J'):
    return 0;
  case ('T'):
    return 9;
  case ('9'):
    return 8;
  case ('8'):
    return 7;
  case ('7'):
    return 6;
  case ('6'):
    return 5;
  case ('5'):
    return 4;
  case ('4'):
    return 3;
  case ('3'):
    return 2;
  case ('2'):
    return 1;
  default:
    return 0;
  }
}

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

void parseInput(const StringVector &input, StringVector &hands,
                std::vector<int> &biddings) {
  std::regex pattern{"(\\w+) (\\d+)"};

  for (auto &input_str : input) {
    std::smatch matches;

    if (std::regex_search(input_str, matches, pattern)) {
      hands.push_back(matches[1].str());
      biddings.push_back(std::stoi(matches[2].str()));
    }
  }
}

void sortHands(const StringVector &hand_strs, const std::vector<int> &biddings,
               std::vector<Hand> &hands, bool part2) {
  Hand check{hand_strs[0], biddings[0]};
  check.evaluateValue();
  std::cout << "\nValue: " << std::to_string(check.hand_type_) << "\n";
  hands.push_back(check);

  for (int i = 1; i < hand_strs.size(); i++) {
    Hand h = Hand(hand_strs[i], biddings[i], part2);
    h.evaluateValue();
    for (int j = 0; j <= hands.size(); j++) {
      if (j == hands.size()) {
        hands.push_back(h);
        break;
      }
      if (h.isHigherThan(hands[j])) {
        hands.insert(hands.begin() + j, h);
        break;
      }
    }
  }
}

int scoreHands(const std::vector<Hand> &hands) {
  int score = hands.size();
  int total_score = 0;

  for (auto &hand : hands) {
    total_score += score * hand.bidding_;
    score--;
  }
  return total_score;
}

void runPartOne(const std::string &input_path) {
  std::vector<std::string> input = getInput(input_path);
  StringVector hands_strs;
  std::vector<int> biddings;
  parseInput(input, hands_strs, biddings);
  std::vector<Hand> hands;
  sortHands(hands_strs, biddings, hands);
  int result = scoreHands(hands);
  printf("\nResult: %d\n", result);
}

void runPartTwo(const std::string &input_path) {
  std::vector<std::string> input = getInput(input_path);
  StringVector hands_strs;
  std::vector<int> biddings;
  parseInput(input, hands_strs, biddings);
  std::vector<Hand> hands;
  sortHands(hands_strs, biddings, hands);
  int result = scoreHands(hands);
  printf("\nResult: %d\n", result);
}
} // namespace day7
