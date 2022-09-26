
#include<bits/stdc++.h>

using namespace std;


class Solution {
public:
    Solution() {
        ios_base::sync_with_stdio(0);
        cin.tie(0);
        cout.tie(0);
    }

    vector<int> searchRange(vector<int>& nums, int target) {
        
        int l = 0;
        int r = nums.size() - 1;


        while(l <= r) {
            cout << "global bin_search between " << l << " and " << r << "\n";
            int m = (l+r)/2;
            if (nums[m] == target) {
                // compute lower and upper limit:
                int rr = m;
                int lrr = m;
                while(rr <= r) {
                    int nrr = (rr + r) / 2;
                    if (nums[nrr] == target) {
                        lrr = nrr;
                        rr = nrr + 1;
                    } else {
                        r = nrr - 1;
                    }
                }

                int ll = m;
                int lll = m;
                while(l <= ll) {
                    int nll = (ll + l) / 2;
                    if (nums[nll] == target) {
                        lll = nll;
                        ll = nll - 1;
                    } else {
                        l = nll + 1;
                    }
                }

                return vector<int> {lll, lrr};

            } else if (nums[m] > target) {
                r = m - 1;
            } else {
                l = m + 1;
            }
        }
        return vector<int>{-1, -1};
    }
    
};

bool test(vector<int>& values, int value, vector<int>& expected) {
    auto result = (new Solution())->searchRange(values, value);
    if (result != expected) {
        cout << "Failed test\n";
        for(const auto i: values)
            cout << i << " ";
        cout << "\n Got: ";
        for(const auto i: result) 
            cout << i << " ";
        cout << "\n Expected";
        for(const auto i: expected) 
            cout << i << " ";
        cout << "\n";

        return false;
    }
    return true;
}

int main() {
    cout << "Running" << "\n";
    vector<tuple<vector<int> *, int, vector<int> *>> tests {
        make_tuple(new vector<int>{5,7,7,8,8,10}, 8, new vector<int>{3,4}),
        make_tuple(new vector<int>{5,7,7,8,8,10}, 6, new vector<int>{-1, -1}),
        make_tuple(new vector<int>{}, 0, new vector<int>{-1, -1}),
        make_tuple(new vector<int>{6, 6, 6}, 6, new vector<int>{0, 2}),
        make_tuple(new vector<int>{6, 6}, 6, new vector<int>{0, 1})
    };

    bool failed = false;

    for(const auto& p : tests) {
        if (!test(*get<0>(p), get<1>(p), *get<2>(p))) {
            failed = true;
            break;
        }
    }

    if (!failed) 
        cout << "Passed all tests\n";

    cout << "Done running.\n";
    return 0;
}