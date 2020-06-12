
#[test]
fn euler002() {
    let result = fibos()
        .take_while(|&x| x <= 4000000)
        .filter(|&x| x % 2 == 0)
        .fold(0, |acc, v| acc + v);

    println!("euler 002: {0}", result.to_string());

}

// okay i confess I cribbed this.
struct Fibos {
    curr: u32,
    next: u32,
}

impl Iterator for Fibos {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        return Some(self.curr);

    }
}

fn fibos() -> Fibos {
    Fibos { curr: 0, next: 1}
}

