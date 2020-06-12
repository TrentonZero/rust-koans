
#[test]
fn euler001() {
    let arr = 1..1000;
    let result = arr
        .filter(|&x| x % 3 == 0 || x % 5 == 0)
        .fold(0, |acc, v| acc + v);

    println!("euler 001: {0}", result.to_string());

}

