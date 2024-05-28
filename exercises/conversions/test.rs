fn from(s: &str) {
    let parts: Vec<&str> = s.split(',').collect();
    for part in &parts {
        println!("{}", parts)
    }
}

fn main() {
  let thing = from("text, example");
}

