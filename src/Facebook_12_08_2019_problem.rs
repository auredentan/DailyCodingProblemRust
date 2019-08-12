
/*
This problem was asked by Facebook.

On a mysterious island there are creatures known as Quxes which come in three colors: red, green, and blue. One power of the Qux is that if two of them are standing next to each other, they can transform into a single creature of the third color.

Given N Quxes standing in a line, determine the smallest number of them remaining after any possible sequence of such transformations.

For example, given the input ['R', 'G', 'B', 'G', 'B'], it is possible to end up with a single Qux through the following steps:

        Arrangement       |   Change
----------------------------------------
['R', 'G', 'B', 'G', 'B'] | (R, G) -> B
['B', 'B', 'G', 'B']      | (B, G) -> R
['B', 'R', 'B']           | (R, B) -> G
['B', 'G']                | (B, G) -> R
['R']                     |

*/
use std::ops::{Add};

#[derive(Debug, Clone, PartialEq)]
struct Quxe {
    color: String
}

impl Add for Quxe {
    type Output = Quxe;

    fn add(self, other: Quxe) -> Quxe {
        let mut colors = vec!["R", "G", "B"];
        if other.color == self.color {
            panic!("Error");
        }
        colors.retain(|element| (element.to_string() != self.color && element.to_string() != other.color));
        return Quxe { color: colors[0].to_string() };
    }
}



fn Quxes(colors: Vec<&str>) -> usize {
    let quxes_vec = colors.into_iter().map(|color| Quxe{ color: color.to_string() }).collect::<Vec<Quxe>>();
    
    let mut stack: Vec<Quxe> = vec![];
    for quxe in quxes_vec{
        if stack.len() == 0 || stack[stack.len() - 1] == quxe {
            stack.push(quxe.clone());
            continue;
        }

        let new = quxe + stack[stack.len() - 1].clone();
        stack.pop();
        stack.push(new);

        while stack.len() > 1 && stack[stack.len() - 1] != stack[stack.len() - 2] {
            let a = stack.pop().unwrap();
            let b = stack.pop().unwrap();
            stack.push(a + b);
        }
    }

    let smaller_number_of_quxes = stack.len();
    println!("Smaller number of remaining Quxe(s) is {}", smaller_number_of_quxes);
    return smaller_number_of_quxes;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_example() {
        let n_remaining = Quxes(vec!["R", "G", "B", "G", "B"]);
        assert_eq!(n_remaining, 1);
    }
}
