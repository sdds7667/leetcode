
#include<bits/stdc++.h>

using namespace std;


class Solution {
public:
    Solution() {
        ios_base::sync_with_stdio(0);
        cin.tie(0);
        cout.tie(0);
    }
    int trap(vector<int>& height) {
        vector<int> r (height.size());
        for(int i = height.size() - 1, m = 0; i >= 0; i--) r[i] = (m = max(m, height[i]));
        int s = 0;
        for(int i = 0, m=height[0], c=height[0]; i < height.size(); c = height[++i]) 
            s += min((m = max(m,c)), r[i]) - c;
        return s;
    }
};

bool test(vector<int>& values, int expected) {
    auto result = (new Solution())->trap(values);
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
        make_pair(new vector<int>{0,1,0,2,1,0,1,3,2,1,2,1}, 6),
        make_pair(new vector<int>{4,2,0,3,2,5}, 9)
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