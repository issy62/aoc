#include <cctype>
#include <cinttypes>
#include <fstream>
#include <iostream>
#include <string>

std::uint32_t get_cal_value(const std::string &str) {
  std::string f_digit;
  for (const auto &ch : str) {
    if (std::isdigit(ch)) {
      f_digit += ch;
    }
  }

  if (f_digit.empty()) {
    return 1337;
  }

  if (f_digit.size() == 1) {
    return std::stoi(f_digit) * 11;
  }

  f_digit.erase(f_digit.begin() + 1, f_digit.end() - 1);

  std::uint32_t res = std::stoi(f_digit);

  return res;
}

int main() {
  std::ifstream input_file("input.txt");

  if (!input_file.is_open()) {
    std::cerr << "Can't open the file!";
    return 1;
  }

  std::uint32_t res = 0;
  std::string line;
  while (std::getline(input_file, line)) {
    res += get_cal_value(line);
  }

  std::cout << "Puzzle Answer: " << res << "\n";

  return 0;
}
