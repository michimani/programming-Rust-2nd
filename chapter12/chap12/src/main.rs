use std::ops::Mul;
use std::ops::Not;

fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq, Debug)]
enum CharacterType {
    Demon,
    Human,
}

struct Character {
    kind: CharacterType,
    name: String,
}

// 単項演算子 (真偽反転) の実装
impl Not for Character {
    type Output = Character;
    fn not(self) -> Character {
        let kind = match &self.kind {
            CharacterType::Demon => CharacterType::Human,
            CharacterType::Human => CharacterType::Demon,
        };

        Character {
            kind: kind,
            name: self.name,
        }
    }
}

// 二項演算子 (乗算) の実装
impl Mul for Character {
    type Output = Character;
    fn mul(self, rhs: Self) -> Character {
        let name = format!("{}-{}", &self.name, &rhs.name);
        let kind = match &self.kind == &rhs.kind {
            true => CharacterType::Human,
            false => CharacterType::Demon,
        };

        Character { kind, name }
    }
}

#[test]
fn test_character_not() {
    let c = Character {
        kind: CharacterType::Human,
        name: "Rengoku-san".to_string(),
    };

    assert_eq!((!c).kind, CharacterType::Demon);
}

#[test]
fn test_character_mul() {
    let cd1 = Character {
        kind: CharacterType::Demon,
        name: "Demon1".to_string(),
    };
    let cd2 = Character {
        kind: CharacterType::Demon,
        name: "Demon2".to_string(),
    };
    let cd3 = Character {
        kind: CharacterType::Demon,
        name: "Demon3".to_string(),
    };
    let cd4 = Character {
        kind: CharacterType::Demon,
        name: "Demon4".to_string(),
    };

    let ch1 = Character {
        kind: CharacterType::Human,
        name: "Human1".to_string(),
    };
    let ch2 = Character {
        kind: CharacterType::Human,
        name: "Human2".to_string(),
    };
    let ch3 = Character {
        kind: CharacterType::Human,
        name: "Human3".to_string(),
    };
    let ch4 = Character {
        kind: CharacterType::Human,
        name: "Human4".to_string(),
    };

    let cdd = cd1 * cd2;
    let chh = ch1 * ch2;
    let cdh = cd3 * ch3;
    let chd = ch4 * cd4;

    assert_eq!(cdd.kind, CharacterType::Human);
    assert_eq!(cdd.name, "Demon1-Demon2");

    assert_eq!(chh.kind, CharacterType::Human);
    assert_eq!(chh.name, "Human1-Human2");

    assert_eq!(cdh.kind, CharacterType::Demon);
    assert_eq!(cdh.name, "Demon3-Human3");

    assert_eq!(chd.kind, CharacterType::Demon);
    assert_eq!(chd.name, "Human4-Demon4");
}
