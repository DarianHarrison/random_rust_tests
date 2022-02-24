use rand::{
    Rng,
    thread_rng, 
    distributions::{
        Distribution, 
        Uniform
    }, 
    SeedableRng,
    rngs::StdRng,
};

fn main() {

    // seeded pseudo random number generator
    let mut rng = StdRng::seed_from_u64(10); // seeded
    let message = Uniform::from(25..50);
    let a = message.sample(&mut rng);
    println!("{:?}", a); // should always be the same

    // random tuple
    let mut rng = thread_rng();
    let tuple: (u8, u8) = rng.gen();
    println!("{:?}", tuple);

    // simulate a stream buffer of random values
    let message = Uniform::from(25..50);
    let mut rng = rand::thread_rng();
    let mut sum = 0;
    for _ in 0..10 { // buffer of 10 random values
        println!("{}", message.sample(&mut rng));
    }

    // most efficient way of generating vectors of random values between range with uniform distribution
    let mut rng = rand::thread_rng();
    let range = Uniform::from(0..20);
    let vals: Vec<u8> = (0..10).map(|_| rng.sample(&range)).collect();
    println!("{:?}", vals);
}
