use rand::Rng;
use std::{thread, time::Duration, usize};


fn create_area(size: (i32, i32)) -> Vec<Vec<bool>>{
    let (w, h)  = size;
    let mut area: Vec<Vec<bool>> = Vec::new();
    for _ in 0..h {
        let mut subvec: Vec<bool> = Vec::new();
        for _ in 0..w {
            let n = rand::thread_rng().gen_range(0..=1);
            if n == 1 {
                subvec.push(true);
            } else {
                subvec.push(false);
            }
        }
        area.push(subvec);
    }
    return area;
}

fn refill_area(area: &mut Vec<Vec<bool>>, area_size: (i32, i32)) {
    let mut to_reset: Vec<(usize, usize, bool)> = Vec::new();
    for (y, line) in area.iter().enumerate() {
        for (x, b) in line.iter().enumerate() {
            let n = count_cells_neighbours((x as i32, y as i32), area, area_size);
            if b == &true && (n > 3 || n < 2){
                to_reset.push((y, x, false));
            }
            if b == &false && (n == 3){
                to_reset.push((y, x, true));
            }
        }
    }

    for reseter in to_reset {
        let (x, y, value) = reseter;
        area[x][y] = value;
    }
}


fn count_cells_neighbours(cell: (i32, i32), area: &Vec<Vec<bool>>, area_size: (i32, i32)) -> i32 {
    let (aw, ah) = area_size;
    let (cx, cy) = cell;

    let mut nw: (i32, i32) = (cx-1, cy-1);
    let mut n: (i32, i32) = (cx, cy-1);
    let mut ne: (i32, i32) = (cx+1, cy-1);
    let mut e: (i32, i32) = (cx+1, cy);
    let mut se: (i32, i32) = (cx+1, cy+1);
    let mut s: (i32, i32) = (cx, cy+1);
    let mut sw: (i32, i32) = (cx-1, cy+1);
    let mut w: (i32, i32) = (cx-1, cy+1);

    if cx == 0 {
        nw = (aw - 1, cy-1);
        w  = (aw - 1, cy);
        sw = (aw - 1, cy+1);
    }
    if cx == aw - 1 {
        ne = (0, cy-1);
        e  = (0, cy);
        se = (0, cy+1);
    }
    if cy == 0 {
        nw = (cx-1, ah-1);  // has? changed
        n  = (cx, ah-1);
        ne = (cx+1, ah-1);  // has? changed
    }
    if cy == ah - 1 {
        sw = (cx-1, 0);  // has? changed
        s  = (cx, 0);
        se = (cx+1, 0);  // has? changed
    }

    let vars = [nw, n, ne, e, se, s, sw, w];

    let mut counter = 0;
    for var in vars {
        let (x, y) = var;
        let mut vx = x;
        let mut vy = y;

        if x == aw {
            vx = 0;
        }
        if x == -1 {
            vx = aw-1;
        }
        if y == ah {
            vy = 0;
        }
        if y == -1 {
            vy = ah-1;
        }

        let possible_n = area[vy as usize][vx as usize];
        if possible_n == true {
            counter += 1;
        }
    }

    return counter;
}


fn print_area(area: &Vec<Vec<bool>>) {
    println!("\x1B[2J");
    for line in area {
        for b in line {
            if b == &true {
                print!("o");
            } else {
                print!(" ");
            }
        }
        println!("")
    }
    println!("")
}


fn main() {
    let size = (64 as i32, 32 as i32);
    let mut area = create_area(size);
    loop {
       thread::sleep(Duration::from_millis(100));
       print_area(&area);
       refill_area(&mut area, size)
    }
}
