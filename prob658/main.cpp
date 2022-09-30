
#include<bits/stdc++.h>

using namespace std;


class Solution {
public:
    Solution() {
        ios_base::sync_with_stdio(0);
        cin.tie(0);
        cout.tie(0);
    }

    vector<int> findClosestElements(vector<int>& arr, int k, int x) {
        int l = 0;
        int r = arr.size() - 1;

        for(int cs = arr.size();cs > k;cs--) 
            if (abs(arr[l] - x) <= abs(arr[r] - x)) 
                r--;
            else
                l++;
        
        auto li = arr.begin() + l;
        auto ri = arr.begin() + r + 1;
        return vector<int>(li, ri);
    }
    
};

int main() {
    // no tests
}