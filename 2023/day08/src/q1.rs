pub fn run() {
    let input = include_bytes!("./input.txt");
    let lines = input.iter().position(|&c| c == b'\n').unwrap();

    let mut map = [0u64; 399999];
    let enc = |n: &[u8]| {
        ((n[0] - b'0') as u64) << 12 | ((n[1] - b'0') as u64) << 6 | (n[2] - b'0') as u64
    };
    input[lines + 2..].split(|&c| c == b'\n').for_each(|n| {
        map[enc(&n[0..3]) as usize] = enc(&n[7..10]) | enc(&n[12..15]) << 32;
    });

    let res = input[0..lines]
        .iter()
        .cycle()
        .scan(enc(b"AAA"), |n, step| {
            *n = if step == &b'L' {
                map[*n as usize] & u32::MAX as u64
            } else {
                map[*n as usize] >> 32
            };
            Some(*n & 0b111111 == (b'Z' - b'0') as u64)
        })
        .position(|n| n)
        .unwrap()
        + 1;

    println!("{}", res);
}
