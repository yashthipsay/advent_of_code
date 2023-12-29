// fn main() {
//     let a = String::from("owighownbw");
//     let b = String::from("xcbowh");

//     let result = longest_word(&a, &b);
//     println!("{}", result);

// }

// fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String{
//     if x.len() >y.len(){
//         x
//     } else {
//         y
//     }
// }

#[derive(Dubug, PartialEq, Copy, Clone)]

enum ProgrammingLanguage {
    Java, 
    Rust,
}

struct Inventory {
    shirts: Vec<ProgrammingLanguage>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ProgrammingLanguage>) -> ProgrammingLanguage {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ProgrammingLanguage {
        let mut num_java = 0;
        let mut num_rust = 0;
    }

    for color in &self.shirts {
        match color {
            ProgrammingLanguage::Java => num_java += 1,
            ProgrammingLanguage::Rust => num_rust += 1,
        }
    }

    if num_java > num_rust {
        ProgrammingLanguage::Java
    } else {
        ProgrammingLanguage::Rust
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ProgrammingLanguage]
    };

    let user_pref1 = Some(ProgrammingLanguage::Java);
    let giveaway1 = store.giveaway(user_pref1);
    println!("The user with preference {:?} will get a {:?}", user_pref1, giveaway1);

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("The user with preference {:?} will get a {:?}", user_pref2, giveaway2);
}