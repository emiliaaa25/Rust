use std::fs;
use serde_derive::Deserialize;

#[derive(Debug, Clone, Deserialize)]
struct Student {
    nume: String,
    numar: String,
    ani: i32,
}

fn main() {
    let s = fs::read_to_string("example.txt").unwrap();
    let mut mare = Student { nume: String::new(), numar: String::new(), ani: 0 };
    let mut mic = Student { nume: String::new(), numar: String::new(), ani: i32::MAX };
    let mut max_age = i32::MIN;

    for line in s.lines() {
        let student: Student = serde_json::from_str(line).unwrap();

        if student.ani > max_age {
            max_age = student.ani;
            mare = student.clone();
        }

        if student.ani < mic.ani {
            mic = student;
        }
    }

    println!("Cel mai în vârstă student: nume={}, numar={}, ani={}", mare.nume, mare.numar, mare.ani);
    println!("Cel mai tânăr student: nume={}, numar={}, ani={}", mic.nume, mic.numar, mic.ani);
}
