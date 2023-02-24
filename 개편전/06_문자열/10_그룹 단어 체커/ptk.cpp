#include <iostream>
#include <vector>
int main() {
	int count, i, result = 0;
	std::cin >> count;
	std::vector<std::vector<char>> vv;
	for (i = 0; i < count; i++) {
		char str[101];
		int index = 0;
		std::cin >> str;
		std::vector<char> vector;
		while (str[index] != '\0') {			
			if (vector.empty()) {
				vector.push_back(str[index]);
			} else {
				if (vector.back() != str[index]) {
					vector.push_back(str[index]);
				}				
			}
			index++;
		}
		vv.push_back(vector);
	}
	for (int i = 0; i < vv.size(); i++) {		
		std::vector<char> word = vv[i];
		bool flag = true;
		for (int j = 0; j < word.size(); j++) {
			int index = j + 1;
			while (index < word.size()) {
				if (word[j] == word[index])
					flag = false;
				index++;
			}
		}
		if (flag) {
			result++;
		}
	}
	std::cout << result;
}
