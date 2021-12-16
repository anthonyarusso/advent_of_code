#include <bits/stdc++.h>

using namespace std;

int main() {
	int prev, curr;
	int count = 0;
	string line;
	ifstream file;
	file.open("input.txt");
	
	if (file.is_open()) {
		getline(file, line);
		prev = stoi(line);

		while (getline(file, line)) {
			curr = stoi(line);
			if (curr > prev) {
				count += 1;
			}

			prev = curr;	
		}

		cout << count << endl;

		file.close();
	} else {
		cout << "Unable to open file.";
	}

	return 0;
}
