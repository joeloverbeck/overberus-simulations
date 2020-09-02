pub fn manhattan_distance(x1: u32, y1: u32, x2: u32, y2: u32) -> u32 {
    let x_diff = if x1 < x2 { x2 - x1 } else { x1 - x2 };
    let y_diff = if y1 < y2 { y2 - y1 } else { y1 - y2 };
    x_diff + y_diff
}
