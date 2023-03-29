#include <iostream>
#include <vector>
#include <unordered_map>

using namespace std;

struct DequeNode {
    DequeNode* prev;
    DequeNode* next;
    int key;
    int uses;

    void remove() {
        prev->next=next;
        next->prev=prev;
    }
};

struct Deque {
    DequeNode* head = nullptr;
    DequeNode* tail = nullptr;
    unordered_map<int, DequeNode*> position;

    DequeNode* pop(int key) {
        if (head == tail) {
            head = nullptr;
            auto* p = tail;
            tail = nullptr;
            position.erase(key);
            return p;
        } else if (head -> key == key) {
            auto* p = head;
            head = head->next;
            position.erase(key);
            return p;
        } else if (tail -> key == key) {
            auto *p = tail;
            tail = tail->prev;
            position.erase(key);
            return p;
        }

        const auto result = position.find(key);
        result->second->remove();
        auto* p = result->second;
        position.erase(key);
        return p;
    }

    DequeNode* pop_front() {
        if (head == nullptr) return nullptr;
        if (head == tail) {
            head = nullptr;
            auto* p = tail;
            tail = nullptr;
            position.erase(p->key);
            return p;
        }
        auto* p = head;
        head = head->next;
        position.erase(p->key);
        return p;
    }

    void push_back(DequeNode* node) {
        if (tail == nullptr) {
            head = tail = node;
            position[node->key] = node;
            return;
        }
        tail->next = node;
        node->next = nullptr;
        node->prev = tail;
        tail = node;
        position[node->key] = node;
    }
};

class LFUCache {
    unordered_map<int, int> cache;
    unordered_map<int, int> use_counter;
    unordered_map<int, Deque> order;
    int least_uses = 0;
    int used = 0;
    int capacity;
public:
    LFUCache(int capacity): capacity(capacity) {

    }

    int get(int key) {
        const auto result = cache.find(key);
        if (result == cache.end()) return -1;
        int u = use_counter[key];
        auto& o = order[u];
        u++;
        order[u].push_back(o.pop(key));
        use_counter[key]++;
        return result->second;
    }

    void put(int key, int value) {
        if (capacity == 0) return;
        const auto result = cache.find(key);
        if (result == cache.end()) {
            cache[key] = value;

            // capacity check
            if (used == capacity) {
                used--;
                for(int i = least_uses; i < 10000; i++) {
                    const auto p = order[i].pop_front();
                    if (p == nullptr) continue;
                    cache.erase(p->key);
                    use_counter.erase(p->key);
                    break;
                }
            }
            used++;
            use_counter[key] = 1;
            auto* d = new DequeNode();
            d->key = key;
            order[1].push_back(d);
            least_uses = 1;

        } else {
            result -> second = value;
            int u = use_counter[key];

            if (u == least_uses && order[u].head == nullptr)
                least_uses++;
            auto& o = order[u];
            order[++u].push_back(o.pop(key));
            use_counter[key]++;
        }
    }
};

int main() {
    auto cache = LFUCache(10);

    vector<string> ops {"put","put","put","get","put","put","get","put","put","get","put","get","get","get","put","put","get","put","get"};
    vector<vector<int>> args{{7,28},{7,1},{8,15},{6},{10,27},{8,10},{8},{6,29},{1,9},{6},{10,7},{1},{2},{13},{8,30},{1,5},{1},{13,2},{12}};

    for(int i = 0; i < ops.size(); i++) {
        if (ops[i] == "put") {
            cache.put(args[i][0], args[i][1]);
        } else {
            cout << cache.get(args[i][0]) << "\n";
        }
    }

}
