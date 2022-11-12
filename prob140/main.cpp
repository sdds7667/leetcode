#include <iostream>
#include <vector>

using namespace std;

struct Trie {
    Trie *m_neighbours[26];
    string m_word;

public:
    Trie() {
        for (int i = 0; i < 26; i++)
            m_neighbours[i] = nullptr;
    }

    Trie(vector<string> &word_dict) {
        for (int i = 0; i < 26; i++)
            m_neighbours[i] = nullptr;
        for (const auto &word: word_dict) {
            Trie *current = this;
            for (const auto letter: word)
                if (current->m_neighbours[letter - 'a'] == nullptr)
                    current = (current->m_neighbours[letter - 'a'] = new Trie());
                else
                    current = current->m_neighbours[letter - 'a'];
            current->m_word = string(word);
        }
    }
};

class Solution {
public:
    vector<string> wordBreak(string s, vector<string> &wordDict) {
        Trie trie {wordDict};

    }

    void backtrack(string s, Trie& trie, vector<string>& res, int i, int j) {
        if (trie.)
    }
};

int main() {
    std::cout << "Hello, World!" << std::endl;
    return 0;
}
