#include <string>
#include <vector>
#include <iostream>

using namespace std;

class Solution {
public:
    vector<vector<string>> solveNQueens(int n) {
        vector<vector<string>> res;
        string s (n, '.');
        vector<string> board(n, s);
        vector<bool> cols(n, false);
        solve(n, res, board, cols, 0);
        return res;
    }

    void solve(int n, vector<vector<string>>& res, vector<string>& board, vector<bool>& cols, int i) {
        for(int j = 0; j < n; j++) {
            if (cols[j]) continue;
            // check diagonals
            bool d = false;
            for(int k = 1; i-k >= 0 && j-k >= 0; k++)
                if (board[i-k][j-k] == 'Q') {d = true;break;}
            if (d) continue;
            for(int  k = 1; i - k >= 0 && j+k < n; k++)
                if (board[i-k][j+k] == 'Q') {d = true;break;}
            if (d) continue;
            board[i][j] = 'Q';
            cols[j] = true;
            if (i == n-1) res.push_back(board);
            else solve(n, res, board, cols, i+1);
            board[i][j] = '.';
            cols[j] = false;
        }
    }

};

int main() {
    cout << Solution().solveNQueens(10).size() << "\n";
}