use std::{thread, time};

fn main() {
    let mut A: f32 = 0.;
    let mut B: f32 = 0.;
    let mut i: f32;
    let mut j: f32;
    let mut z: [f32; 1760];
    // let mut z = vec![0.0; 1760];
    let mut b: [char; 1760];
    // let mut N: i32;
    let s = String::from(".,-~:;=!*#$@");

    print!("\x1B[2J");

    loop {
        b = [' '; 1760];
        z = [0.0; 1760];
        // z = z.into_iter().map(|x| x * 0.0).collect::<Vec<_>>();
        j = 0.0;
        i = 0.0;
        while j < 6.28 {
            while i < 6.28 {
                let c: f32 = i.sin();
                let d: f32 = j.cos();
                let e: f32 = A.sin();
                let f: f32 = j.sin();
                let g: f32 = A.cos();
                let h: f32 = d + 2.0;
                let D: f32 = 1.0 / (c * h * e + f * g + 5.0);
                let l: f32 = i.cos();
                let m: f32 = B.cos();
                let n: f32 = B.sin();
                let t: f32 = c * h * g - f * e;
                let x: i32 = (40.0 + 30.0 * D * (l * h * m - t * n)) as i32;
                let y: i32 = (12.0 + 15.0 * D * (l * h * n + t * m)) as i32;
                let o: usize = (x + 80 * y) as usize;
                let N: usize = ((8.0 * ((f * e - c * d * g) * m - c * d * e - f * g - l * d * n))
                    as i32)
                    .abs() as usize;

                if 22 > y && y > 0 && x > 0 && 80 > x && D > z[o] {
                    z[o] = D;
                    if N > 0 {
                        b[o] = *&s.as_bytes()[N] as char;
                    } else {
                        b[o] = *&s.as_bytes()[0] as char;
                    }
                }
                i += 0.02;
            }
            j += 0.07;
        }
        print!("\x1B[H");

        for k in 0..=1760 {
            if k % 80 != 0 {
                print!("{}", b[k as usize])
            } else {
                println!()
            }
            A += 0.00004;
            B += 0.00002;
        }
        thread::sleep(time::Duration::from_millis(200));
    }
}
