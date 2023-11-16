use std::{fs, process};

trait Comanda {
    fn get_name(&self) -> String;
    fn exec(&mut self, args: &[String]);
}

struct Ping;

impl Comanda for Ping {
    fn get_name(&self) -> String {
        "ping".to_string()
    }

    fn exec(&mut self, _args: &[String]) {
        println!("pong!");
    }
}

struct Count;

impl Comanda for Count {
    fn get_name(&self) -> String {
        "count".to_string()
    }

    fn exec(&mut self, args: &[String]) {
        println!("counted {} args", args.len());
    }
}

struct Times {
    count: u32,
}

impl Comanda for Times {
    fn get_name(&self) -> String {
        "times".to_string()
    }

    fn exec(&mut self, _args: &[String]) {
        self.count += 1;
        println!("command has been called {} times", self.count);
    }
}
struct Stop;

impl Comanda for Stop {
    fn get_name(&self) -> String {
        "stop".to_string()
    }

    fn exec(&mut self, _args: &[String]) {
        println!("Am oprit executia!");
        process::exit(0);
    }
}
struct Add;

impl Comanda for Add{
    fn get_name(&self) -> String {
        "add".to_string()
    }

    
    fn exec(&mut self, args: &[String]) {
        if args.len() == 2 {
            let mut string = args[0].to_string();
            let character = args[1].chars().next().unwrap_or('\0');
            string.push(character);
            println!("New string: {}",string);
        } else {
            println!("Invalid number of arguments for add_from_file");
        }
    }
    }


struct Terminal {
    comenzi: Vec<Box<dyn Comanda>>,
}

impl Terminal {
    fn new() -> Self {
        Terminal { comenzi: Vec::new() }
    }

    fn register(&mut self, command: Box<dyn Comanda>) {
        self.comenzi.push(command);
    }

    fn run(&mut self) {
        let s = fs::read_to_string("example.txt");
        match s {
            Ok(s) => {
                for line in s.lines() {

                    let mut parts = line.split_ascii_whitespace();
                    
                    if let Some(command_name) = parts.next() {
                        if !command_name.is_empty() {
                        let mut args = Vec::new();
                        for arg in parts {
                            args.push(String::from(arg));
                        }
                    
                        let mut command_found = 0;
                        for command in &mut self.comenzi {
                        if command.get_name() == command_name  {
                            command_found = 1;
                            command.exec(&args);
                            break;
                        }
                }

                        if command_found != 1  {
                            println!("Invalid command: {}", command_name);
                         }
                    }
                }
            }
        }
            Err(err) => {
                eprintln!("Error reading the file {}", err);
            }
        }
    }

}

fn main() {
    let mut terminal = Terminal::new();

    terminal.register(Box::new(Ping {}));
    terminal.register(Box::new(Count {}));
    terminal.register(Box::new(Times { count: 0 }));
    terminal.register(Box::new(Stop {}));
    terminal.register(Box::new(Add {}));

    terminal.run();
}
