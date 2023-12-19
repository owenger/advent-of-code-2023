#include "day10.h"
#include <deque>
#include <fstream>
#include <iostream>
#include <math.h>
#include <regex>
#include <string>

namespace day10 {

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

void parseInput(const StringVector &input, DirMapVecVec &map_array,
                std::vector<StatusVector> &status_array, int &start_x,
                int &start_y) {
  printf("\nInput size: %ld / %ld\n", input[0].size(), input.size());
  for (int y = 0; y < input.size(); y++) {
    std::vector<DirectionMap> dir_map;
    for (int x = 0; x < input[y].size(); x++) {
      if (input[y][x] == 'S') {
        start_x = x;
        start_y = y;
        printf("\nx_start: %d, y_start: %d\n", x, y);
      }
      std::cout << "\nX: " << std::to_string(x) << " Y: " << std::to_string(y)
                << " Char: " << input[y][x] << std::endl;
      dir_map.push_back(meta_map[input[y][x]]);
    }
    map_array.push_back(dir_map);
  }
  for (auto &map : map_array) {
    StatusVector inner_vec(map.size(), Status::UNKNOWN);
    status_array.push_back(inner_vec);
  }
}

void getMovement(const Direction &direction, int &x, int &y) {
  switch (direction) {
  case UP:
    y += 1;
    break;
  case DOWN:
    y -= 1;
    break;
  case LEFT:
    x += 1;
    break;
  case RIGHT:
    x -= 1;
    break;
  default:
    break;
  }
}

void walkPipes(DirMapVecVec &map_array, int &count, int x, int y,
               Direction dir) {
  count++;
  try {
    Direction new_dir = map_array[y][x].at(dir);
    getMovement(new_dir, x, y);
    printf("\nGoing from %d to %d; To %d/%d\n", dir, new_dir, x, y);
    walkPipes(map_array, count, x, y, new_dir);

  } catch (const std::out_of_range &e) {
    std::cout << "Exception: " << e.what() << std::endl;
    count--;
  }
}

void walkPipes2(DirMapVecVec &map_array,
                std::vector<StatusVector> &status_array, int &count, int x,
                int y, Direction dir) {
  count++;
  try {
    Direction new_dir = map_array[y][x].at(dir);
    status_array[y][x] = Status::LOOP;
    printf("\n x: %d, y: %d now loop", x, y);
    getMovement(new_dir, x, y);
    // printf("\nGoing from %d to %d; To %d/%d\n", dir, new_dir, x, y);
    walkPipes2(map_array, status_array, count, x, y, new_dir);
  } catch (const std::out_of_range &e) {
    std::cout << "Exception: " << e.what() << std::endl;
    count--;
  }
}

void walkStatus(std::vector<StatusVector> &status_array, int x, int y) {
  if (y < 0 || y >= status_array.size() || x < 0 ||
      x >= status_array[y].size()) {
    return;
  }
  if (status_array[y][x] == Status::LOOP ||
      status_array[y][x] == Status::CONNECTED) {
    return;
  }
  status_array[y][x] = Status::CONNECTED;

  walkStatus(status_array, x + 1, y);
  walkStatus(status_array, x - 1, y);
  walkStatus(status_array, x, y + 1);
  walkStatus(status_array, x, y - 1);
}

bool checkIfSeed(const std::vector<StatusVector> &status_array, int x, int y) {}

void runPartOne(const std::string &input_path) {
  std::vector<std::string> input = getInput(input_path);
  DirMapVecVec map_array;
  int start_x, start_y;
  std::vector<StatusVector> status_array;
  parseInput(input, map_array, status_array, start_x, start_y);
  int height = map_array.size();
  int width = map_array[0].size();
  int count = 0;
  if (start_x > 0)
    walkPipes(map_array, count, start_x - 1, start_y, Direction::RIGHT);
  if (count > 0) {
    printf("\nResult: %d\n", (count + 1) / 2);
    return;
  }
  if (start_x < width - 1)
    walkPipes(map_array, count, start_x + 1, start_y, Direction::LEFT);
  if (count > 0) {
    printf("\nResult: %d\n", (count + 1) / 2);
    return;
  }
  if (start_y < height - 1)
    walkPipes(map_array, count, start_x, start_y + 1, Direction::UP);
  if (count > 0) {
    printf("\nResult: %d\n", (count + 1) / 2);
    return;
  }
  if (start_y > 0)
    walkPipes(map_array, count, start_x, start_y - 1, Direction::DOWN);
  if (count > 0) {
    printf("\nResult: %d\n", (count + 1) / 2);
    return;
  }
}

void runPartTwo(const std::string &input_path) {
  std::vector<std::string> input = getInput(input_path);
  DirMapVecVec map_array;
  int start_x, start_y;
  std::vector<StatusVector> status_array;
  parseInput(input, map_array, status_array, start_x, start_y);
  int height = map_array.size();
  int width = map_array[0].size();
  int count = 0;
  if (start_x > 0)
    walkPipes2(map_array, status_array, count, start_x - 1, start_y,
               Direction::RIGHT);
  if (start_x < width - 1)
    walkPipes2(map_array, status_array, count, start_x + 1, start_y,
               Direction::LEFT);
  if (start_y < height - 1)
    walkPipes2(map_array, status_array, count, start_x, start_y + 1,
               Direction::UP);
  if (start_y > 0)
    walkPipes2(map_array, status_array, count, start_x, start_y - 1,
               Direction::DOWN);
  status_array[start_y][start_x] = Status::LOOP;

  for (int y = 0; y < status_array.size(); y++) {
    walkStatus(status_array, 0, y);
    walkStatus(status_array, status_array[y].size() - 1, y);
  }
  for (int x = 0; x < status_array[0].size(); x++) {
    walkStatus(status_array, x, 0);
    walkStatus(status_array, x, status_array.size() - 1);
  }
  int y_sum = 0, x_sum = 0;

  int unknown_count = 0;
  for (int i = 0; i < status_array.size(); i++) {
    for (int j = 0; j < status_array[i].size(); j++) {
      if (status_array[i][j] == Status::UNKNOWN) {
        printf("\nx: %d, y: %d, status: %d", j, i, status_array[i][j]);
        unknown_count++;
      }
    }
  }
  printf("\nResult: %d\n", unknown_count);
}
} // namespace day10
