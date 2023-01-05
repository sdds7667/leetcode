#include<iostream>
#include <bitset>
#include <vector>
#include <unordered_map>

using namespace std;


class Solution {
public:

    vector<int> number_to_vector(int number) {
        vector<int> result(8);
        for(int i = 7; i >= 0; i--, number >>= 1)
            result[i] = (number & 1);
        return result;
    }


    int get_next(int current) {
        int new_number = 0;
        for(int i = 0; i < 6; i++) {
            new_number <<= 1;
            new_number |= !((current >> (7 - i)) & 1) ^ (1 & (current >> (5 - i)));
        }
        return new_number << 1;
    }
    vector<int> prisonAfterNDays(const vector<int>& cells, int n) {
        unordered_map<int, int> indexes;
        vector<int> numbers;
        int current = 0;
        for(int i: cells) {
            current |= i;
            current <<= 1;
        }
        current >>= 1;
        for(;n > 0; n--) {
            const auto indexes_result = indexes.find(current);
            if (indexes_result == indexes.end()) {
                indexes[current] = numbers.size();
                numbers.push_back(current);
            } else {
                int cycle_start = indexes_result->second;
                if (cycle_start == numbers.size())
                    return number_to_vector(current);
                return number_to_vector(numbers[cycle_start + (n-1) % (numbers.size() - cycle_start)]);
            }
            current = get_next(current);
        }
        return number_to_vector(current);
    }
};

int main() {
    vector<int> data{1,0,0,1,0,0,1,0};
   Solution().prisonAfterNDays(data, 1000000000);
}