fn main() {
    println!("Hello, world!");
}

#[test]
fn test_raw_pointer() {
    let mut x = 10;
    let x_ptr = &mut x as *mut i32;

    let mut y = 20;
    let y_ptr = &mut y as *mut i32;

    unsafe {
        println!("{:?}", x_ptr);
        println!("{:?}", y_ptr);
        assert_eq!(*x_ptr, 10);
        assert_eq!(*y_ptr, 20);
    }
}
