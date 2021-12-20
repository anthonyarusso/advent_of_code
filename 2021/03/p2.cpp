#include <bits/stdc++.h>

#define U_INT_BITS sizeof(unsigned int) * 8

using namespace std;

unsigned int to_decimal(string&);

int main() {
	string line;
	int line_count = 0;
	vector<unsigned int> nums = {};
	ifstream file;

	file.open("input.txt");

	if (file.is_open()) {
		getline(file, line);
		line_count += 1;
		unsigned int line_length = line.length();
		nums.push_back(to_decimal(line));

		while (getline(file, line)) {
			line_count += 1;
			nums.push_back(to_decimal(line));
		}

		unsigned int bit_mask = pow(2, line_length - 1);
		vector<unsigned int> oxys = nums;
		vector<unsigned int> co2s = nums;

		while (bit_mask > 0) {
			if (oxys.size() > 1) {
				// Determine the most common bit value.
				int ones = 0;
				for (auto o : oxys) {
					if ((o & bit_mask) == bit_mask) {
						ones += 1;
					}
				}
				unsigned int most_common;
				if (ones >= ceil(oxys.size() / 2.0)) {
					most_common = bit_mask;
				} else {
					most_common = 0;
				}

				// Remove elements which do not meet the bit criteria.
				vector<unsigned int> tmp_oxys = oxys;
				oxys.clear();
				for (auto tmp : tmp_oxys) {
					if ((tmp & bit_mask) == most_common) {
						oxys.push_back(tmp);
					}
				}
			}

			if (co2s.size() > 1) {
				// Determine the most common bit value.
				int ones = 0;
				for (auto o : co2s) {
					if ((o & bit_mask) == bit_mask) {
						ones += 1;
					}
				}
				unsigned int least_common;
				if (ones < ceil(co2s.size() / 2.0)) {
					least_common = bit_mask;
				} else {
					least_common = 0;
				}

				// Remove elements which do not meet the bit criteria.
				vector<unsigned int> tmp_co2s = co2s;
				co2s.clear();
				for (auto tmp : tmp_co2s) {
					if ((tmp & bit_mask) == least_common) {
						co2s.push_back(tmp);
					}
				}
			}

			bit_mask >>= 1;
		}
			cout << "oxys.size(): " << oxys.size()
				<< " oxys[0]: " << oxys[0] << endl;
			cout << "co2s.size(): " << co2s.size()
				<< " co2s[0]: " << co2s[0] << endl;
	} else {
		cout << "Failed to open file." << endl;
	}

	return 0;
}

unsigned int to_decimal(string& bits) {
	unsigned int result = 0;

	for (int i = 0; i < bits.length(); i++) {
		if (bits[i] == '1') {
			result += pow(2, bits.length() - i - 1);
		}
	}

	return result;
}
