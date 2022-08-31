
#include<bits/stdc++.h>

using namespace std;


class Solution {
public:
    Solution() {
        ios_base::sync_with_stdio(0);
        cin.tie(0);
        cout.tie(0);
    }
    int uniquePaths(int m, int n) {
        vector<vector<int>> dp(m, vector<int>(n, 1));
        for(int i = 1; i < m; i++) 
            for(int j = 1; j < n; j++) 
                dp[i][j] = dp[i][j-1] + dp[i-1][j];
        return dp[m-1][n-1];
    }
};

bool test(int m, int n, int expected) {
    auto result = (new Solution())->uniquePaths(m, n);
    if (result != expected) {
        cout << "Got " << result << " expected " << expected << "\n";
        cout << "Failed test\n";
        cout << m << " " << n;
        cout << "\n";
        return false;
    }
    return true;
}

int main() {
    cout << "Running" << "\n";
    vector<pair<pair<int, int>, int>> tests {
        make_pair(make_pair(3, 7), 28),
        make_pair(make_pair(3, 2), 3)
    };

    bool failed = false;

    for(const auto& p : tests) {
        if (!test(p.first.first, p.first.second, p.second)) {
            failed = true;
            break;
        }
    }

    if (!failed) 
        cout << "Passed all tests\n";

    cout << "Done running.\n";
    return 0;
}