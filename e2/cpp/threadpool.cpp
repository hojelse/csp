#include <iostream>
#include "includes/thread_pool.hpp"

int counter = 0;

void inc()
{ 
    for (int i = 0; i < 100000; i++) 
    { 
        counter++;
    }
}

int main(int argc, char *argv[]) 
{
    
    if (argc != 2) 
        {
        std::cout << "Please input the amount of threads to spawn\n";
        return 1;
        }

    int num_cores = atoi(argv[1]);
    thread_pool pool(num_cores);    
    std::cout << "Running with " << num_cores << " threads" << std::endl;
    
    pool.submit(inc);

    pool.wait_for_tasks();
    std::cout << counter << std::endl;
}