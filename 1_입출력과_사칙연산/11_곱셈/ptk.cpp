#include <iostream>
int main() 
{
	int a, b;
	std::cin >> a >> b;
	// 3 8 5 
	// 3
	int x = b / 100;
	// 8 // 385 - 300 / 10
	int y = (b - (x * 100)) / 10;
	// 5 // 385 - 300 - 80
	int z = b - x * 100 - y * 10;
	std::cout << z*a << "\n" << y*a << "\n" << x*a << "\n";
	std::cout << a * b;
}
