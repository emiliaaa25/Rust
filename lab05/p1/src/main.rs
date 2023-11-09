use std::fs;

#[derive( Clone)]
struct Student {
    nume: String,
    numar: String,
    ani: i32,
}

fn main() {
    let content = fs::read_to_string("example.txt").unwrap();
    let mut mare: Student = Student { nume: "name".to_string(), numar: "numar".to_string(), ani: 0 };
    let mut mic: Student = Student { nume: "name".to_string(), numar: "numar".to_string(), ani: 255 };
    let mut max_age = i32::MIN;
    let mut min_age = i32::MAX;

    for line in content.lines() {
        let mut iter = line.trim().split(',');
        let nume = iter.next().unwrap_or("").to_string();
        let numar = iter.next().unwrap_or("").to_string();
        let ani = iter.next().unwrap_or("0").parse::<i32>().unwrap_or(0);
        let student = Student { nume, numar, ani };

        if ani > max_age {
            max_age = ani;
            mare = student.clone();
        }

        if ani < min_age {
            min_age = ani;
            mic = student;
        }
    }

    println!("Cel mai in varsta student: nume={}, numar={}, ani={}", mare.nume, mare.numar, mare.ani);
    println!("Cel mai tanar student: nume={}, numar={}, ani={}", mic.nume, mic.numar, mic.ani);
}
