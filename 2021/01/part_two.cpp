#include <bits/stdc++.h>

using namespace std;

int main() {
	int win1[3] = {}, win2[3] = {};
	int count = 0;
	string line;
	ifstream file;
	file.open("input.txt");

	if (file.is_open()) {
		getline(file, line);
		win1[0] = stoi(line);

		while (getline(file, line)) {
		}

		cout << count << endl;

		file.close();
	} else {
		cout << "Unable to open file.";
	}

	return 0;
}
