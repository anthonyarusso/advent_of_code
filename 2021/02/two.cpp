#include <bits/stdc++.h>

using namespace std;

int main() {
	int horizontal = 0, depth = 0, aim = 0;
	string line, command;
	int distance;
	int space_index;
	ifstream file;
	file.open("input.txt");

	if (file.is_open()) {
		while (getline(file, line)) {
			space_index = line.find(' ', 0);
			command = line.substr(0, space_index);
			distance = stoi(line.substr(space_index + 1, line.length()));

			if (command == "forward") {
				horizontal += distance;
				depth += aim * distance;
			} else if (command == "up") {
				aim -= distance;
			} else if (command == "down") {
				aim += distance;
			} else {
				cout << "Command not recognized!" << endl;
				break;
			}
		}

		cout << "Horizontal: " << horizontal << " Depth: " << depth << endl;
		cout << "Product: " << horizontal * depth << endl;

		file.close();
	} else {
		cout << "Unable to open file." << endl;
	}

	return 0;
}
