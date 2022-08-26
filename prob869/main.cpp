
#include<bits/stdc++.h>

using namespace std;


class Solution {
public:
    Solution() {
        ios_base::sync_with_stdio(0);
        cin.tie(0);
        cout.tie(0);
    }
    bool reorderedPowerOf2(int n) {
        if (n == 0) return false;
        vector<int> c(10, 0);
        vector<int> ic(10, 0);
        int cn = 0;
        for(;n > 0; n /= 10, cn++) {
            c[n%10]++;
        }

        
        for(int i = 1;; i <<= 1) {
            fill(ic.begin(), ic.end(), 0);
            int cj = 0;
            for(int j = i; j > 0; j /= 10, cj++) 
                ic[j%10]++;
            if (cj < cn) continue;
            if (cj > cn) break;
            if (c == ic) return true;
        }
        return false;
    }
};

bool test(int n, bool expected) {
    cout << "New Test =================================================\n";
    auto result = (new Solution())->reorderedPowerOf2(n);
    if (result != expected) {
        cout << "Got " << result << " expected " << expected << "\n";
        cout << "Failed test\n";
        cout << n;
        cout << "\n";
        return false;
    }
    return true;
}

int main() {
    cout << "Running" << "\n";
    vector<pair<int, bool>> tests {
        make_pair(1, true),
        make_pair(10, false)
    };

    bool failed = false;

    for(const auto& p : tests) {
        if (!test(p.first, p.second)) {
            failed = true;
            break;
        }
    }

    if (!failed) 
        cout << "Passed all tests\n";

    cout << "Done running.\n";
    return 0;
}