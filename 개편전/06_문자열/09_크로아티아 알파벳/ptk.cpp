#include <iostream>
#include <vector>
#include <string>
// ljes=njak
int main() {
	std::vector<std::string> vector = { "c=", "c-", "dz=", "d-", "lj", "nj", "s=", "z=" };
	std::string str;
	std::cin >> str;
	int total = 0;
	for (int i = 0; i < str.size(); i++) {
		for (int j = 0; j < vector.size(); j++) {
			if (i + vector[j].size() > str.size())
				continue;
			if (str.substr(i, vector[j].size()) == vector[j]) {				
				i = i + vector[j].size() - 1;
				break;
			}
		}
		total++;
	}
	std::cout << total;

}
