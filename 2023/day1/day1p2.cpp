#include <algorithm>
#include <cinttypes>
#include <fstream>
#include <iostream>
#include <map>
#include <string>

std::uint32_t get_val(const std::string &str) {
  const std::map<std::string, std::string> dict{
      {"one", "1"},   {"two", "2"},   {"three", "3"},
      {"four", "4"},  {"five", "5"},  {"six", "6"},
      {"seven", "7"}, {"eight", "8"}, {"nine", "9"}};

  std::string calc_digit;

  for (std::size_t i = 0; i < str.size(); ++i) {
    if (std::isdigit(str.at(i))) {
      calc_digit += str.at(i);
    } else {
      for (const auto &num : dict) {
        const auto it = str.find(num.first, i);
        if (it == i) {
          calc_digit += num.second;
          i += num.first.length() - 2;
        }
      }
    }
  }

  std::clog << "Original Input: " << str << "\n";
  std::clog << "Clensed Input: " << calc_digit << "\n";

  if (calc_digit.size() == 1) {
    const std::uint32_t res = std::stoi(calc_digit) * 11;
    std::clog << "Single digit handled: " << res << "\n";
    return res;
  }

  if (calc_digit.empty()) {
    std::cerr << "Empty string!\n";
    return 1337;
  }

  // Erase anything bwteen the first and last digit
  calc_digit.erase(calc_digit.begin() + 1, calc_digit.end() - 1);

  std::clog << "Sliced Double Digit: " << calc_digit << "\n";

  const std::uint32_t res = static_cast<std::uint32_t>(std::stoi(calc_digit));

  std::clog << "Result: " << res << "\n";
  return res;
}

int main(int argc, char *argv[]) {
  std::ifstream input_file("input.txt");

  if (!input_file.is_open()) {
    std::cerr << "Can't open the file!";
    return 1;
  }

  std::uint32_t res = 0;
  std::string line;

  while (std::getline(input_file, line)) {
    res += get_val(line);
  }

  std::cout << "Puzzle Answer: " << res << "\n";

  return 0;
}
