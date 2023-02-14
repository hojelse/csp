#include <iostream> 


/* 
Some of you probably know that passing by reference and value are two different things and will behave differently.
If we pass by value, we copy the underlying object and use that within the function scope.
If we pass by reference, then we will operate directly on the argument we pass to it. 
*/

void incrementByOneValue(int j)  // <-- Pass by value!
{
    j = j + 1; 
    std::cout << "I increment by value and I changed x to be " << j << " \n"; 
};

void incrementByOneReference(int &j)  // <--- Pass by reference!
{
    j = j + 1;
    std::cout << "I increment by reference and I changed x to be " << j << " \n";  
}

int main() 
{
    int x = 5; 

    std::cout << "Before calling by value, x is: " << x << " \n"; 
    incrementByOneValue(x);
    std::cout << "After calling by value, x is: " << x << " \n"; 

    std::cout << "\n";

    std::cout << "Before calling by reference, x is: " << x << " \n"; 
    incrementByOneReference(x);
    std::cout << "After calling by reference, x is: " << x << " \n"; 
}


