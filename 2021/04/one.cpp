#include <bits/stdc++.h>

#define BOARD_SIZE 25;

using namespace std;

int main() {
	vector<int> draws;
	vector<int*> boards;
	string line;
	ifstream file;
	file.open("input.txt");

	if (file.is_open()) {
		getline(file, line);
		char* token;
		// Convert line to char* for use in strtok.
		char cstr[line.length() + 1];
		strcpy(cstr, line.c_str());

		token = strtok(cstr, ",");

		while (token != NULL) {
			draws.push_back(atoi(token));
			token = strtok(NULL, ",");
		}

		char num_cstr[30];
		int board_index = 0;
		// Push a pointer to an empty int array to the boards vector.
		boards.push_back(new int[25]);

		// Fill the boards vector.
		while (getline(file, line)) {
			if (line == "") {
				// Start a new board with an empty array.
				boards.push_back(new int[25]);
				board_index = 0;
			} else {
				// Push numbers to current board.
				strcpy(num_cstr, line.c_str());
				token = strtok(num_cstr, " ");

				while (token != NULL) {
					// Current board is at the end of the boards vector.
					// I.e. boards.at(boards.size() - 1) is the current board.
					boards.at(boards.size() - 1)[board_index] = atoi(token);
					board_index += 1;
					token = strtok(NULL, " ");
				}
			}
		}
		// Boards vector is now filled.
	} else {
		cout << "Failed to open file." << endl;
	}

	return 0;
}
