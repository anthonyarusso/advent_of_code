#include <bits/stdc++.h>

#define BOARD_SIZE 25

using namespace std;

int draws_to_win(
	   vector<int>& draws,
	   vector<int>& marks,
	   int* board,
	   int row_size,
	   int column_size
	   );

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
		boards.push_back(new int[BOARD_SIZE]);

		// Fill the boards vector.
		while (getline(file, line)) {
			if (line == "") {
				// Start a new board with an empty array.
				boards.push_back(new int[BOARD_SIZE]);
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
		vector<int> marks[boards.size()];
		vector<int> draws_count;
		int d;
		for (int i = 0; i < boards.size(); i++) {
			draws_count.push_back(
					draws_to_win(
						draws,
						marks[i],
						boards[i],
						5,
						5)
					);
		}
		/*
		for (auto board : boards) {
			draws_count.push_back(draws_to_win(draws, board, 5, 5));
		}
		*/
		int index = 0, winning_index = -1;
		cout << "draws_count: ";
		for (auto c : draws_count) {
			cout << c << ", ";
			if (c != -1) {
				winning_index = index;
				break;
			}
			index += 1;
		}
		cout << endl;
		cout << "winning_index: " << winning_index << endl;
		if (winning_index == -1) {
			cout << "No winner found!" << endl;
		} else {
			cout << "Winner found!" << endl;
			cout << "Draws to win: " << draws_count[winning_index] << endl;
			cout << "Winner's Marks: ";
			for (auto m : marks[winning_index]) {
				cout << m << ", ";
			}
			cout << endl;
		}

	} else {
		cout << "Failed to open file." << endl;
	}

	return 0;
}

int draws_to_win(
		vector<int>& draws,
		vector<int>& marks,
		int* board,
		int row_size,
		int column_size
		) {
	int result = -1;
	int board_size = row_size * column_size;
	int rows[row_size] {0};
	int columns[column_size] = {0};
	
	for (int i = 0; i < draws.size(); i++) {
		for (int j = 0; j < board_size; j++) {
			if (draws[i] == board[j]) {
				// Increment our counters for rows and columns.
				rows[j / row_size] += 1;
				columns[j % row_size] += 1;
				// Push index to marks.
				marks.push_back(j);

				if ((rows[j / row_size] == row_size) ||
				(columns[j % row_size] == column_size)) {
					result = i + 1;
					goto result_found;
				}
			}
		}
	}

	result_found:

	return result;
}
