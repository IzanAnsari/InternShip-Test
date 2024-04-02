fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let mut start = 0;
    let mut end = chars.len() - 1;
    while start < end {
        if chars[start] != chars[end] {
            return false;
        }
        start += 1;
        end -= 1;
    }
    true
}

fn first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    for (i, &num) in arr.iter().enumerate() {
        if num == target {
            return Some(i);
        }
    }
    None
}

fn shortest_word(s: &str) -> String {
    let mut shortest = String::new();
    let mut shortest_len = usize::MAX;
    let mut current_word = String::new();
    let mut current_len = 0;

    for c in s.chars() {
        if c == ' ' {
            if current_len < shortest_len {
                shortest_len = current_len;
                shortest = current_word.clone();
            }
            current_word.clear();
            current_len = 0;
        } else {
            current_word.push(c);
            current_len += 1;
        }
    }

    if current_len < shortest_len {
        shortest = current_word;
    }

    shortest
}


fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=n / 2 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        (arr[len / 2 - 1] + arr[len / 2]) as f64 / 2.0
    } else {
        arr[len / 2] as f64
    }
}

fn longest_common_prefix(strings: &[&str]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let mut prefix = String::new();
    let bytes = strings[0].as_bytes();
    'outer: for (i, &byte) in bytes.iter().enumerate() {
        for s in &strings[1..] {
            if i >= s.len() || s.as_bytes()[i] != byte {
                break 'outer;
            }
        }
        prefix.push(byte as char);
    }

    prefix
}

fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    if k > 0 && k <= arr.len() {
        let mut sorted_arr = arr.to_vec();
        for i in 0..arr.len() {
            let mut min_idx = i;
            for j in i + 1..arr.len() {
                if sorted_arr[j] < sorted_arr[min_idx] {
                    min_idx = j;
                }
            }
            sorted_arr.swap(i, min_idx);
        }
        Some(sorted_arr[k - 1])
    } else {
        None
    }
}

struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => {
            let left_depth = max_depth(node.left);
            let right_depth = max_depth(node.right);
            1 + left_depth.max(right_depth)
        }
        None => 0,
    }
}

fn reverse_string(s: &str) -> String {
    let mut reversed = String::new();
    for c in s.chars().rev() {
        reversed.push(c);
    }
    reversed
}

fn is_prime_rust(num: u64) -> bool {
    if num <= 1 {
        return false;
    }
    let mut i = 2;
    while i * i <= num {
        if num % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(arr1.len() + arr2.len());
    let (mut i, mut j) = (0, 0);
    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            result.push(arr1[i]);
            i += 1;
        } else {
            result.push(arr2[j]);
            j += 1;
        }
    }
    while i < arr1.len() {
        result.push(arr1[i]);
        i += 1;
    }
    while j < arr2.len() {
        result.push(arr2[j]);
        j += 1;
    }
    result
}

fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_sum = std::i32::MIN;
    let mut current_sum = 0;
    for &num in arr {
        current_sum += num;
        max_sum = max_sum.max(current_sum);
        if current_sum < 0 {
            current_sum = 0;
        }
    }
    max_sum
}

fn main() {
    // Palindrome
    let test_string = "racecar";
    if is_palindrome(test_string) {
        println!("{} is a palindrome.", test_string);
    } else {
        println!("{} is not a palindrome.", test_string);
    }

    // First occurrence
    let sorted_array = vec![1, 2, 3, 4, 5];
    let target_number = 3;
    if let Some(index) = first_occurrence(&sorted_array, target_number) {
        println!("First occurrence of {} is at index {}.", target_number, index);
    } else {
        println!("{} not found in the array.", target_number);
    }

    // Shortest word
    let test_string = "This is a test string";
    let shortest = shortest_word(test_string);
    println!("Shortest word: {}", shortest);

    // Prime check
    let num = 17;
    if is_prime(num) {
        println!("{} is prime.", num);
    } else {
        println!("{} is not prime.", num);
    }

    // Median
    let sorted_array = vec![1, 2, 3, 4, 5];
    let median_val = median(&sorted_array);
    println!("Median of the array: {}", median_val);

    // Longest common prefix
    let test_strings = vec!["flower", "flow", "flight"];
    let common_prefix = longest_common_prefix(&test_strings);
    println!("Longest common prefix: {}", common_prefix);

    // Kth smallest element
    let test_array = vec![4, 2, 7, 1, 5];
    let k = 3;
    if let Some(kth_smallest) = kth_smallest(&test_array, k) {
        println!("The {}th smallest element is: {}", k, kth_smallest);
    } else {
        println!("Invalid input.");
    }

    // Maximum depth of a binary tree
    let root = Some(Box::new(TreeNode {
        val: 3,
        left: Some(Box::new(TreeNode::new(9))),
        right: Some(Box::new(TreeNode {
            val: 20,
            left: Some(Box::new(TreeNode::new(15))),
            right: Some(Box::new(TreeNode::new(7))),
        })),
    }));
    println!("Maximum depth of the tree: {}", max_depth(root));

    // Reverse a string
    let test_string = "hello";
    let reversed_string = reverse_string(test_string);
    println!("Reversed string: {}", reversed_string);

    // Prime check in Rust
    let num = 17;
    if is_prime_rust(num) {
        println!("{} is prime.", num);
    } else {
        println!("{} is not prime.", num);
    }

    // Merge sorted arrays
    let arr1 = vec![1, 3, 5, 7, 9];
    let arr2 = vec![2, 4, 6, 8, 10];
    let merged_array = merge_sorted_arrays(&arr1, &arr2);
    println!("Merged sorted arrays: {:?}", merged_array);

    // Maximum subarray sum
    let arr = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let max_sum = max_subarray_sum(&arr);
    println!("Maximum subarray sum: {}", max_sum);
}
