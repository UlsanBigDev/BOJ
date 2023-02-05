#include <iostream>
#include <vector>

int answer[10001] = { 0, };
int Solution(int n)
{	
	int result = n;
	std::vector<int> vector;
	while (n) {		
		vector.push_back(n % 10);
		n /= 10;
	}
	for (int i = 0; i < vector.size(); i++) {
		result += vector[i];
	}	
	answer[result] = 1;
	return result;
}

int main()
{	
	for (int i=1; i<=10000; i++) {		
		Solution(i);
	}
	
	for (int i = 1; i <= 10000; i++) {
		if (answer[i] == 0) {
			std::cout << i << "\n";
		}
	}
	
}

