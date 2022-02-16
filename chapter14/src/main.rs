fn main() {}

#[derive(Copy, Clone)]
enum Statistic {
    UserWeight,
    UserHeight,
}

#[derive(Debug)]
struct User {
    name: String,
    weight: i64,
    height: i64,
}

impl User {
    fn get_statistic(&self, stat: Statistic) -> i64 {
        match stat {
            Statistic::UserWeight => self.weight,
            Statistic::UserHeight => self.height,
        }
    }
}

fn sort_by_static(users: &mut Vec<User>, stat: Statistic) {
    users.sort_by_key(|user| -user.get_statistic(stat));
}

#[test]
fn test_sort_users() {
    let mut users = Vec::new();
    users.push(User {
        name: "user-1".to_string(),
        weight: 50,
        height: 165,
    });
    users.push(User {
        name: "user-2".to_string(),
        weight: 70,
        height: 160,
    });
    users.push(User {
        name: "user-3".to_string(),
        weight: 60,
        height: 180,
    });

    println!("{:?}", users);

    sort_by_static(&mut users, Statistic::UserHeight);
    println!("{:?}", users);

    sort_by_static(&mut users, Statistic::UserWeight);
    println!("{:?}", users);
}

use std::thread;
fn start_sorting_thread(mut users: Vec<User>, stat: Statistic) -> thread::JoinHandle<Vec<User>> {
    // let key_fn = |user: &User| -> i64 { -user.get_statistic(stat) };
    let key_fn = move |user: &User| -> i64 { -user.get_statistic(stat) };

    // thread::spawn(|| {
    thread::spawn(move || {
        users.sort_by_key(key_fn);
        println!("{:?}", users);
        users
    })
}

#[test]
fn test_start_sorting_thread() {
    let mut users = Vec::new();
    users.push(User {
        name: "user-1".to_string(),
        weight: 50,
        height: 165,
    });
    users.push(User {
        name: "user-2".to_string(),
        weight: 70,
        height: 160,
    });
    users.push(User {
        name: "user-3".to_string(),
        weight: 60,
        height: 180,
    });

    println!("{:?}", users);

    start_sorting_thread(users, Statistic::UserHeight);
}

fn count_specific_user(users: &Vec<User>, test_fn: fn(u: &User) -> bool) -> usize {
    let mut count = 0;
    for user in users {
        if test_fn(&user) {
            count += 1;
        }
    }

    count
}

#[test]
fn test_count_specific_user() {
    struct Case {
        test_fn: fn(u: &User) -> bool,
        want: usize,
    }

    let mut users = Vec::new();
    users.push(User {
        name: "user-1".to_string(),
        weight: 50,
        height: 165,
    });
    users.push(User {
        name: "user-2".to_string(),
        weight: 70,
        height: 160,
    });
    users.push(User {
        name: "user-3".to_string(),
        weight: 60,
        height: 180,
    });

    fn t1(u: &User) -> bool {
        u.weight > 50
    }
    fn t2(u: &User) -> bool {
        u.height > 170
    }

    let cases = [
        Case {
            test_fn: t1,
            want: 2,
        },
        Case {
            test_fn: t2,
            want: 1,
        },
    ];

    for c in cases {
        let ac = count_specific_user(&users, c.test_fn);
        assert_eq!(ac, c.want);
    }
}
