
#include<bits/stdc++.h>

using namespace std;


class Solution {
public:
    Solution() {
        ios_base::sync_with_stdio(0);
        cin.tie(0);
        cout.tie(0);
    }
    vector<vector<int>> generate(int numRows) {

        vector<vector<int>> r {{1}};
        if (numRows == 1) return r;
        r.push_back(vector<int>{1,1});
        
        for(int i = 2; i < numRows; i++) {
            r.push_back(vector<int>{1});
            auto& v = r[i];
            auto& s = r[i-1];
            v.reserve(s.size() + 1);
            for(int j = 1; j < s.size(); j++) 
                v.push_back(s[j-1] + s[j]);
            v.push_back(1);
        }
        return r;
    }
    
};



int main() {
    cout << "Tests not provided" << "\n";
    return 0;
}