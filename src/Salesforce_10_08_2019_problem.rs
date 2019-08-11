#[allow(non_snake_case)]

/*
This problem was asked by Salesforce.

The number 6174 is known as Kaprekar's contant, 
after the mathematician who discovered an associated property: for all four-digit numbers with at least two distinct digits, 
epeatedly applying a simple procedure eventually results in this value. 

The procedure is as follows:

For a given input x, create two new numbers that consist of the digits in x in ascending and descending order.
Subtract the smaller number from the larger number.
For example, this algorithm terminates in three steps when starting from 1234:

4321 - 1234 = 3087
8730 - 0378 = 8352
8532 - 2358 = 6174
Write a function that returns how many steps this will take for a given input N.
*/

fn algo_iteration(x: i64) -> i64 {

    // Helper closures
    let number_to_vec = |n: i64| -> Vec<i64> {
        return n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect::<Vec<i64>>();
    };

    let vec_to_number = |vec: Vec<i64>| -> i64 {
        return vec.clone()
        .into_iter()
        .map(|i| i.to_string())
        .collect::<String>()
        .parse::<i64>()
        .unwrap();
    };

    let mut x_vec: Vec<i64> = number_to_vec(x);
    x_vec.sort();
    let mut x_rev = x_vec.clone();
    x_rev.reverse();

    let x_ascending: i64 = vec_to_number(x_vec);
    let x_descending: i64 =  vec_to_number(x_rev);

    if x_ascending > x_descending {
        return (x_descending-x_ascending).abs();
    } else {
        return (x_ascending-x_descending).abs();
    }
}


pub fn Salesforce_10_08_2019_problem(x: i64) -> i64 {
    let mut res: i64 = x;
    let mut n_steps: i64 = 0;
    while res != 6174 {
        res = algo_iteration(res);
        n_steps += 1;
    } 
    println!("Salesforce_10_08_2019_problem ended after {} steps", n_steps);
    return n_steps;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_example() {
        let res: i64 = Salesforce_10_08_2019_problem(1234);
        assert_eq!(res, 3);
    }
}
