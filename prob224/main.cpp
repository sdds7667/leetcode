#include <iostream>
#include <string>
#include <vector>
using namespace std;




class Solution {
public:
    int calculate(string s) {
        int i = 0;
        string ns;
        for(char c: s)
            if (c != ' ')
                ns.push_back(c);
        return get_ast(s, &i);
    }

    long get_ast(const string& s, int* i) {
        long value = get_operator(s, i);
        (*i)++;
        for(int n = s.length(); (*i) < n; (*i)++) {
            char c = s[(*i)];
            if (c == '+') {
                (*i)++;
                value += get_operator(s, i);
            } else if (c == '-') {
                (*i)++;
                value -= get_operator(s, i);
            } else if (c == ')') {
                return value;
            }
        }
        return value;
    }

    long get_operator(const string& s, int* i) {
        int negative = 1;
        long current_number = 0;
        int state = 0;
        char c;
        for(int n = s.length(); (*i) < n; (*i)++) {
            switch (state) {
                case 2:
                    c = s[(*i)];
                    if ('0' <= c && c <= '9')
                        current_number = current_number * 10 + c - '0';
                    else {
                        (*i)--;
                        return negative * current_number;
                    }
                    break;
                case 1:
                    c = s[(*i)];
                    if ('0' <= c && c <= '9') {
                        state = 2;
                        current_number = c - '0';
                    } else {
                        (*i)++;
                        return negative * get_ast(s, i);
                    }
                    break;
                case 0:
                    c = s[(*i)];
                    if (c == '-') {
                        negative = -1;
                        state = 1;
                    } else {
                        state = 1;
                        (*i)--;
                    }
                    break;
            }
        }
        return negative * current_number;
    }
};


int main() {
    cout << Solution().calculate("(1+(4+5+2)-3)+(6+8)") << "\n";
    return 0;
}
