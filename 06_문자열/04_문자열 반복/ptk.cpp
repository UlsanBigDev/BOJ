#include <iostream>
#include <vector>
int main() {
	int count, i;
	std::cin >> count;
	for (i = 0; i < count; i++) {
		int repeat, index = 0;
		char str[21];
		std::vector<char> vector;
		std::cin >> repeat >> str;		
		while (str[index] != '\0') {
			for (int j = 0; j < repeat; j++) {
				vector.push_back(str[index]);
			}
			index++;
		}
		for (int j = 0; j < vector.size(); j++) {
			std::cout << vector[j];
		}
		std::cout << "\n";
	}
}
