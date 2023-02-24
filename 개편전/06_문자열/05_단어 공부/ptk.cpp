#include <iostream>
#include <map>
#include <vector>
int main() {	
	char str[1000001];
	int index = 0;	
	std::cin >> str;			
	std::map<char, int> map;
	while (str[index] != '\0') {		
		if (str[index] > 'Z') {
			str[index] -= 32; // 모든 소문자를 대문자로
		}		
		if (map[str[index]] == 0) {			
			map.erase(str[index]);
			map.insert(std::make_pair(str[index], 1));
		} else {
			int temp = map[str[index]] + 1;			
			map.erase(str[index]);
			map.insert({ str[index], temp });
		}
		index++;
	}	
	int max = 0;
	std::map<int, std::vector<char>> tmap;
	for (auto iter = map.begin(); iter != map.end(); iter++) {		
		if (max < iter->second)
			max = iter->second;
		if (tmap[iter->second].empty()) {
			std::vector<char> temp;
			temp.push_back(iter->first);
			tmap.erase({ iter->second });
			tmap.insert({iter->second, temp});
		} else {
			std::vector<char> temp = tmap[iter->second];
			temp.push_back(iter->first);
			tmap.erase({iter->second});
			tmap.insert({ iter->second, temp });
		}
	}
	if (tmap[max].size() > 1) {
		std::cout << "?" << "\n";
	} else {		
		std::cout << tmap[max][0];
	}
	
}
