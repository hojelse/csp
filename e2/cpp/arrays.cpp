#include <iostream>
#include <vector>


using namespace std; // In 90% of all cases, avoid doing this!

// 
std::vector<int> someVector = {1,2,3,4,5}; // <--- has ability to resize it self. Resides on the heap. 
int someArray[5]            = {1,2,3,4,5}; // <--- can not be resized. Resides on the stack. 


int main() 
{
    for(int i = 0; i < someVector.size(); i ++) 
    {
        std::cout << someVector[i] << std::endl; 
    };


    for(int i = 0; i < 5; i ++) 
    {
        std::cout << someArray[i] << std::endl; 
    };
    
    // Note: raw arrays do not have a .size(), because, well.. we declared it and know how big it is.
}

