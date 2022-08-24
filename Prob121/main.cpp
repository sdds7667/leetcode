
#include<bits/stdc++.h>

using namespace std;


class Solution {
public:
    Solution() {
        ios_base::sync_with_stdio(0);
        cin.tie(0);
        cout.tie(0);
    }
    int maxProfit(vector<int>& prices) {
        if (prices.size() == 1) return 0;
        int dp = prices[0];
        int m = 0;
        for (int i = 1; i < prices.size(); i++) {
            m = max(m , prices[i] - dp);
            dp = min(dp, prices[i]);
        }
        return m;
    }
};

bool test(vector<int> values, int expected) {
    auto result = (new Solution())->maxProfit(values);
    if (result != expected) {
        cout << "Got " << result << " expected " << expected << "\n";
        cout << "Failed test\n";
        for(const auto i: values)
            cout << i << " ";
        cout << "\n";
        return false;
    }
    return true;
}

int main() {
    cout << "Running" << "\n";
    vector<pair<vector<int> *, int>> tests {
        make_pair(new vector<int>{7,1,5,3,6,4},5),
        make_pair(new vector<int>{7,6,4,3,1}, 0)
    };

    bool failed = false;

    for(const auto& p : tests) {
        if (!test((*p.first), p.second)) {
            failed = true;
            break;
        }
    }

    if (!failed) 
        cout << "Passed all tests\n";

    cout << "Done running.\n";
    return 0;
}