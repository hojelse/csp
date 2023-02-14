#include <stdio.h>
#include <pthread.h>
#include <stdatomic.h>
#include <stdlib.h>

atomic_int counter = 0;

void *inc()  
// Arguments must match when creating the pthreads. By docs: void *(*start_routine) (void *)
{
    int i;
    for (i = 0; i < 1000000; i++) 
    {
        counter++;
        // NOTE: More C-like to utilize atomic_fetch_add (&obj, incrementBy), or (&counter, 1) here.
    }
};

int main(int argc, char *argv[])
{
    if (argc != 2) 
    {
        printf("Please enter the number of threads\n");
        return 1;
    }

    
    int num_threads = atoi(argv[1]);
    
    printf("Running with %d threads\n", num_threads);

    pthread_t threads[num_threads];

    for (int i = 0; i < num_threads; i++)
    {
        pthread_create(&threads[i], NULL, inc, NULL);
        // Note that NULL is the pthread policy, we wont go into that, but you can do fancy stuff with it
    }

    for (int i = 0; i < num_threads; i++)
    {
        pthread_join(threads[i], NULL);
        // 
    }

    printf("The counter is now %d\n", counter);
    return 0;
}