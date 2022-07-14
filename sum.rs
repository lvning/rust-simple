fn calc_sum(arr: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
    for x in arr {
        let (_sum, overflow) = sum.overflowing_add(*x);
        if overflow {
            return None;
        }
        sum = _sum;
    }
    Some(sum)
}

fn main() {
    println!(
        "Calculate normal result: {:?}",
        calc_sum(&[u32::MAX - 1, 1])
    );
    println!(
        "Calculate overflow result: {:?}",
        calc_sum(&[u32::MAX - 1, 2])
    );
}
