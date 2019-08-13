/*
This problem was asked by Glassdoor.

An imminent hurricane threatens the coastal town of Codeville. 
If at most two people can fit in a rescue boat, 
and the maximum weight limit for a given boat is k, 
determine how many boats will be needed to save everyone.

For example, given a population with weights [100, 200, 150, 80] and a boat limit of 200, 
the smallest number of boats required will be three.

*/


fn how_many_boats(pop_weight: Vec<i64>, boats_used: i64) -> i64{
    let BOAT_LIMIT = 200;

    if pop_weight.len() < 2 {
        return boats_used + (pop_weight.len()) as i64;
    }

    let first = pop_weight[0].clone();
    let remaining: Vec<i64> = pop_weight.iter().skip(1).map(|&e| e.into()).collect();
    if first == BOAT_LIMIT {
        return how_many_boats(remaining, boats_used + 1);
    }
    
    let allowed = BOAT_LIMIT - first;
    let mut second_index = remaining.len() - 1;
    while allowed >= pop_weight[second_index] {
        second_index = second_index - 1;
    }
        
    if second_index == remaining.len() {
        return how_many_boats(remaining, boats_used + 1);
    }

    let mut p1: Vec<i64> = remaining.iter().take(second_index).map(|&e| e.into()).collect();
    let p2 = remaining.iter().skip(second_index + 1);
    p1.extend(p2);
    return how_many_boats(p1, boats_used + 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_example() {
        let mut vect = vec![100, 200, 150, 80];
        vect.sort();
        vect.reverse();
        let number = how_many_boats(vect, 0);
        assert_eq!(number, 3);
    }
}
