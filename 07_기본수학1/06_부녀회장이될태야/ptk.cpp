#include <iostream>
int arr[100][14];
int Solution(int k, int n)
{
	if (k == 0 && n == 0 || k < 0) return 1;
	if (arr[k][n] != 0) return arr[k][n];
	int temp = 0;
	int i = n;
	int j = k - 1;
	while (i > 0)
	{
		temp += Solution(j, i);
		i--;
	}	
	return arr[k][n] = temp;
}
int main()
{	
	int count;
	std::cin >> count;
	for (int i=0; i<count; i++)
	{
		int k, n;
		std::cin >> k >> n;
		std::cout << Solution(k, n) << "\n";
	}
}
