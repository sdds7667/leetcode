
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
    int maxProfit(vector<int> &prices)
    {
        int c = prices.size();
        vector<vector<int>> b(3, vector<int>(c + 1, 0));
        vector<vector<int>> s(3, vector<int>(c + 1, 0));

        for (int j = c - 1; j >= 0; j--)
        {
            for (int i = 2; i > 0; i--)
            {
                b[i][j] = max(b[i][j + 1], s[i][j + 1] - prices[j]);
                s[i][j] = max(s[i][j + 1], b[i - 1][j + 1] + prices[j]);
            }
        }

        return b[2][0];
    }
};

int main()
{
    cout << "No tests\n";
}