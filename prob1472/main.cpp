#include <vector>
#include <string>

using namespace std;
class BrowserHistory {
public:

    string history[5000];
    int index = 0;
    int maxIndex = 0;
    BrowserHistory(string homepage) {
        history[index] = homepage;
    }

    void visit(string url) {
        history[++index] = std::move(url);
        maxIndex = index;
    }

    string& back(int steps) {
        if (index > steps) index -= steps;
        else index = 0;
        return history[index];

    }

    string& forward(int steps) {
        if (index + steps >= maxIndex) index = maxIndex;
        else index += steps;
        return history[index];
    }
};

/**
 * Your BrowserHistory object will be instantiated and called as such:
 * BrowserHistory* obj = new BrowserHistory(homepage);
 * obj->visit(url);
 * string param_2 = obj->back(steps);
 * string param_3 = obj->forward(steps);
 */