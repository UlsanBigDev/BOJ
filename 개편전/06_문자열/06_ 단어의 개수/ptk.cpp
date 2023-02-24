#include <iostream>
#include <string>
int main() {
	std::string str;
	std::getline(std::cin, str);
	bool isWord = false;
	int index = 0, total = 0;
	while (str[index] != '\0') {
		if (str[index] != ' ') {
			total++;
			int j = index + 1;
			while (str[j] != ' ' && str[j] != '\0') {
				j++;
			}		
			// std::cout << "[" << j << "]" << "\n";
			index = j - 1;
		}
		index++;
	}
	std::cout << total;
}
