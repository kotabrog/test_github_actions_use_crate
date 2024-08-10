use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen();
    let y = std::f64::consts::PI;
    println!("Hello, world!! random num: {}", x * y);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
