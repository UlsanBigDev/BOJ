#include <iostream>
int main()
{
	int a;
	std::cin >> a;
	for (int i=0; i<a; i++)
	{
		for (int j=i+1; j<a; j++)
		{
			std::cout << " ";
		}
		for (int j=i; j>=0; j--)
		{
			std::cout << "*";
		}
		for (int j=0; j<i; j++)
		{
			std::cout << "*";
		}
		/*
		for (int j=i+1; j<a; j++)
		{
			std::cout << "\s";
		}
		*/
		std::cout << "\n";
	}
	for (int i=a-1; i>0; i--)
	{
		for (int j=i; j<a; j++)
		{
			std::cout << " ";
		}
		for (int j=i; j>0; j--)
		{
			std::cout << "*";
		}
		for (int j=i-1; j>0; j--)
		{
			std::cout << "*";
		}
		/*
		for (int j=i; j<a; j++)
		{
			std::cout << "\s";
		}
		*/
		std::cout << "\n";
	}
}
