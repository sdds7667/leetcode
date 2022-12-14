
#include<bits/stdc++.h>

using namespace std;


class Solution {
public:
    int maximalRectangle(vector<vector<char>>& matrix) {
        vector<int> h(matrix[0].size(), 0);
        int r = matrix.size(), c = matrix[0].size();
        int m;
        for(int i = 0; i < r; i++) {
            for(int j = 0; j < c; j++) 
                h[j] = ((matrix[i][j] == '0') ? 0 : (h[j] + 1));
            m = max(largestRectangleArea(h), m);
        }
        return m;
    }
    
    int largestRectangleArea(vector<int>& heights) {
        heights.push_back(0);
        vector<pair<int, int>> s;
        s.emplace_back(0, heights[0]);
        int m = heights[0];
        for(int i = 1; i < heights.size(); i++) {
            int h = heights[i];
            if (h == heights[i-1]) continue;
            if (heights[i] > s.back().second ) {
                s.emplace_back(i, h);
                continue;
            }
            
            int l = i;
            int ch;
            while (!s.empty() && h < s.back().second) {
                auto& p = s.back();
                s.pop_back();
                ch = p.second;
                l = p.first;
                m = max(m, ch * (i - l));
            }
            s.emplace_back(l, h);
        }
        return m;
    }
};

// bool test(UNDEFINED values, int expected) {
//     auto result = (new Solution())->;
//     if (result != expected) {
//         cout << "Got " << result << " expected " << expected << "\n";
//         cout << "Failed test\n";
//         for(const auto i: values)
//             cout << i << " ";
//         cout << "\n";
//         return false;
//     }
//     return true;
// }

// int main() {
//     cout << "Running" << "\n";
//     vector<pair<vector<int> *, int>> tests {
//     };

//     bool failed = false;

//     for(const auto& p : tests) {
//         if (!test((*p.first), p.second)) {
//             failed = true;
//             break;
//         }
//     }

//     if (!failed) 
//         cout << "Passed all tests\n";

//     cout << "Done running.\n";
//     return 0;
// }
int main() {

    /* -- no tests -- */
}