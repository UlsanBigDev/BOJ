#include <iostream>
#include <string>
#include <cmath>
int main()
{	
	std::string str;
	std::cin >> str;
	int mid = std::ceil(str.length() / (float) 2);
	int l = mid;
	int r = str.length() % 2 == 0 ? l + 1 : l;
	bool result = true;
	for (l,r; l>=0 && r <= str.length(); l--, r++)
	{
		if (str[l - 1] != str[r - 1])
		{
			result = false;
			break;
		}		
	}
	std::cout << result;
}
