fn main() {
  let mut s1 = String::from("hello");
  println!("s1 before: {}", s1);

  // let len = calculate_length(&s1);

  // println!("The length of '{}' is {}.", s1, len);
  change(&mut s1);
  println!("s1 after: {}", s1);
}

fn calculate_length(s: &String) -> usize {
  s.len()
}

fn change(some_string: &mut String) {
  some_string.push_str(", world");
}