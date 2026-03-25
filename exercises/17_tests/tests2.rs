// Calculates the power of 2 using a bit shift.
// `1 << n` is equivalent to "2 to the power of n".
fn power_of_2(n: u8) -> u64 {
    1 << n
}

fn main() {
    // You can optionally experiment here.
}

#[test]
fn you_can_assert_eq() {
    assert_eq!(power_of_2(0), 1);   // 2^0 = 1
    assert_eq!(power_of_2(1), 2);   // 2^1 = 2
    assert_eq!(power_of_2(8), 256); // 2^8 = 256
    assert_eq!(power_of_2(10), 1024); // 2^10 = 1024
}
