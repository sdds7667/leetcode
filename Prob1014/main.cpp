
#include<bits/stdc++.h>

using namespace std;


class Solution {
public:
    Solution() {
        ios_base::sync_with_stdio(0);
        cin.tie(0);
        cout.tie(0);
    }
    int maxScoreSightseeingPair(vector<int>& values) {
        int m = INT_MIN;
        int dp = values[0] - 1;
        for(auto i = 1; i < values.size(); i++) {
            m = max(m, values[i] + dp);
            dp = max(values[i] - 1, dp - 1);
        }
        return m;
    }
};

bool test(vector<int>& values, int expected) {
    auto result = (new Solution())->maxScoreSightseeingPair(values);
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
        make_pair(new vector<int>{8,1,5,2,6},11),
        make_pair(new vector<int>{1,2},2),
        make_pair(new vector<int>{9, 2, 4, 5, 6, 8, 8, 4, 5, 9}, 15),
        make_pair(new vector<int>{3, 4, 6, 3, 4, 2, 3, 3, 8, 6}, 13)
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