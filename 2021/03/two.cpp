#include <bits/stdc++.h>

using namespace std;

unsigned int to_decimal(string& bits);

/* The `bit_value` approach was incorrect as the
 * bit mask produced powers of 2, not necessarily
 * 1 or 0.
 */

int main() {
	int line_count = 0;
	bool bit_value = false;
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
		// cout << "bit_mask: " << bs << endl;

		oxys = nums;
		co2s = nums;
		int ones = 0;

		while (bit_mask > 0) {
			// Determine the most common bit value at this
			// position for the remaining oxys.
			ones = 0;
			if (oxys.size() > 1) {
				for (auto n : oxys) {
					// If the bit in the current position is `1`.
					if ((bit_mask & n) == bit_mask) {
						ones += 1;
					}
				}
				
				cout << "oxy ones: " << ones << endl;
				if (ones >= oxys.size() / 2) {
					// Default to 1 for oxys.
					bit_value = true;
				} else {
					bit_value = false;
				}

				auto oxy_iter = oxys.begin();
				while (oxy_iter != oxys.end()) {
					// If current value in oxys does NOT contain the
					// most common bit value at the specified position,
					// remove it.
					cout << "mask: " << (*oxy_iter & bit_mask);
					cout << " bool: " << ((*oxy_iter & bit_mask) != bit_value) << endl;
					if ((*oxy_iter & bit_mask) != bit_value) {
						oxy_iter = oxys.erase(oxy_iter);
					} else {
						// Otherwise, retain this value by incrementing
						// the iterator to continue.
						oxy_iter += 1;
					}
				}
			}
			cout << "oxys.size(): " << oxys.size()
				<< " bit_value: " << bit_value
				<< " ones: " << ones << endl;


			// Determine the most common bit value at this
			// position for the remaining co2s.
			ones = 0;
			if (co2s.size() > 1) {
				for (auto n : co2s) {
					// If the bit in the current position is `1`.
					if ((bit_mask & n) == bit_mask) {
						ones += 1;
					}
				}
				
				if (ones > co2s.size() / 2) {
					// Default to 0 for co2s.
					bit_value = true;
				} else {
					bit_value = false;
				}

				auto co2_iter = co2s.begin();
				while (co2_iter != co2s.end()) {
					// If current value in co2s DOES contain the
					// most common bit value at the specified position,
					// remove it.
					if ((*co2_iter & bit_mask) == bit_value) {
						co2_iter = co2s.erase(co2_iter);
					} else {
						// Otherwise, retain this value by incrementing
						// the iterator to continue.
						co2_iter += 1;
					}
				}
			}

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
