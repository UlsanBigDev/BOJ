#include <iostream>
#include <string>
int main()
{
	std::string x, y, big, small, value = "";
	std::cin >> x >> y;

	big = x.length() >= y.length() ? x.c_str() : y.c_str();
	small = x.length() >= y.length() ? y.c_str() : x.c_str();
	while (small.length() < big.length())
	{
		small = "0" + small;
	}
	// std::cout << "|\t" << big << "|" << "\n";
	// std::cout << "|\t" << small << "|" <<"\n";	 	
	int index = small.length() - 1, carry = 0;
	while (index >= 0)
	{
		
		int temp = (big[index] - '0' + small[index] - '0') + carry;
		if (temp >= 10)
		{
			temp -= 10;
			carry = 1;
		}
		else
		{
			carry = 0;
		}
		// std::cout << "X : " << big[index] - '0' << " | Y : " << small[index] - '0' << "| SUM : " << temp << " | CARRY : " << carry << "\n";

		value = std::to_string(temp) + value;		
		index--;	
	}
	if (carry == 1) value = '1' + value;
	std::cout << value << "\n";
	

}
