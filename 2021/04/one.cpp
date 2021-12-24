#include <bits/stdc++.h>

#define BOARD_SIZE 25
#define MAX_BOARD_CHARS 30

using namespace std;

int draws_to_win (
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
		// Read the drawn numbers from file.
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

		// Read boards from file.
		char num_cstr[MAX_BOARD_CHARS];
		int board_index = 0;
	
		// Fill the boards vector.
		while (getline(file, line)) {
			if (line == "") {
				// Start a new board with a pointer to an empty array.
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
		vector<vector<int>> marks;
		vector<int> draws_count;
		int d;
		for (int i = 0; i < boards.size(); i++) {
			vector<int> ms;
			// Push an empty vector to marks to be filled
			// by the draws_to_win function.
			marks.push_back(ms);
			draws_count.push_back(
					draws_to_win(
						draws,
						marks[i],
						boards[i],
						5,
						5)
					);
		}
		int index = 0, winning_index = -1;
		int tracker = INT_MAX;
		for (auto c : draws_count) {
			if (c < tracker) {
				tracker = c;
				winning_index = index;
			}
			index += 1;
		}
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
			int last_draw = boards[winning_index][*(marks[winning_index].end() - 1)];
			cout << "Winner's last draw: winner[" << *(marks[winning_index].end() - 1)
				<< "] = "
				<< last_draw
				<< "\ndraws[" << draws_count[winning_index]
				<< " - 1] == " << draws[draws_count[winning_index] - 1]
				<< endl;
			// Sum all the unmarked numbers from the winning board.
			// To do this, set all mark numbers to 0 and sum all numbers.
			for (auto m : marks[winning_index]) {
				boards[winning_index][m] = 0;	
			}
			int sum = 0;
			for (int i = 0; i < BOARD_SIZE; i++) {
				sum += boards[winning_index][i];
			}
			cout << "Sum: " << sum << " Last Draw: " << last_draw
				<< " Product: " << sum * last_draw << endl;
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
				// cout << "i: " << i << ", j: " << j << ", val: " << draws[i] << endl;
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
