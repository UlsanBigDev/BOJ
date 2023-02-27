#include <iostream>
#include <string>
int main()
{
	int arr[100] = { 0, };
	int n, m;
	std::cin >> n >> m;

	for (int c=0; c<m; c++)
	{
		int i, j, k;
		std::cin >> i >> j >> k;
		for (i; i<=j; i++)
		{
			arr[i] = k;
		}
	}

	for (int c=1; c<=n; c++)
	{
		std::cout << arr[c] << " ";
	}

}
