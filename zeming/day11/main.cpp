#include <bits/stdc++.h>
using namespace std;

typedef int i32;
typedef long i64;
typedef unsigned int u32;

int main() {
    i64 row = 10;
    i64 col = 10;
    string temp = 0;

    vector<vector<i64>> oct;
    for (i64 i = 0; i < row; i++) {
        vector<i64> o;
        cin >> temp;
        for (i64 j = 0; j < col; j++) {
            o.push_back(atoi(temp[j]));
        }
        oct.push_back(o);
    }

    for (auto e : oct) {
        for (auto e2: e) {
            cout << e2;
        }
        cout << endl;
    }

    return 0;
}
