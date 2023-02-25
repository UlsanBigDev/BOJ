#include <iostream>
#include <string>
int main()
{
	int n, x;
	std::string str;
	std::cin >> n;
	x = n / 4;	
	for (int i=0; i<x; i++)
	{
		str += "long ";
	}
	str += "int";
	std::cout << str;
}
