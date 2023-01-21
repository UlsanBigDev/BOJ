
int main()
{
    int x;
    std::cin >> x;
    for (int i=x; i>0; i--)
    {
        for (int j=1; j<=x; j++)
        {            
            if (j >= i)
            {
                std::cout << "*";
            }
            else
            {
                std::cout << " ";
            }
            
        }
        std::cout << "\n";
    }
    
}
