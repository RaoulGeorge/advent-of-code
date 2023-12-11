use std::iter::repeat;

pub fn run() {
    let map = include_bytes!("./input.txt");
    let w = map.iter().position(|&b| b == b'\n').unwrap();
    let s = map.iter().position(|&b| b == b'S').unwrap();

    let mut c = vec![false; (w + 1) * w];
    let (mut pos, mut dir) = {
        if matches!(map[s - w - 1], b'|' | b'7' | b'F') {
            (s - w - 1, 0)
        } else if matches!(map[s + w + 1], b'|' | b'L' | b'J') {
            (s + w + 1, 2)
        } else {
            (s - 1, 3)
        }
    };

    repeat(())
        .position(|_| unsafe {
            *c.get_unchecked_mut(pos) = true;
            match (map.get_unchecked(pos), dir) {
                (b'|', 0) => pos -= w + 1,
                (b'|', 2) => pos += w + 1,
                (b'-', 1) => pos += 1,
                (b'-', 3) => pos -= 1,
                (b'L', 2) | (b'F', 0) => {
                    pos += 1;
                    dir = 1;
                }
                (b'L', 3) | (b'J', 1) => {
                    pos -= w + 1;
                    dir = 0;
                }
                (b'7', 0) | (b'J', 2) => {
                    pos -= 1;
                    dir = 3;
                }
                (b'7', 1) | (b'F', 3) => {
                    pos += w + 1;
                    dir = 2;
                }
                (b'S', _) => return true,
                (_, _) => unreachable!(),
            }
            false
        })
        .unwrap();

    let mut x = false;
    let res = map
        .iter()
        .enumerate()
        .filter(|(pos, cell)| {
            let pipe = unsafe { *c.get_unchecked(*pos) };
            x &= pos % (w + 1) != 0;
            x ^= pipe && matches!(*cell, b'|' | b'F' | b'7');
            x && (!pipe || **cell == b'.') && (pos % (w + 1) != w)
        })
        .count();

    println!("{}", res);
}
