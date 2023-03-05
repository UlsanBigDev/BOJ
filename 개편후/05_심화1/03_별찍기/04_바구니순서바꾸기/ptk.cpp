#include <iostream>
int arr[101];
void Solution(int begin, int mid, int end)
{
	int temp[101] = { 0, };
	int index = begin;
	for (int i=mid; i<=end; i++, index++)
	{
		temp[index] = arr[i];
	}
	for (int i=begin; i<mid; i++, index++)
	{
		temp[index] = arr[i];
	}
	for (int i=begin; i<=end; i++)
	{
		arr[i] = temp[i];		
	}

}
int main()
{
	int n, m;
	std::cin >> n >> m;
	for (int i=1; i<=n; i++)
	{
		arr[i] = i;
	}
	for (int a=0; a<m; a++)
	{
		int i, j, k;
		std::cin >> i >> j >> k;
		Solution(i, k, j);
	}
	for (int a=1; a<=n; a++)
	{
		std::cout << arr[a] << " ";
	}
}
