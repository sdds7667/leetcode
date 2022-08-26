
#include <bits/stdc++.h>

using namespace std;

class Solution
{
public:
    Solution()
    {
        ios_base::sync_with_stdio(0);
        cin.tie(0);
        cout.tie(0);
    }

    bool wordBreak(string s, vector<string> &wordDict)
    {
        vector<bool> dp(s.length());
        unordered_set<string> dict;
        for (auto i : wordDict) {
            dict.insert(i);
        }
        vector<int> ps{(int) s.length()};
        for (int i = s.length() - 1; i >= 0; i--)
        {
            for (int j = ps.size() - 1; j >= 0; j--) {
                if (dict.find(s.substr(i, ps[j] - i)) != dict.end()) {
                    ps.push_back(i);
                    break;
                }
            }
        }
        return (ps.back() == 0);
    }
};

bool test(string s, vector<string> &dict, int expected)
{
    auto result = (new Solution())->wordBreak(s, dict);
    if (result != expected)
    {
        cout << "Got " << result << " expected " << expected << "\n";
        cout << "Failed test\n";
        cout << "[" << s << "]\n with dict \n";
        for (const auto i : dict)
            cout << i << " ";
        cout << "\n";
        return false;
    }
    return true;
}

struct TestCase
{
    string m_s;
    vector<string> *m_dict;
    bool m_expected;

    TestCase(string s, vector<string> *dict, bool expected) : m_s(s), m_dict(dict), m_expected(expected) {}
};

int main()
{
    cout << "Running"
         << "\n";
    vector<TestCase> tests{
        TestCase("leetcode", new vector<string>{"leet","code"}, true),
        TestCase("applepenapple", new vector<string>{"apple", "pen"}, true),
        TestCase("catsandog", new vector<string>{"cats","dog","sand","and","cat"}, false)
    };

    bool failed = false;

    for (const auto &p : tests)
    {
        if (!test(p.m_s, (*p.m_dict), p.m_expected)) {
            failed = true;
            break;
        }
    }

    if (!failed)
        cout << "Passed all tests\n";

    cout << "Done running.\n";
    return 0;
}