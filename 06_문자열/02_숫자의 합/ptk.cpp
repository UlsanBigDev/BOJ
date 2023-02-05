#include <iostream>
int main() {
	int count, sum, index=0;
	std::cin >> count;
	char value[101];
	std::cin >> value;
	sum = 0;	
	while (value[index] != '\0') {
		sum += value[index] - '0';
		index++;
	}
	std::cout << sum;
	

}
