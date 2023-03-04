#include <iostream>
#include <string>
int main()
{
	std::string str;
	
	do
	{
		std::getline(std::cin, str);
		std::cout << str << "\n";
	} while (str != "");
}
