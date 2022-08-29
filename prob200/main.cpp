
#include<vector>
#include<iostream>
using namespace std;


class Solution {
public:
    Solution() {
        ios_base::sync_with_stdio(0);
        cin.tie(0);
        cout.tie(0);
    }
    int numIslands(vector<vector<char>>& grid) {
        int c = 0;
        for (int i = 0; i < grid.size(); i++) {
            for (int j = 0; j < grid[i].size(); j++) {
                if (grid[i][j] == '1') {
                    dfs(grid, i, j);
                    c++;
                }
            }
        }
        return c;
    }

    void dfs(vector<vector<char>>& grid, int sx, int sy) {
        if (sx < 0 || sx >= grid.size() || sy < 0 || sy >= grid[0].size() || grid[sx][sy] != '1') return;
        grid[sx][sy] = '0';
        dfs(grid, sx - 1, sy);
        dfs(grid, sx + 1, sy);
        dfs(grid, sx, sy - 1);
        dfs(grid, sx, sy + 1);
    }
};

bool test(vector<vector<char>>& values, int expected) {
    auto result = (new Solution())->numIslands(values);
    if (result != expected) {
        cout << "Got " << result << " expected " << expected << "\n";
        cout << "Failed test\n";
        for (const auto& i : values) {
            for (const auto& j : i)
                cout << j << " ";
            cout << "\n";
        }
        return false;
    }
    return true;
}

int main() {
    cout << "Running" << "\n";
    vector<pair<vector<int> *, int>> tests {
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