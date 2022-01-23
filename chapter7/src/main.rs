use std::io;

fn main() {
    // let ans = divide(10, 0);
    let ans = match divide_safe(10, 0) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("{:?}", e);
            std::process::exit(1)
        }
    };

    println!("{}", ans)
}

fn divide(num: usize, divisor: usize) -> usize {
    num / divisor
}

fn divide_safe(num: usize, divisor: usize) -> Result<usize, io::Error> {
    if divisor == 0 {
        let err_msg = "cannot divide with 0 value";
        return Err(io::Error::new(io::ErrorKind::Other, err_msg));
    }

    Ok(num / divisor)
}
