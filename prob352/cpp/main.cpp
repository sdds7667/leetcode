#include <iostream>
#include <vector>

using namespace std;

struct DisjointSet {
    vector<int> parents;
    vector<int> max;
    DisjointSet(): parents(std::move(vector<int>(10002, -1))), max(std::move(vector<int>(10001, -1))) {
    }

    int find(int value) {
        if (parents[value] == -1) return -1;
        if (parents[value] == value) return value;
        return parents[value] = find(parents[value]);
    }

    bool merge(int value) {
        if (find(value) != -1) return false;
        auto cp = value;
        auto cm = value;
        if (value != 0) {
            auto lp = find(value - 1);
            if (lp != -1)
                cp = lp;
        }

        auto rp = find(value+1);
        if (rp != -1) {
            parents[rp] = cp;
            cm = max[rp];
        }

        max[cp] = cm;
        parents[value] = cp;
        if (cp != value) {
            max[value] = cm;
        }
        return true;
    }
};

class SummaryRanges {
    vector<int> ranges;
    DisjointSet disjointSet;
    bool sorted = true;
public:
    SummaryRanges() {

    }

    void addNum(int value) {
        if (disjointSet.merge(value)) {
            ranges.push_back(value);
            sorted = false;
        }
    }

    vector<vector<int>> getIntervals() {
        vector<vector<int>> results;
        vector<int> new_intervals;

        if (!sorted) {
            sorted = true;
            std::sort(ranges.begin(), ranges.end());
        }

        for(auto i: ranges) {
            if (disjointSet.parents[i] == i) {
                results.push_back(vector<int>{i, disjointSet.max[i]});
                new_intervals.push_back(i);
            }
        }
        ranges = std::move(new_intervals);
        return results;
    }
};

int main() {
    std::cout << "Hello, World!" << std::endl;
    return 0;
}
