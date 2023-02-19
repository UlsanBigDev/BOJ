#include <iostream>
int arr[5001];
int SMALL = 3;
int BIG = 5;
int Solution(int n, int x, int y)
{
	if (x * SMALL + y * BIG > n)
	{
		x++;
		y = 0;
	}
	if (x * SMALL + y * BIG > n) return -1;
	if (n - (x * SMALL + y * BIG) == 0) return x + y;	
	return Solution(n, x, y + 1);

}
int main()
{
	int n;
	std::cin >> n;
	std::cout << Solution(n, 0, 0);

}
