#include <iostream>

int main() {
	int a, b, c, x, count = 0;
	std::cin >> a >> b >> c;
	if (b >= c) {
		std::cout << -1;
		return 0;
	}
	std::cout << a / (c - b) + 1;
	
}
