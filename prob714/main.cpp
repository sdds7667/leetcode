
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

    int maxProfit(vector<int> &prices, int fee)
    {
        int n = prices.size();
        vector<int> dp((n + 2) * 2, 0);

        for (int i = n - 1; i >= 0; i--)
        {
            dp[i * 2] = max(dp[(i + 1) * 2], dp[(i+1)*2+1] - prices[i]);
            dp[i * 2 + 1] = max(dp[(i+1) * 2 + 1], dp[(i+1)*2] + prices[i] - fee);
        }

        return dp[0];
    }
};

bool test(vector<int> &values, int fee, int expected)
{
    auto result = (new Solution())->maxProfit(values, fee);
    if (result != expected)
    {
        cout << "Got " << result << " expected " << expected << "\n";
        cout << "Failed test\n";
        for (const auto i : values)
            cout << i << " ";
        cout << "\n";
        return false;
    }
    return true;
}

struct TestCase
{
    vector<int> *m_prices;
    int m_fee;
    int m_expected;

    TestCase(vector<int> *prices, int fee, int expected) : m_prices(prices), m_fee(fee), m_expected(expected) {}
};

int main()
{
    cout << "Running"
         << "\n";
    vector<TestCase> tests{
        TestCase{new vector<int>{1, 3, 2, 8, 4, 9}, 2, 8},
        TestCase{new vector<int>{1, 3, 7, 5, 10, 3}, 3, 6},
        TestCase(new vector<int>{50,  7, 89, 82, 38, 61, 79,  6, 36, 55, 38,  5, 98, 63, 87}, 9, 244)    
    };

    bool failed = false;

    for (const auto &p : tests)
    {
        if (!test((*p.m_prices), p.m_fee, p.m_expected))
        {
            failed = true;
            break;
        }
    }

    if (!failed)
        cout << "Passed all tests\n";

    cout << "Done running.\n";
    return 0;
}