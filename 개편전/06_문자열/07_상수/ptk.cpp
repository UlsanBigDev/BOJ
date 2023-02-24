#include <iostream>
int main() {
	int x, y, max;
	std::cin >> x >> y;
	if (x % 10 > y % 10) {
		max = x;
	} else if (x % 10 < y % 10) {
		max = y;
	} else {		
		if (x % 100 / 10 > y % 100 / 10) {
			max = x;
		} else if (x % 100 / 10 < y % 100 / 10) {
			max = y;
		} else {
			if (x / 100 > y / 100) {
				max = x;
			} else {
				max = y;
			}
		}
	}
	std::cout << max % 10 << max % 100 / 10 << max / 100;

}
