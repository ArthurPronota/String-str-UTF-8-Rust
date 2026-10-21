fn main() {
    let s = String::from("привет") ;

    println!("len: {} (bytes), chars: {} (chars), capacity: {} (bytes)", s.len(), s.chars().count(), s.capacity());
} /* Out:
len: 12 (bytes), chars: 6 (chars), capacity: 12 (bytes)
*/
