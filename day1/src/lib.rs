pub fn calc_diffs(m: &[i32]) -> Vec<bool> {
    let mut last = m[0];
    m[1..]
        .iter()
        .map(|v| {
            let increased = v > &last;
            last = *v;
            increased
        })
        .collect()
}

pub fn count_increases(m: &[i32]) -> u32 {
    calc_diffs(m)
        .iter()
        .filter(|v| *v.clone())
        .collect::<Vec<&bool>>()
        .len() as u32
}

pub fn count_increases2(m: &[i32]) -> u32 {
    let m: Vec<i32> = m.windows(3).map(|w| w.iter().sum()).collect();
    count_increases(&m)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_measurments() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(count_increases(&input), 7);
    }

    #[test]
    fn test_measurments2() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(count_increases2(&input), 5);
    }
}
