#include <iostream>  // Only for I/O 

#include <thread> // posix threads
#include <mutex>  // mutexes


// We declare a mutex to lock shared resource
std::mutex mut;

// Declare a counter 
std::atomic_int counter = 0; 


void inc()
{ 
    for (int i = 0; i < 1000000; i++) 
    { 
        counter++; // increment counter 
    }
}

int main(int argc, char *argv[]) 
{
    
    if (argc != 2) 
        {
        std::cout << "Please input the amount of threads to spawn\n";
        return 1;
        }

    int num_threads = atoi(argv[1]);
    
    std::cout << "Created " << num_threads << " threads" << std::endl; 

    std::thread threads[num_threads];
    
    for (int i = 0; i < num_threads; ++i)
        {
            threads[i] = std::thread(inc); 
        };
    
    for (int i = 0; i < num_threads; i++)
        { 
            threads[i].join();
        };
    
    std::cout << counter << std::endl; 
}