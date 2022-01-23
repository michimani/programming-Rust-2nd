fn main() {
    let mut num = 0;

    'top: loop {
        if num > 200 {
            break;
        }

        if num < 100 {
            loop {
                if num > 100 {
                    break 'top;
                }
                num += 1;
            }
        }

        num += 10;
    }

    assert_eq!(num, 101);
    // assert_eq!(num, 201);
}
