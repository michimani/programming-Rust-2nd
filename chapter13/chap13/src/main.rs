use std::ops::Drop;

fn main() {
    born_and_die()
}

#[derive(Debug)]
struct Human {
    name: String,
    beloved: String,
}

impl Drop for Human {
    fn drop(&mut self) {
        println!("I'm {}. Thank you {}. Good by...", self.name, self.beloved)
    }
}

fn born_and_die() {
    let h = Human {
        name: "Ken".to_string(),
        beloved: "Emi".to_string(),
    };

    println!("{:?}", h);
}

#[test]
fn test_iter_partision() {
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let (even, odd): (Vec<usize>, Vec<usize>) = numbers.iter().partition(|&n| n % 2 == 0);

    assert_eq!(even, [2, 4, 6, 8, 10]);
    assert_eq!(odd, [1, 3, 5, 7, 9]);
}

#[test]
fn test_default() {
    use std::default::Default;

    struct CocoichiCurry {
        hot: i32,
    }

    impl Default for CocoichiCurry {
        fn default() -> CocoichiCurry {
            return CocoichiCurry { hot: 2 };
        }
    }

    let c10 = CocoichiCurry { hot: 10 };
    let c2 = CocoichiCurry::default();
    assert_eq!(c10.hot, 10);
    assert_eq!(c2.hot, 2)
}

#[test]
fn test_ip_from() {
    use std::net::Ipv4Addr;

    let addr = Ipv4Addr::new(203, 0, 113, 0);
    assert_eq!(addr, Ipv4Addr::from([203, 0, 113, 0]));
}
