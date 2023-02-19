#include <iostream>
#include <cmath>
#include <vector>
bool isSosu(int n)
{
	int count = 0;
	for (int i = 1; i < n; i++) {
		if (n % i == 0) count++;
	}
	if (count == 1) return true;
	return false;

}
int main()
{	
	int m, n, min = 10000, total = 0;
	bool flag = false;
	std::cin >> m >> n;
	for (int i=m; i<=n; i++)
	{
		if (isSosu(i))
		{
			flag = true;
			total += i;
			if (min > i)
			{
				min = i;
			}
		}
	}
	if (flag)
	{
		std::cout << total << "\n";
		std::cout << min << "\n";
	}
	else
	{
		std::cout << -1 << "\n";
	}
	
}
