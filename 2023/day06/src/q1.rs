pub fn run() {
    let times = vec![38, 67, 76, 73];
    let distances = vec![234, 1027, 1157, 1236];
    let mut res = 1;
    for (time, hs) in times.iter().zip(distances.iter()) {
        let mut count = 0;
        for x in 1..*time {
            let dist = (time - x) * x;
            if dist > *hs {
                count += 1;
            }
        }
        res *= count;
    }
    println!("{}", res);
}