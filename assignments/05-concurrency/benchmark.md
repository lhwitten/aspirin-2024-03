My program runs O(nlogn) time. Breaking the initial list down requires log(n)
steps while the actual merging process takes O(n) amount of time. This results
in a standard merge sort taking nlog(n) time. Breaking the problem into threads
does not actually reduce the time complexity because the log(n) process is still
happening and the O(n) process occurrs because of the length of the list, not
the number of computations used to achieve it. Using threads effectively reduces
whatever constants multiply the O(n) process.

My process takes a long time because of the many many clones and allocations
necessary to get the program to run. The program runs much faster when there are
less vectors in the overrall vector of vectors being passed around by the
orchestration program. The program with the most threads will always run the
fastest in my merge sort because the number of threads only increases the amount
of task batching that is possible. if only one thread exists then only one task
will be assigned at a time, when 100 threads exist, one hundred threads will be
assigned at a time. If there are less tasks to complete than the number of
threads present then the extra threads will not be helpful. One thing that could
speed up my program would be getting the next batch of tasks ready before trying
to receive the result of the previous batch. I believe my program is slowed down
by the get results function which waits on thread channels in order. 100 is the
fastest number of threads.
