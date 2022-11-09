
#include<vector>
#include<iostream>

using namespace std;

class StockSpanner {
    vector<int> m_xs;
    vector<int> m_ls;
public:

    StockSpanner() {
    }
    
    int next(int price) {
        int t = 1;
        while ((!m_xs.empty()) && m_xs.back() <= price) {
            t += m_ls.back();
            m_ls.pop_back();
            m_xs.pop_back();
        }
        m_xs.push_back(price);
        m_ls.push_back(t);
        return t;
    }
};

int main() {

}