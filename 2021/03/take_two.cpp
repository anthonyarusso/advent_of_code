#include <bits/stdc++.h>

using namespace std;

unsigned int to_decimal(string& bits);

int main() {
	int line_count = 0;
	unsigned int bit_value = 0;
	vector<unsigned int> nums, oxys, co2s;
	string line;
	ifstream file;
	file.open("test1.txt");

	if (file.is_open()) {
		getline(file, line);
		nums.push_back(to_decimal(line));
		line_count += 1;
		int line_length = line.length();

		while (getline(file, line)) {
			// Push the converted number as an unsigned int.
			nums.push_back(to_decimal(line));
			line_count += 1;
		}

		// For each bit index find the most common bit value
		// out of the REMAINING numbers.
		unsigned int bit_mask = pow(2, line_length - 1);

		oxys = nums;
		co2s = nums;
		int ones = 0;

		while (bit_mask > 0) {
			ones = 0;
			if (oxys.size() > 1) {
				for (auto n : oxys) {
					if ((bit_mask & n) == bit_mask) {
						ones += 1;
					}
				}

				if (ones >= oxys.size() / 2) {
					bit_value = bit_mask;
				} else {
					bit_value = 0;
				}

				auto oxy_iter = oxys.begin();
				while (oxy_iter != oxys.end()) {
					if (*oxy_iter & bit_mask != bit_value) {
						bitset<sizeof(unsigned int) * 8> bs(bit_mask);
						bitset<sizeof(unsigned int) * 8> ox(*oxy_iter);
						cout << endl;
						cout << "bit_mask:  " << bs << endl;
						cout << "*oxy_iter: " << ox << endl;
						oxy_iter = oxys.erase(oxy_iter);
					} else {
						oxy_iter += 1;
					}
				}
			}

			ones = 0;
			if (co2s.size() > 1) {
				for (auto n : co2s) {
					if ((bit_mask & n) == bit_mask) {
						ones += 1;
					}
				}

				if (ones < co2s.size() / 2) {
					// Default to 1 for co2s.
					bit_value = bit_mask;
				} else {
					bit_value = 0;
				}

				auto co2_iter = co2s.begin();
				while (co2_iter != co2s.end()) {
					if  (*co2_iter & bit_mask != bit_value) {
						bitset<sizeof(unsigned int) * 8> bs(bit_mask);
						bitset<sizeof(unsigned int) * 8> co(*co2_iter);
						cout << endl;
						cout << "bit_mask:  " << bs << endl;
						cout << "*co2_iter: " << co << endl;
						co2_iter = co2s.erase(co2_iter);
					} else {
						co2_iter += 1;
					}
				}
			}

			// Shift our bit_mask right one.
			bit_mask >>= 1;
		}

		cout << "oxys.size(): " << oxys.size()
			<< " co2s.size(): " << co2s.size()	
			<< " oxys[0]: " << oxys[0]
			<< " co2s[0]: " << co2s[0] << endl;

		cout << "product: " << (oxys[0] * co2s[0]) << endl;

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
