#include <iostream>
#include <cmath>
int main()
{
	int a, b, v;
	std::cin >> a >> b >> v;
	long long temp = std::ceil((double) (v - b) / (double) (a - b));
	
	std::cout << temp;
}
