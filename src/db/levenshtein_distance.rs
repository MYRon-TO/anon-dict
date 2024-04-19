pub fn distance(a: &str, b: &str) -> usize {
    // if a is longer than b, switch them
    let (a, b) = if a.chars().count() < b.chars().count() {
        (a, b)
    } else {
        (b, a)
    };

    let len_a = a.chars().count();
    let len_b = b.chars().count();

    if len_a == 0 {
        return len_b;
    } else if len_b == 0 {
        return len_a;
    }

    let len_b = len_b + 1;

    // # calculate
    let mut pre;
    let mut tmp;
    let mut cur = vec![0; len_b];

    // initialize string b
    (1..len_b).for_each(|i| {
        cur[i] = i;
    });

    // calculate edit distance
    for (i, ca) in a.chars().enumerate() {
        // get first column for this row
        pre = cur[0];
        cur[0] = i + 1;
        for (j, cb) in b.chars().enumerate() {
            tmp = cur[j + 1];
            cur[j + 1] = std::cmp::min(
                // deletion
                tmp + 1,
                std::cmp::min(
                    // insertion
                    cur[j] + 1,
                    // match or substitution
                    pre + if ca == cb { 0 } else { 1 },
                ),
            );
            pre = tmp;
        }
    }
    cur[len_b - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        assert_eq!(distance("kitten", "sitting"), 3);
        assert_eq!(distance("sitting", "kitten"), 3);
        assert_eq!(distance("sitting", "sitting"), 0);
    }
}
