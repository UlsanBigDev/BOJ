#include <iostream>
int main()
{
	int n, m;
	int arr[101] = {0};
	std::cin >> n >> m;
	for (int i=1; i<=n; i++)
	{
		arr[i] = i;
	}
	for (int i=0; i<m; i++)
	{		
		int j, k, temp;
		std::cin >> j >> k;
		for (j; j<=k; j++, k--)
		{
			temp = arr[j];
			arr[j] = arr[k];
			arr[k] = temp;
		}
	}
	for (int i=1; i<=n; i++)
	{
		std::cout<< arr[i] << " ";
	}

}
