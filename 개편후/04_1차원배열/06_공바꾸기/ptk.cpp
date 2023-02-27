#include <iostream>
#include <string>
int main()
{
	int n, m;
	int arr[1001] = {0};
	std::cin >> n >> m;
	for (int i=1; i<=n; i++)
	{
		arr[i] = i;
	}
	for (int i=0; i<m; i++)
	{
		int j, k, temp;
		std::cin >> j >> k;
		temp = arr[j];
		arr[j] = arr[k];
		arr[k] = temp;
	}
	for (int i=1; i<=n; i++)
	{
		std::cout<< arr[i] << " ";
	}

}
