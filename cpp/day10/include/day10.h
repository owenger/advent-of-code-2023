#pragma once

#include <map>
#include <set>
#include <string>
#include <utility>
#include <vector>

namespace day10 {

enum Direction { UP = 0, DOWN = 1, LEFT = 2, RIGHT = 3 };
enum Status { UNKNOWN = 0, CONNECTED = 1, LOOP = 3 };

typedef std::vector<std::string> StringVector;
typedef std::vector<Status> StatusVector;
typedef std::map<Direction, Direction> DirectionMap;
typedef std::vector<std::vector<DirectionMap>> DirMapVecVec;

inline DirectionMap l_map = {{Direction::UP, Direction::UP},
                             {Direction::DOWN, Direction::DOWN}};

inline DirectionMap dash_map = {{Direction::RIGHT, Direction::RIGHT},
                                {Direction::LEFT, Direction::LEFT}};

inline DirectionMap L_map = {{Direction::UP, Direction::LEFT},
                             {Direction::RIGHT, Direction::DOWN}};

inline DirectionMap J_map = {{Direction::UP, Direction::RIGHT},
                             {Direction::LEFT, Direction::DOWN}};

inline DirectionMap seven_map = {{Direction::DOWN, Direction::RIGHT},
                                 {Direction::LEFT, Direction::UP}};

inline DirectionMap f_map = {{Direction::DOWN, Direction::LEFT},
                             {Direction::RIGHT, Direction::UP}};

inline DirectionMap dot_map = {};

inline std::map<char, DirectionMap> meta_map = {
    {'|', l_map},     {'-', dash_map}, {'L', L_map},   {'J', J_map},
    {'7', seven_map}, {'F', f_map},    {'.', dot_map}, {'S', dot_map}};

StringVector getInput(const std::string &file_path);

void parseInput(const StringVector &input, DirMapVecVec &map_array,
                std::vector<StatusVector> &status_array, int &start_x,
                int &start_y);

void getMovement(const Direction &direction, int &x, int &y);

void walkPipes(DirMapVecVec &map_array, int &count, int x, int y,
               Direction dir);

void walkPipes2(DirMapVecVec &map_array,
                std::vector<StatusVector> &status_array, int &count, int x,
                int y, Direction dir);

bool checkIfSeed(const std::vector<StatusVector> &status_array, int x, int y);

void walkStatus(std::vector<StatusVector> &status_array, int x, int y);

void runPartOne(const std::string &input_path);

void runPartTwo(const std::string &input_path);

} // namespace day10
