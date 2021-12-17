#include <bits/stdc++.h>

using namespace std;

unsigned int to_decimal(string& bits);

int main() {
	int line_count = 0;
	unsigned int bit_value;
	vector<unsigned int> nums;
	string line;
	ifstream file;
	file.open("input.txt");

	if (file.is_open()) {
		getline(file, line);
		int length = line.length();
		nums.push_back(to_decimal(line));

		for (int i = 0; i < length; i++) {
			if (line[i] == '1') {
			}
		}
		line_count += 1;

		while (getline(file, line)) {
			// Push the converted number as an unsigned int.
			nums.push_back(to_decimal(line));
			line_count += 1;
		}

		// For each bit index find the most common bit value
		// out of the REMAINING numbers.
		unsigned int bit_mask = pow(2, length - 1);
		bitset<sizeof(unsigned int) * 8> bs(bit_mask);
		cout << "bit_mask: " << bs << endl;

		while (bit_mask != 0) {
			// Determine the most common bit value at this position.
			/*
			int ones = 0;
			for (auto n : oxys) {
				// If the bit in the current position is `1`.
				if (bit_mask & n == bit_mask) {
					ones += 1;
				}
			}
			
			if (ones >= line_count / 2) {
				bit_value = 1;
			} else {
				bit_value = 0;
			}

			// Remove all numbers that do not contain the most
			// common bit value at this position.
			for (size_t i = 0; i < oxys.size(); i++) {
				if (bit_mask & oxys[i] != bit_value) {
					oxys.erase(oxys.begin() + i);
				} else {
					co2s.erase(co2s.begin() + i);
				}
			}
			*/

			// Shift our bit_mask right one.
			bit_mask >>= 1;
		}

	} else {
		cout << "Unable to open file." << endl;
	}

	return 0;
}

unsigned int to_decimal(string& bits) {
	unsigned int result = 0;

	for (int i = 0; i < bits.length(); i++) {
		if (bits[i] == '1') {
			// As index increases the power of 2 decreases.
			result += pow(2, bits.length() - i - 1);
		}
	}
	
	return result;
}
