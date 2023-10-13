// // // // // // // // // // // // // // // // // //         // Question 1

// // // // // // // // // // // // // // // // // // // fn main(){
// // // // // // // // // // // // // // // // // // //     println!("Hello , Rust")
// // // // // // // // // // // // // // // // // // // }


// // // // // // // // // // // // // // // // // //     // Question 2

// // // // // // // // // // // // // // // // // // fn main(){
// // // // // // // // // // // // // // // // // //     let mut line = String::new();
// // // // // // // // // // // // // // // // // //     println!("Enter Your  Name");
// // // // // // // // // // // // // // // // // //     let _b1 = std::io::stdin().read_line(&mut line).unwrap();
// // // // // // // // // // // // // // // // // //     println!("Welcome {}", line);
// // // // // // // // // // // // // // // // // // }


// // // // // // // // // // // // // // // // // //         // Question 3 ;


// // // // // // // // // // // // // // // // // use std::io;

// // // // // // // // // // // // // // // // // fn main() {
// // // // // // // // // // // // // // // // //     println!("Temperature Converter");
   

   
// // // // // // // // // // // // // // // // //     println!("Enter the temperature value: ");
// // // // // // // // // // // // // // // // //     let mut input = String::new();
// // // // // // // // // // // // // // // // //     io::stdin().read_line(&mut input).expect("Failed to read line");
    

// // // // // // // // // // // // // // // // //     let temperature: f64 = match input.trim().parse() {
// // // // // // // // // // // // // // // // //         Ok(num) => num,
// // // // // // // // // // // // // // // // //         Err(_) => {
// // // // // // // // // // // // // // // // //             println!("Invalid input. Please enter a valid number.");
// // // // // // // // // // // // // // // // //             return;
// // // // // // // // // // // // // // // // //         }
// // // // // // // // // // // // // // // // //     };

   
// // // // // // // // // // // // // // // // //     println!("Enter the temperature scale (C or F): ");
// // // // // // // // // // // // // // // // //     let mut scale = String::new();
// // // // // // // // // // // // // // // // //     io::stdin().read_line(&mut scale).expect("Failed to read line");
    
// // // // // // // // // // // // // // // // //     // Convert the temperature based on the scale
// // // // // // // // // // // // // // // // //     let converted_temperature = match scale.trim().to_lowercase().as_str() {
// // // // // // // // // // // // // // // // //         "c" => (temperature * 9.0/5.0) + 32.0,
// // // // // // // // // // // // // // // // //         "f" => (temperature - 32.0) * 5.0/9.0,
// // // // // // // // // // // // // // // // //         _ => {
// // // // // // // // // // // // // // // // //             println!("Invalid scale. Please enter 'C' or 'F'.");
// // // // // // // // // // // // // // // // //             return;
// // // // // // // // // // // // // // // // //         }
// // // // // // // // // // // // // // // // //     };

    
// // // // // // // // // // // // // // // // //     println!("Converted temperature: {} {}", converted_temperature, scale.trim());
// // // // // // // // // // // // // // // // // }



// // // // // // // // // // // // // // // // // //         Question 4


// // // // // // // // // // // // // // // // // fn factorial(n: i32) -> i32  {
// // // // // // // // // // // // // // // // //     if n == 0 {
// // // // // // // // // // // // // // // // //         1
// // // // // // // // // // // // // // // // //     } else {
// // // // // // // // // // // // // // // // //         n * factorial(n - 1) 
// // // // // // // // // // // // // // // // //     }
// // // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // // // fn main() {
// // // // // // // // // // // // // // // // //     let n = 9; 
// // // // // // // // // // // // // // // // //     let result = factorial(n);
// // // // // // // // // // // // // // // // //     println!("Factorial of {} is {}", n, result);
// // // // // // // // // // // // // // // // // }


// // // // // // // // // // // // // // // // //             Question 5 


// // // // // // // // // // // // // // // // fn main(){
// // // // // // // // // // // // // // // // let int = 7;


// // // // // // // // // // // // // // // // if int % 2 == 0 {
// // // // // // // // // // // // // // // //     print!("the number is even")

// // // // // // // // // // // // // // // // }

// // // // // // // // // // // // // // // // else {
// // // // // // // // // // // // // // // //     println!("the number is odd")
// // // // // // // // // // // // // // // // }


// // // // // // // // // // // // // // // // }


// // // // // // // // // // // // // // // //         Question 6



// // // // // // // // // // // // // // // fn calculate_sum(n: u32) -> u32 {
// // // // // // // // // // // // // // //     // Calculate the sum using the arithmetic series formula
// // // // // // // // // // // // // // //     n * (n + 1) / 2
// // // // // // // // // // // // // // // }


// // // // // // // // // // // // // // // fn main(){
// // // // // // // // // // // // // // //     let n = 4;

// // // // // // // // // // // // // // //     println!("{}", calculate_sum(n));
// // // // // // // // // // // // // // // }




// // // // // // // // // // // // // // //             Question 7


// // // // // // // // // // // // // // fn main() {
// // // // // // // // // // // // // //     for num in 1..=100 {


// // // // // // // // // // // // // //         if num % 3 == 0 && num % 5 == 0 {
// // // // // // // // // // // // // //             println!("FizzBuzz");


// // // // // // // // // // // // // //         } 
        
// // // // // // // // // // // // // //         else if num % 3 == 0 {
// // // // // // // // // // // // // //             println!("Fizz");

// // // // // // // // // // // // // //         }
        
        
// // // // // // // // // // // // // //          else if num % 5 == 0 {
         
         
// // // // // // // // // // // // // //          println!("Buzz");


// // // // // // // // // // // // // //         } 
        
// // // // // // // // // // // // // //         else {
// // // // // // // // // // // // // //             println!("{}", num);
// // // // // // // // // // // // // //         }
// // // // // // // // // // // // // //     }
// // // // // // // // // // // // // // }



// // // // // // // // // // // // // //         Question 8


// // // // // // // // // // // // // fn is_palindrome(input: &str) -> bool {
// // // // // // // // // // // // //     let input = input.chars().collect::<Vec<char>>();
// // // // // // // // // // // // //     let len = input.len();

// // // // // // // // // // // // //     for i in 0..len / 2 {

// // // // // // // // // // // // //         if input[i] != input[len - i - 1] {
// // // // // // // // // // // // //             return false;
// // // // // // // // // // // // //         }
// // // // // // // // // // // // //     }

// // // // // // // // // // // // //     true
// // // // // // // // // // // // // }

// // // // // // // // // // // // // fn main() {
// // // // // // // // // // // // //     let test_string = "level"; // Change this to the string you want to check
// // // // // // // // // // // // //     if is_palindrome(test_string) {
// // // // // // // // // // // // //         println!("'{}' is a palindrome.", test_string);
// // // // // // // // // // // // //     } else {
// // // // // // // // // // // // //         println!("'{}' is not a palindrome.", test_string);
// // // // // // // // // // // // //     }
// // // // // // // // // // // // // }




// // // // // // // // // // // // //         Question 9






// // // // // // // // // // // // extern crate rand;

// // // // // // // // // // // // use std::io;
// // // // // // // // // // // // use rand::Rng;

// // // // // // // // // // // // fn main() {
    

    
// // // // // // // // // // // //     let secret_number = rand::thread_rng().gen_range(1..=100);

// // // // // // // // // // // //     println!("you've selected a random number between 1 and 100. Try to guess it!");

// // // // // // // // // // // //     let mut attempts = 0;

// // // // // // // // // // // //     loop {
// // // // // // // // // // // //         println!("Please enter your guess: ");
// // // // // // // // // // // //         let mut guess = String::new();

// // // // // // // // // // // //         io::stdin()
// // // // // // // // // // // //             .read_line(&mut guess)
// // // // // // // // // // // //             .expect("Failed to read line");

// // // // // // // // // // // //         let guess: u32 = match guess.trim().parse() {
// // // // // // // // // // // //             Ok(num) => num,
// // // // // // // // // // // //             Err(_) => {
// // // // // // // // // // // //                 println!("Invalid input. Please enter a valid number.");
// // // // // // // // // // // //                 break;;
// // // // // // // // // // // //             }
// // // // // // // // // // // //         };

// // // // // // // // // // // //         attempts += 1;

// // // // // // // // // // // //         match guess.cmp(&secret_number) {
// // // // // // // // // // // //             std::cmp::Ordering::Less => println!("Too low!"),
// // // // // // // // // // // //             std::cmp::Ordering::Greater => println!("Too high!"),
// // // // // // // // // // // //             std::cmp::Ordering::Equal => {
// // // // // // // // // // // //                 println!("Congratulations! You guessed the number {} in {} attempts.", secret_number, attempts);
// // // // // // // // // // // //                 break;
// // // // // // // // // // // //             }
// // // // // // // // // // // //         }
// // // // // // // // // // // //     }
// // // // // // // // // // // // }




// // // // // // // // // // // //             Question 10


// // // // // // // // // // // fn main() {
    


// // // // // // // // // // //     let num1: f64 = 10.0; 
// // // // // // // // // // //     let num2: f64 = 5.0;  
// // // // // // // // // // //     let operation = '+';  

// // // // // // // // // // //     // Perform the calculation based on the chosen operation
// // // // // // // // // // //     let result = match operation {
// // // // // // // // // // //         '+' => num1 + num2,
// // // // // // // // // // //         '-' => num1 - num2,
// // // // // // // // // // //         '*' => num1 * num2,
// // // // // // // // // // //         '/' => {
// // // // // // // // // // //             if num2 == 0.0 {
// // // // // // // // // // //                 println!("Error: Division by zero.");
// // // // // // // // // // //                 return;
// // // // // // // // // // //             }
// // // // // // // // // // //             num1 / num2
// // // // // // // // // // //         }
// // // // // // // // // // //         _ => {
// // // // // // // // // // //             println!("Invalid operation. Please choose +, -, *, or /.");
// // // // // // // // // // //             return;
// // // // // // // // // // //         }
// // // // // // // // // // //     };

// // // // // // // // // // //     println!("Number 1: {}", num1);
// // // // // // // // // // //     println!("Number 2: {}", num2);
// // // // // // // // // // //     println!("Operation: {}", operation);
// // // // // // // // // // //     println!("Result: {} {} {} = {}", num1, operation, num2, result);
// // // // // // // // // // // }







// // // // // // // // // // //         Question 11


// // // // // // // // // // fn find_largest_number(arr: &[i32]) -> Option<i32> {
// // // // // // // // // //     if arr.is_empty() {
// // // // // // // // // //         return None;
// // // // // // // // // //     }

// // // // // // // // // //     let mut largest = arr[0];

// // // // // // // // // //     for &num in arr.iter() {
// // // // // // // // // //         if num > largest {
// // // // // // // // // //             largest = num;
// // // // // // // // // //         }
// // // // // // // // // //     }

// // // // // // // // // //     Some(largest)
// // // // // // // // // // }

// // // // // // // // // // fn main() {
// // // // // // // // // //     let numbers = vec![5, 12, 9, 25, 4, 0, 8];
    
// // // // // // // // // //     match find_largest_number(&numbers) {
// // // // // // // // // //         Some(largest) => println!("The largest number is: {}", largest),

// // // // // // // // // //         None => println!("Error"),
// // // // // // // // // //     }
// // // // // // // // // // }



// // // // // // // // // //             Question 12;



// // // // // // // // // fn reverse_string(input: &str) -> String {
// // // // // // // // //     let reversed: String = input.chars().rev().collect();
// // // // // // // // //     reversed
// // // // // // // // // }

// // // // // // // // // fn main() {
// // // // // // // // //     let original = "Hello, World!";
// // // // // // // // //     let reversed = reverse_string(original);
// // // // // // // // //     println!("{}", original);
// // // // // // // // //     println!("Reversed: {}", reversed);
// // // // // // // // // }



// // // // // // // // //                 Question 13;


// // // // // // // // fn fibonacci_sequence(n: usize) -> Vec<i32> {
// // // // // // // //     let mut sequence = Vec::new();
    
// // // // // // // //     if n >= 1 {
// // // // // // // //         sequence.push(0);
// // // // // // // //     }
    
// // // // // // // //     if n >= 2 {
// // // // // // // //         sequence.push(1);
// // // // // // // //     }
    
// // // // // // // //     while sequence.len() < n {
// // // // // // // //         let next = sequence[sequence.len() - 1] + sequence[sequence.len() - 2];
// // // // // // // //         sequence.push(next);
// // // // // // // //     }
    
// // // // // // // //     sequence
// // // // // // // // }

// // // // // // // // fn main() {
// // // // // // // //     let n = 10; 
// // // // // // // //     let fibonacci_sequence = fibonacci_sequence(n);
    
// // // // // // // //     println!("Fibonacci Sequence  {} : {:?}", n, fibonacci_sequence);
// // // // // // // // }




// // // // // // //             Quesion 14


// // // // // // fn is_prime(num: i32) -> bool {
// // // // // //     if num <= 1 {
// // // // // //         return false;
// // // // // //     }
    
// // // // // //     let mut i = 2;
// // // // // //     while i * i <= num {
// // // // // //         if num % i == 0 {
// // // // // //             return false;
// // // // // //         }
// // // // // //         i += 1;
// // // // // //     }
    
// // // // // //     true
// // // // // // }

// // // // // // fn primes_limit(limit: i32) -> Vec<i32> {
// // // // // //     let mut primes = Vec::new();
    
// // // // // //     for num in 2..=limit {
// // // // // //         if is_prime(num) {
// // // // // //             primes.push(num);
// // // // // //         }
// // // // // //     }
    
// // // // // //     primes
// // // // // // }

// // // // // // fn main() {
// // // // // //     let n = 20;
// // // // // //     let prime_numbers = primes_limit(n);
    
// // // // // //     println!("Prime numbers up to {}: {:?}", n, prime_numbers);
// // // // // // }



// // // // // //                 Question 15



// // // // // fn take_and_modify_string(mut input_string: String) {
// // // // //     // Modify the string by appending " - Modified" to it
// // // // //     input_string.push_str(" - Modified");
// // // // //     println!("Inside the function: {}", input_string);
// // // // //     // The ownership of `input_string` will be transferred back to the caller when the function ends.
// // // // // }

// // // // // fn main() {
   
// // // // //     let mut my_string = String::from("Hello, World");

// // // // //     println!("Before function call: {}", my_string);

// // // // //     take_and_modify_string(my_string.clone());

    
   
// // // // //     println!("After function call: {}", my_string); 

    
// // // // //     let modified_string = String::from("Hello, World - Modified");
// // // // //     println!("New reference: {}", modified_string);

    
// // // // // }


// // // // //             Question 16



// // // // fn find_longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
// // // //     if str1.len() >= str2.len() {
// // // //         str1
// // // //     } else {
// // // //         str2
// // // //     }
// // // // }

// // // // fn main() {
// // // //     let string1 = "hockey";
// // // //     let string2 = "Badminton";

// // // //     let longest = find_longest(string1, string2);
    
// // // //     println!("The longest string is: {}", longest);
// // // // }


// // // //         Question 17


// // // use std::io;

// // // fn main() {
// // //     println!("Enter a string:");

// // //     let mut input_string = String::new();

// // //     io::stdin()
// // //         .read_line(&mut input_string)
// // //         .expect("Failed to read input.");


// // //         let input_string = input_string.trim();
   
// // //     if input_string.is_empty() {
// // //         println!("The input string is empty.");
// // //     } 
// // //     else {
// // //         let first_char = &input_string[0..1]; 
// // //         let last_char = &input_string[input_string.len() - 1..]; 

// // //         println!("First character: {}", first_char);
// // //         println!("Last character: {}", last_char);
// // //     }
// // // }





// //             Question 18



// fn count_substring(main_string: &str, substring: &str) -> usize {
//     let mut count = 0;
//     let mut start = 0;
    
//     while let Some(position) = main_string[start..].find(substring) {
//         count += 1;
//         start += position + substring.len();
//     }
    
//     count
// }

// fn main() {
//     let input_string = "sbsdassaababbbbb";
//     let substring = "ab";
    
//     let occurrences = count_substring(input_string, substring);
    
//     println!("The substring '{}' appeared {} times in the string.", substring, occurrences);
// }





        




