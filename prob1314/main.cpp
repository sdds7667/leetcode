
#include<bits/stdc++.h>

using namespace std;


class Solution {
public:
    Solution() {
        ios_base::sync_with_stdio(0);
        cin.tie(0);
        cout.tie(0);
    }

    vector<vector<int>> matrixBlockSum(vector<vector<int>>& mat, int k) {
        /* prefix sum */
        int r = mat.size();
        int c = mat[0].size();
        int cs = 0;

        for(auto j = 1; j < c; j++) 
            mat[0][j] += mat[0][j-1];
        

        for(auto i = 1; i < r; i++) {
            cs = 0;
            for(int j = 0; j < c; j++) {
                cs += mat[i][j];
                mat[i][j] = cs + mat[i-1][j];
            }
        }
        
        vector<vector<int>> rs(r, vector<int>(c, 0));
        
        for(int i = 0; i < r; i++) 
            for(int j = 0; j < c; j++) 
                {
                    rs[i][j] = get(mat, i + k, j + k) 
                        + get(mat, i-k-1, j-k-1) 
                        - get(mat, i-k-1, j+k) 
                        - get(mat, i+k, j-k-1);
                }
         
         return rs;  
         
    }        
    
    
    int get(vector<vector<int>>&mat, int i, int j) {
        if (i < 0 || j < 0) return 0;
        return mat[min((int) mat.size()-1, i)][min((int) mat[0].size()-1, j)];
    }
};


int main() {
    auto rs = (new Solution())->matrixBlockSum( (*(new vector<vector<int>>{{1, 2, 3}, {4, 5, 6}, {7, 8, 9}})), 1);
    for(const auto& r: rs) {
        for(auto j: r) 
            cout << j << " ";
        cout << "\n";
    }
}