#include <iostream>
#include <unordered_map>
#include <string>
#include <vector>

using namespace std;

class Solution {
public:
    bool wordPattern(string pattern, string s) {
        unordered_map<char, string> c_to_s;
        unordered_map<string, char> s_to_c;
        vector<string> words;
        string current_word;
        for(int i = 0; i < s.size(); i++) {
            if (s[i] == ' ') {
                words.push_back(current_word);
                current_word.clear();
            } else {
                current_word.push_back(s[i]);
            }
        }
        words.push_back(current_word);

        if (pattern.size() != words.size())
            return false;

        for(int i = 0; i < pattern.size(); i++) {
            const auto c_to_s_res = c_to_s.find(pattern[i]);
            const auto s_to_c_res = s_to_c.find(words[i]);

            if (c_to_s_res == c_to_s.end()) {
                if (s_to_c_res == s_to_c.end()) {
                    s_to_c[words[i]] = pattern[i];
                    c_to_s[pattern[i]] = words[i];
                } else return false;
            } else if (!(c_to_s_res->second == words[i] && s_to_c_res->second == pattern[i]))
                return false;
        }

        return true;
    }
};

int main() {
    string pattern = "abba";
    string words = "dog cat cat dog";
    cout << Solution().wordPattern(pattern, words) << "\n";
    return 0;
}
