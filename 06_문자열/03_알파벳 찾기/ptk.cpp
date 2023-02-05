#include <iostream>
#include <map>
int main() {	
	std::map<char, int> map;
	
	char str[101];
	std::cin >> str;
	int index = 0;
	while (str[index] != '\0') {
		map.insert({ str[index], index });
		index++;
	}

	for (char i = 'a'; i <= 'z'; i++) {
		map.insert({ i, -1 });
	}
	for (auto iter = map.begin(); iter != map.end(); iter++) {
		std::cout << iter->second << " ";
	}
	
}
