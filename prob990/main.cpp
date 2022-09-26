
#include<bits/stdc++.h>

using namespace std;


class Solution {
public:
    int parent[26];
    int rank[26];

    bool equationsPossible(vector<string>& equations) {
        init();
        for(const auto& eq: equations) {
            if (eq[1] == '!') continue;
            merge(eq[0] - 'a', eq[3] - 'a');
        }

        for(const auto& eq: equations) {
            if (eq[1] == '=') continue;
            if (find(eq[0] - 'a') == find(eq[3] - 'a')) {
                // cout << "eq[0] = '" << eq[0] << "';\n eq[3] = " << eq[3] << "'\n'";

                return false;
            }
        }
        return true;
    }


    void init() {
        for(int i = 0; i < 26; i++)
            parent[i] = i;
    }

    int find(int i) {
        if (parent[i] == i) return i;
        return (parent[i] = find(parent[i]));
    }

    void merge(int a, int b) {
        int pa = find(a);
        int pb = find(b);
        if (pa == pb) return;
        if (rank[pa] == rank[pb])
        {
            rank[pa] ++;
            parent[pb] = pa;
        }
        else if (rank[pa] > rank[pb])
            parent[pb] = pa;
        else 
            parent[pa] = pb;
        
    }
};

bool test(vector<string>& values, int expected) {
    auto result = (new Solution())->equationsPossible(values);
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
    vector<pair<vector<string> *, bool>> tests {
        make_pair(new vector<string>{"a==b","b!=a"}, false),
        make_pair(new vector<string>{"b==a","a==b"}, true),
        make_pair(new vector<string>{"a==a","b==a","c==b","c==d","d==e","e!=a"}, false),
        make_pair(new vector<string>{"a==a","b==a","c==b","c==d","d==e","b!=a"}, false),
        make_pair(new vector<string>{"c==c","b==d","x!=z"}, true)
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