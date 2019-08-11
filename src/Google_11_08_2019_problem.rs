
/*
This problem was asked by Google.

The game of Nim is played as follows. Starting with three heaps, each containing a variable number of items, two players take turns removing one or more items from a single pile. The player who eventually is forced to take the last stone loses. For example, if the initial heap sizes are 3, 4, and 5, a game could be played as shown below:

  A  |  B  |  C
-----------------
  3  |  4  |  5
  3  |  1  |  3
  3  |  1  |  3
  0  |  1  |  3
  0  |  1  |  0
  0  |  0  |  0 
In other words, to start, the first player takes three items from pile B. The second player responds by removing two stones from pile C. The game continues in this way until player one takes last stone and loses.

Given a list of non-zero starting values [a, b, c], and assuming optimal play, determine whether the first player has a forced win.
*/


/* Solution 
piles/heaps at any point of the game is called Nim-Sum at that point.

“If both A and B play optimally (i.e- they don’t make any mistakes), then the player starting first is guaranteed to win if the Nim-Sum at the beginning of the game is non-zero. Otherwise, if the Nim-Sum evaluates to zero, then player A will lose definitely
*/

use std::ops::BitXor;

fn compute_nim_sum(nim_board: Vec<u32>) -> u32 {

    let n: u32 = nim_board.len() as u32;
    let mut bits = 0;

    for x in nim_board {
        bits = bits ^ x;
    }
    let ans = bits * u32::pow(2, n-1);
    return ans;
}

fn nim_winner(nim_board: Vec<u32>) -> u32 {
    let nim_sum = compute_nim_sum(nim_board);

    if nim_sum != 0 {
        println!("First player to play will win the game !");
        return 1;
    } else {
        println!("Second player to play will win the game !");
        return 2;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_example() {
        let winner = nim_winner(vec![3,4,5]);
        assert_eq!(winner, 1);
    }
}
