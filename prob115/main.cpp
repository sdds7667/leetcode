
#include<bits/stdc++.h>

using namespace std;


class Solution {
public:
    Solution() {
        ios_base::sync_with_stdio(0);
        cin.tie(0);
        cout.tie(0);
    }
   
    int numDistinct(string s, string t) {
        int r = s.size();
        int c = t.size();
        vector<vector<int>> dp(r + 1, vector<int>(c + 1));
        for(int i = 0; i < r + 1; i++) 
            dp[i][c] = 1;
        for(int i = r - 1; i >= 0; i--)
            for(int j = min(i, c - 1); j >= 0; j--)
                dp[i][j] = (long) min( ((long) dp[i+1][j] + ((s[i] == t[j]) ? dp[i+1][j+1] : 0)), (long) INT_MAX - 1);
        return dp[0][0];
    }
};



int main() {
   /* No tests */
}