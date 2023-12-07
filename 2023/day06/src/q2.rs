pub fn run() {
    let times: Vec<i64> = vec![38677673];
    let distances: Vec<i64> = vec![234102711571236];
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