#include <iostream>
#include <map>
std::map<char, int> dial;
void init() {	
	dial.insert({ 'A', 3 });
	dial.insert({ 'B', 3 });
	dial.insert({ 'C', 3 });
	dial.insert({ 'D', 4 });
	dial.insert({ 'E', 4 });
	dial.insert({ 'F', 4 });
	dial.insert({ 'G', 5 });
	dial.insert({ 'H', 5 });
	dial.insert({ 'I', 5 });
	dial.insert({ 'J', 6 });
	dial.insert({ 'K', 6 });
	dial.insert({ 'L', 6 });
	dial.insert({ 'M', 7 });
	dial.insert({ 'N', 7 });
	dial.insert({ 'O', 7 });
	dial.insert({ 'P', 8 });
	dial.insert({ 'Q', 8 });
	dial.insert({ 'R', 8 });
	dial.insert({ 'S', 8 });
	dial.insert({ 'T', 9 });
	dial.insert({ 'U', 9 });
	dial.insert({ 'V', 9 });
	dial.insert({ 'W', 10 });
	dial.insert({ 'X', 10 });
	dial.insert({ 'Y', 10 });
	dial.insert({ 'Z', 10 });
}

int main() {
	init();
	char str[16];
	std::cin >> str;
	int index = 0 ,sum = 0;
	while (str[index] != '\0') {
		sum += dial[str[index]];
		index++;
	}
	std::cout << sum;
}
