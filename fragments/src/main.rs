fn main() {
    let numbers = vec![12, 3, 5, 1];
    let smallest = smallest(&numbers);
    println!("{} is the smallest number in {:?}", smallest, numbers);

    let names = StringTable {
        elements: vec!["Albert".to_string(), "Rodianne".to_string()],
    };
    let prefix = "A";
    let name_starts_with = names.find_by_prefix(prefix);
    print!("{} was found staring with '{}'",
           match name_starts_with {
               Some(n) => format!("The name {}", n),
               None => "No name".to_string(),
           }, prefix);
}

fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s {
            s = r;
        }
    }
    s
}

struct StringTable {
    elements: Vec<String>,
}

impl StringTable {
    fn find_by_prefix(&self, prefix: &str) -> Option<&String> {
        for i in 0..self.elements.len() {
            if self.elements[i].starts_with(prefix) {
                return Some(&self.elements[i]);
            }
        }
        None
    }
}
