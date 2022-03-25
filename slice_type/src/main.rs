fn main() {
  let mut s = String::from("hello world");
  let word = first_word(&s);
  
  s.clear();
}

fn first_word(s: &String) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
          return &s[0..i];
      }
  }

  &s[..]
}

/*
  let a = [1, 2, 3, 4, 5];

  let slice = &a[1..3];

  assert_eq!(slice, &[2, 3]);
*/

/*
  let s = String::from("hello world");

  let hello = &s[0..5];
  let world = &s[6..11];
  println!("first is:  {}, second is: {}", hello, world);
*/

/*
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];
*/

/*
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
*/

/*
let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];
*/