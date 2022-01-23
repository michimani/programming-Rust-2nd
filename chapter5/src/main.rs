fn main() {
    // 5.3.2
    f(&32);

    // 5.3.3
    let x = 10;
    g(&x);

    // 5.3.4
    let s;
    {
        let parabola = [9, 4, 1, 0, 1, 4, 9];
        s = smallest(&parabola);
        assert_eq!(*s, 0);
    }

    // 5.3.5
    let ss;
    let d;
    {
        let x = 10;
        ss = S { r: &x };
        d = D { s: ss };
        assert_eq!(*d.s.r, 10);
    }

    // 5.3.6
    let x = 10;
    let r;
    {
        let y = 20;
        {
            let xy = XY { x: &x, y: &y };
            r = xy.x;
        }
    }
    println!("{}", r)
}

// 5.3.2
static mut STASH: &i32 = &128;
fn f(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}

// 5.3.3
fn g<'a>(p: &'a i32) {
    print!("{}", p)
}

// 5.3.4
fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];

    for r in &v[1..] {
        if *r < *s {
            s = r;
        }
    }
    s
}

// 5.3.5
struct S<'a> {
    // r: &i32, // コンパイルエラー: expected named lifetime parameter
    // r: &'static i32, // 制約が強すぎる
    r: &'a i32,
}

struct D<'a> {
    // s: S, // expected named lifetime parameter
    // s: S<'static>, // 制約が強すぎる
    s: S<'a>,
}

// 5.3.6
struct XY<'a, 'b> {
    x: &'a i32,
    // y: &'a i32, // x と同じ生存期間はとれない
    y: &'b i32,
}
