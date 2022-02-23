// this is how we benchmark our top k hitters
    use rand::{thread_rng, Rng, distributions::{Distribution, Uniform}};


fn main() {

    // fixed seed




    // random tuple
    let mut rng = thread_rng();
    let tuple: (u8, u8) = rng.gen();

    // simulate a stream buffer of random values
    let between = Uniform::from(25..50);
    let mut rng = rand::thread_rng();
    let mut sum = 0;
    for _ in 0..100 { // buffer of 100 random values
        println!("{}", between.sample(&mut rng));
    }

    // most efficient way of generating vectors of random values between range with uniform distribution
    let mut rng = rand::thread_rng();
    let range = Uniform::new(0, 20);
    let vals: Vec<u8> = (0..10).map(|_| rng.sample(&range)).collect();


}
