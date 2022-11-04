
#include<vector>
#include<string>
#include<iostream>
#include<queue>
#include<algorithm>


using namespace std;

class Solution {
public:
    vector<vector<string>> findLadders(string beginWord, string endWord, vector<string>& wordList) {

        vector<vector<string>> solutions;
        int n = wordList.size();
        int end_index = find(wordList.begin(), wordList.end(), endWord) - wordList.begin();
        
        if (end_index == n) 
            return solutions;

        int start_index = find(wordList.begin(), wordList.end(), beginWord) - wordList.begin();
        if (start_index == n) 
            wordList.push_back(beginWord);

        const auto bfs_results = bfs(wordList, start_index, end_index);
        // debug_lists(wordList, bfs_results.second, "parents");
        vector<string> current(bfs_results.first);
       
        dfs(solutions, wordList, bfs_results.second, current, current.size()-1, start_index, end_index);

        return solutions;
    }

    bool can_ladder_from(const string& l, const string& r) {
        int diff = 0; 
        for(int i = 0, wl = l.length(); i < wl && diff < 2; i++)
            diff += l[i] != r[i];
        return diff == 1;
    }

    vector<vector<int>> build_adjacency_list(const vector<string>& word_list) {
        vector<vector<int>> adj_list(word_list.size());
        for(int i = 0, n = word_list.size(); i < n; i++) 
            for(int j = i + 1; j < n; j++) {
                if (can_ladder_from(word_list[i], word_list[j])) {
                    adj_list[i].push_back(j);
                    adj_list[j].push_back(i);
                }
            }
        return adj_list;
    }

    void debug_lists(const vector<string>& word_list,const vector<vector<int>>& target, const string& name) {
        for(int i = 0; i < target.size(); i++) {
            cout << name << "[" << word_list[i] << "] = {";
            for(const auto& j : target[i]) {
                cout << word_list[j] << ", ";
            }

            cout << "}" << endl;
        }
    }

    pair<int, vector<vector<int>>> bfs(
        const vector<string>& word_list,
        const int start,
        const int end
    ) {
        int n = word_list.size();
        vector<vector<int>> parents (n);
        vector<bool> queued(n);
        vector<int> visited(n, 501);
        const vector<vector<int>> adj_list = build_adjacency_list(word_list);
        // debug_lists(word_list, adj_list, "neighbours");
        queue<int> word_queue;
        visited[start] = 0;
        word_queue.push(start);
        int min_size = 0;

        while(!word_queue.empty()) {
            int size = word_queue.size();
            for(int _ = 0; _ < size; _++) {
                const int current_node = word_queue.front();
                // cout << "bfs from " << current_node << endl;
                if (current_node == end) return make_pair(min_size + 1, parents);
                word_queue.pop();
                for(const auto neighbour: adj_list[current_node]) {
                    if (visited[neighbour] <= min_size) continue;
                    parents[neighbour].push_back(current_node);
                    if (queued[neighbour]) continue;
                    queued[neighbour] = true;
                    visited[neighbour] = min_size + 1;
                    word_queue.push(neighbour);
                }
            }
            min_size += 1;
        }
        return make_pair(min_size, parents);
    }

    void dfs(
        vector<vector<string>>& solution,
        const vector<string>& word_list,
        const vector<vector<int>>& parents,
        vector<string>& current,
        const int current_index,
        const int start_index,
        const int i
    ) {
        current[current_index] = word_list[i];
        if (i == start_index) {
            solution.push_back(current);
            return;
        }

        for(const auto parent: parents[i]) 
            dfs(solution, word_list, parents, current, current_index - 1, start_index, parent);
    }
};

int main() {
    vector<string> word_list {"hot","dog"};

    for (const auto& wl: Solution().findLadders("hot", "dog", word_list)) {
        cout << "{";
        for (const auto& w: wl) {
            cout << w << " ";
        }
        cout << "}" << endl;
    }
}
