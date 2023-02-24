#include <iostream>
#include <vector>
bool Solution(int n) {
	std::vector<int> vector;
	while (n) {
		vector.push_back(n % 10);
		n /= 10;
	}	
	if (vector.size() == 1)
		return true;
	int i, d;
	d = vector[1] - vector[0];
	for (i = 0; i < vector.size() - 1; i++) {
		if (vector[i+1] - vector[i] != d) {
			return false;
		}
	}
	return true;
}

int main() {
	int count, i, total;
	std::cin >> count;
	total = 0;
	for (i = 1; i <= count; i++) {
		if (Solution(i)) {
			total++;
		}
	}
	std::cout << total;
}
