#include <bits/stdc++.h>

// Abbreviations
#define fr first
#define sc second

#define MAX_LINE_CHARS 30

using namespace std;

struct Point {
	public:
		int x;
		int y;
};
struct LineSeg {
	public:
		Point first;
		Point second;
};

void parse_coords(string, LineSeg&);
void insert_intersections(set<Point>&, LineSeg&, LineSeg&);
float calc_slope(LineSeg);

int main() {
	vector<LineSeg> line_segments;
	set<Point> intersections;
	string line;
	ifstream file;
	file.open("input.txt");

	if (file.is_open()) {
		// Parse input into line_segments vector.
		while (getline(file, line)) {
			LineSeg ls;
			parse_coords(line, ls);
			line_segments.push_back(ls);
		}
		// line_segments is now full.
		 
		// Loop through each line_seg
		// Compare each line to every other line *below* it in the file.
		for (size_t i = 0; i < line_segments.size(); i++) {
			for (size_t j = i; j < line_segments.size(); j++) {
				insert_intersections(intersections, line_segments[i], line_segments[j]);
			}
		}
		// For each pair of line segments being compared determine the
		// coordinates at which they overlap. Mark these coordinates into
		// a hashmap (to avoid double counting).
		//                                 
		// Yes we know this will have BigO(n^2), we just don't care.
	} else {
		cout << "Failed to open file!" << endl;
	}
	return 0;
}

void parse_coords (string line, LineSeg& line_segment) {
	char* token;
	char cstr[MAX_LINE_CHARS];
	strcpy(cstr, line.c_str());

	token = strtok(cstr, ",");
	int x1 = atoi(token);
	token = strtok(NULL, " -> ");
	int y1 = atoi(token);
	token = strtok(NULL, ",");
	string tmp = token;
	tmp.erase(0, 3);
	int x2 = stoi(tmp);
	// cout << "x2_int: " << x2 << endl;
	token = strtok(NULL, ",");
	int y2 = atoi(token);
	Point coord1 = {x1, y1};
	Point coord2 = {x2, y2};

	line_segment = {coord1, coord2};
}

void insert_intersections (set<Point>& set, LineSeg& seg1, LineSeg& seg2) {
	// Check NON-DIAGONALS ONLY!
	/*
	if (((seg1.fr.x != seg1.sc.x) || seg1.fr.y != seg1.sc.y)
			|| ((seg2.fr.x != seg2.sc.x) || (seg2.fr.y != seg2.sc.y))) {
		// If either of the two line segments are diagonal (no matching Xs or Ys.
		goto skip;
	}
	*/
	// Check to see if the line segments are anywhere near each other first.
	// I.e. check for overlap in the X and Y ranges of the two segments.
	float slope1, slope2;
	slope1 = calc_slope(seg1);
	slope2 = calc_slope(seg2);

	if (slope1 == slope2) {
		// Line segments are parallel.
		// If their y-intercepts are also the same then they overlap.
		// b = observed_y - no_offset_y;
		int b1 = seg1.fr.y - (slope1 * seg1.fr.x);
		int b2 = seg2.fr.y - (slope2 * seg2.fr.x);
		if (b1 == b2) {
			// The two line segments overlap.
		}
	} else {
		// Line segments are non-parallel meaning there is only one
		// intersection point.
	}
skip:
	0;
}

float calc_slope (LineSeg ls) {
	if (ls.sc.x - ls.fr.x == 0) {
		return numeric_limits<float>::infinity();
	} else {
		int delta_y = ls.sc.y - ls.fr.y;
		int delta_x = ls.sc.x - ls.fr.x;
		return abs(delta_y / delta_x);
	}
}
