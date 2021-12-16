#include <bits/stdc++.h>

using namespace std;

int main() {
	/* window[] is used as a **circular array**!
	 * Derive the "zero index" / "oldest member"
	 * by taking the modulo of the numbers we
	 * have counted thus far (nums).
	 */

	// For the first four entries the oldest
	// member will be index 0 until an
	// overwrite occurs.
	
	// Afterwards the oldest member will shift
	// and always be the next member following
	// the overwritten index.
	int window[4] = {};
	int nums = 0;
	int count = 0;
	string line;
	ifstream file;
	file.open("input.txt");

	if (file.is_open()) {
		getline(file, line);
		window[nums] = stoi(line);
		nums += 1;

		while (getline(file, line)) {
			window[nums % 4] = stoi(line);
			nums += 1;
	
			// Only once our windows are filled.
			if (nums >= 4) {
				// Index of the oldest member.
				int oldest = nums % 4;
				// Instead of checking the sums just compare
				// the numbers which are exclusive to the
				// first and second window (oldest and newest
				// indices).
				int one = window[oldest];
				int two = window[(oldest + 3) % 4];
				if (two > one) {
					count += 1;
				}
			}
		}

		cout << count << endl;

		file.close();
	} else {
		cout << "Unable to open file.";
	}

	return 0;
}
