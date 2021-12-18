#include <bits/stdc++.h>

using namespace std;

unsigned int to_decimal(string& bits);

int main() {
	int line_count = 0;
	unsigned int bit_value;
	vector<unsigned int> nums, oxys, co2s;
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

		oxys = nums;
		co2s = nums;
		int ones = 0;

		cout << "oxys.size(): " << oxys.size() << endl;
		while (bit_mask != 0) {
			bitset<sizeof(unsigned int) * 8> bs(bit_mask);
			cout << "bit_mask: " << bs << endl;
			// Determine the most common bit value at this
			// position for the remaining oxys.
			ones = 0;
			for (auto n : oxys) {
				// If the bit in the current position is `1`.
				if (bit_mask & n == bit_mask) {
					ones += 1;
				}
			}
			
			// If greater than or equal to half. Default to 1 if half.
			if (ones >= line_count / 2) {
				bit_value = 1;
			} else {
				bit_value = 0;
			}

			auto oxy_iter = oxys.begin();
			while (oxy_iter != oxys.end()) {
				// If current value in oxys does NOT contain the
				// most common bit value at the specified position,
				// remove it.
				if (*oxy_iter & bit_mask != bit_value) {
					oxy_iter = oxys.erase(oxy_iter);
				} else {
					// Otherwise, retain this value by incrementing
					// the iterator to continue.
					oxy_iter += 1;
				}
			}

			// Determine the most common bit value at this
			// position for the remaining oxys.

			ones = 0;
			for (auto n : co2s) {
				// If the bit in the current position is `1`.
				if (bit_mask & n == bit_mask) {
					ones += 1;
				}
			}
			
			// Only if greater than half. Default to 0 if half.
			if (ones > line_count / 2) {
				bit_value = 1;
			} else {
				bit_value = 0;
			}

			auto co2_iter = co2s.begin();
			while (co2_iter != co2s.end()) {
				// If current value in co2s DOES contain the
				// most common bit value at the specified position,
				// remove it.
				if (*co2_iter & bit_mask == bit_value) {
					co2_iter = co2s.erase(co2_iter);
				} else {
					// Otherwise, retain this value by incrementing
					// the iterator to continue.
					co2_iter += 1;
				}
			}

			// Shift our bit_mask right one.
			bit_mask >>= 1;
		}
		cout << "oxys.size(): " << oxys.size() << endl;
		cout << "co2s.size(): " << co2s.size() << endl;
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
