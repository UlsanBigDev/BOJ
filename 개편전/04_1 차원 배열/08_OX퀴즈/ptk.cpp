#include <iostream>
int main()
{
	int count, i;	
	std::cin >> count;
	int answer[101];
	for (i=0; i<count; i++)
	{
		char arr[81];
		std::cin >> arr;
		int index = 0;
		int temp = 1;
		int result = 0;
		while (arr[index] != '\0')
		{
			if (arr[index] == 'O')
			{
				result += temp;
				temp++;
			}
			else 
			{
				temp = 1;
			}
			index++;
		}		
		answer[i] = result;

	}
	for (i=0; i<count; i++)
	{
		std::cout << answer[i] << "\n";
	}
}
