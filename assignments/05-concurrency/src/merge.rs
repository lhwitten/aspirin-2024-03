//Implementing merge sort

//merge 2 sorted arrays

use crate::thread_pool::*;

#[derive(Clone)]
pub struct MergeArgs {
    arr1: Vec<i64>,
    arr2: Vec<i64>,
}
pub fn merge(args: MergeArgs) -> Vec<i64> {
    let mut output: Vec<i64> = Vec::with_capacity(args.arr1.len() + args.arr2.len());
    let (mut i1, mut i2) = (0, 0);

    while i1 < args.arr1.len() && i2 < args.arr2.len() {
        if args.arr1[i1] <= args.arr2[i2] {
            output.push(args.arr1[i1]);
            i1 += 1;
        } else {
            output.push(args.arr2[i2]);
            i2 += 1;
        }
    }

    // Append remaining elements, if any
    if i1 < args.arr1.len() {
        output.extend_from_slice(&args.arr1[i1..]);
    }
    if i2 < args.arr2.len() {
        output.extend_from_slice(&args.arr2[i2..]);
    }

    output
}
// pub struct Task<T_Args, T_Outs> {
//     func: fn(T_Args) -> T_Outs,
//     inputs: T_Args,
//     outputs: T_Outs,
//     end_thread: bool,
// }
pub fn merge_sort(arr: Vec<i64>, num_threads: usize) -> Vec<i64> {
    let mut pool: ThreadPool<Task<MergeArgs, Vec<i64>>> = ThreadPool::new(num_threads);
    let original_len = arr.len();

    // Initialize `mergables` with each element as a separate vector
    let mut mergables: Vec<Vec<i64>> = arr.into_iter().map(|x| vec![x]).collect();

    loop {
        let mergables_len = mergables.len();
        println!("mergables len: {}", mergables_len);

        // If there's only one sorted array left, we're done
        if mergables_len == 1 {
            break;
        }

        let mut tasklist: Vec<Task<MergeArgs, Vec<i64>>> = Vec::with_capacity((mergables_len / 2));
        let mut next_mergables: Vec<Vec<i64>> = Vec::with_capacity((mergables_len / 2));

        // Handle odd number of arrays
        let odd_vec = if mergables_len % 2 == 1 {
            Some(mergables.pop().expect("odd vec empty"))
        } else {
            None
        };

        // Create tasks for merging pairs of vectors
        for _ in 0..(mergables.len() / 2) {
            let arr1 = mergables.pop().expect("vec empty");
            let arr2 = mergables.pop().expect("vec empty");
            tasklist.push(Task {
                func: merge,
                inputs: MergeArgs { arr1, arr2 },
                outputs: Vec::with_capacity(std::cmp::max(original_len + 2 - mergables_len, 1)),
                end_thread: false,
            });
        }
        let mut result_vec: Vec<Task<MergeArgs, Vec<i64>>> = Vec::with_capacity(mergables_len / 2);
        let num_threads = pool.num_threads; // Number of threads available in the pool

        // Execute tasks in chunks using the thread pool
        let mut number = 0;
        while !tasklist.is_empty() {
            // Create a sublist of tasks equal to the number of threads or fewer if not enough tasks remain

            number += num_threads.min(tasklist.len());
            let sublist: Vec<Task<MergeArgs, Vec<i64>>> =
                tasklist.drain(..num_threads.min(tasklist.len())).collect();

            // Execute the current sublist of tasks
            pool.execute(sublist);
            if number % 1000 > 990 {
                println!("progress: {}", number);
            }

            // Collect results from the current batch of tasks
            result_vec.extend(pool.get_results());
        }

        // Add merged results to `next_mergables`
        for task in result_vec {
            next_mergables.push(task.outputs);
        }

        // If there's an odd vector, add it to the next round
        if let Some(odd) = odd_vec {
            next_mergables.push(odd);
        }

        // Update `mergables` for the next iteration
        mergables = next_mergables;
    }
    let arr1: Vec<i64> = vec![0, 1];
    let arr2: Vec<i64> = vec![0, 1];
    let pool_ender = Task {
        func: merge,
        inputs: MergeArgs { arr1, arr2 },
        outputs: Vec::new(),
        end_thread: true,
    };
    pool.end_pool(pool_ender);

    // Return the final sorted array

    let final_list = mergables.pop().expect("failed to return merged list");
    //println!("final list: {:?}", final_list);
    final_list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_two_sorted_arrays() {
        let arr1 = vec![1, 3, 5, 7];
        let arr2 = vec![2, 4, 6, 8];
        let args = MergeArgs { arr1, arr2 };

        let result = merge(args);
        assert_eq!(
            result,
            vec![1, 2, 3, 4, 5, 6, 7, 8],
            "Merging two sorted arrays failed"
        );
    }

    #[test]
    fn test_merge_with_empty_array() {
        let arr1 = vec![1, 2, 3];
        let arr2 = vec![];
        let args = MergeArgs { arr1, arr2 };

        let result = merge(args);
        assert_eq!(result, vec![1, 2, 3], "Merging with an empty array failed");
    }

    #[test]
    fn test_merge_sort_empty_array() {
        let arr: Vec<i64> = vec![];
        let sorted_arr = merge_sort(arr, 2);
        assert_eq!(
            sorted_arr,
            vec![],
            "Sorting an empty array should return an empty array"
        );
    }

    #[test]
    fn test_merge_sort_single_element() {
        let arr = vec![42];
        let sorted_arr = merge_sort(arr, 2);
        assert_eq!(
            sorted_arr,
            vec![42],
            "Sorting a single-element array should return the same array"
        );
    }

    #[test]
    fn test_merge_sort_already_sorted() {
        let arr = vec![1, 2, 3, 4, 5];
        let sorted_arr = merge_sort(arr.clone(), 2);
        assert_eq!(
            sorted_arr, arr,
            "Sorting an already sorted array should return the same array"
        );
    }
    #[test]
    fn test_merge_sort_odd_length_array() {
        let arr: Vec<i64> = vec![9, 3, 5, 1, 7];
        let sorted_arr: Vec<i64> = merge_sort(arr.clone(), 2);
        assert_eq!(
            sorted_arr,
            vec![1, 3, 5, 7, 9],
            "Sorting an odd-length array failed"
        );
    }

    #[test]
    fn test_merge_sort_unsorted_array() {
        let arr: Vec<i64> = vec![5, 3, 8, 6, 2, 7, 4, 1];
        let sorted_arr = merge_sort(arr.clone(), 2);
        assert_eq!(
            sorted_arr,
            vec![1, 2, 3, 4, 5, 6, 7, 8],
            "Sorting an unsorted array failed"
        );
    }

    #[test]
    fn test_merge_sort_large_array() {
        let arr: Vec<i64> = (0..1_000).rev().collect(); // Large array in reverse order
        let sorted_arr = merge_sort(arr.clone(), 4); // Testing with multiple threads
        assert_eq!(
            sorted_arr,
            (0..1_000).collect::<Vec<i64>>(),
            "Sorting a large array failed"
        );
    }
}
