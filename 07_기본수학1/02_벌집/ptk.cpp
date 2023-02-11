#include <iostream>

int main() {
	int x, count = 1, value=1;
	std::cin >> x;
	while (value < x) {
		value += 6 * count;
		count++;
	}
	std::cout << count;
}
