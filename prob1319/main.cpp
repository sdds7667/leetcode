#include <iostream>
#include <vector>

using namespace std;


class UnionFind {
    vector<int> parent;
    vector<int> rank;
    int clusters;
public:
    explicit UnionFind(int n) {
        clusters = n;
        parent.reserve(n);
        for(int i = 0; i < n; i++) {
            parent.push_back(i);
            rank.push_back(0);
        }
    }

    int get_clusters() const {
        return clusters;
    }

    int find(int a) {
        if (parent[a] == a) return a;
        return parent[a] = find(parent[a]);
    }

    int merge(int a, int b) {
        int pa = find(a);
        int pb = find(b);
        if (pa == pb)
            return 1;
        if (rank[pa] > rank[pb])
            parent[pb] = pa;
        else if (rank[pa] < rank[pb])
            parent[pa] = pb;
        else {
            parent[pb] = pa;
            rank[pa]++;
        }
        clusters--;
        return 0;
    }
};

class Solution {
public:
    int makeConnected(int n, vector<vector<int>>& connections) {
        // usign a disjoint set, count the number of different clusters, and for each cluster, the number of redundant wires.
        // in the end, if the number of (redundant wires < clusters - 1) = return -1
        // else, return the number of the clusters.
        int redundantWires = 0;
        UnionFind unionFind{n};
        for(const auto& connection: connections)
            redundantWires += unionFind.merge(connection[0], connection[1]);
        int wireNeeded = unionFind.get_clusters() - 1;
        if (wireNeeded > redundantWires) return -1;
        return wireNeeded;
    }
};

int main() {
    std::cout << "Hello, World!" << std::endl;
    return 0;
}
