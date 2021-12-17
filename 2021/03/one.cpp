#include <bits/stdc++.h>

using namespace std;

int to_decimal(string& bits);

int main() {
	int line_count = 0;
	string line;
	ifstream file;
	file.open("input.txt");

	if (file.is_open()) {
		getline(file, line);
		int length = line.length();
		int ones[length] = {0};

		for (int i = 0; i < length; i++) {
			if (line[i] == '1') {
				ones[i] += 1;
			}
		}
		line_count += 1;

		while (getline(file, line)) {
			line_count += 1;

			for (int i = 0; i < length; i++) {
				if (line[i] == '1') {
					ones[i] += 1;
				}
			}
		}

		cout << "line_count: " << line_count << endl;
		cout << "ones: {";
		for (int i = 0; i < length; i++) {
			cout << ones[i] << ", ";
		}
		cout << "}\n";

		string gamma;
		string epsilon;

		for (int i = 0; i < length; i++) {
			if (ones[i] >= (line_count / 2)) {
				gamma += '1';
				epsilon += '0';
			} else {
				gamma += '0';
				epsilon += '1';
			}
		}

		int gamma_decimal = to_decimal(gamma);
		int epsilon_decimal = to_decimal(epsilon);

		cout << "Gamma:   0b" << gamma << " : " << gamma_decimal << "\n"
			<< "Epsilon: 0b" << epsilon << " : " << epsilon_decimal << "\n"
			<< "Product: " << gamma_decimal * epsilon_decimal << endl;
	} else {
		cout << "Unable to open file." << endl;
	}

	return 0;
}

int to_decimal(string& bits) {
	int result = 0;

	for (int i = 0; i < bits.length(); i++) {
		if (bits[i] == '1') {
			// As index increases the power of 2 decreases.
			result += pow(2, bits.length() - i - 1);
		}
	}
	
	return result;
}
