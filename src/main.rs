extern crate csv;
extern crate time;
extern crate rand;
extern crate rustc_serialize;

use rand::{Rng, SeedableRng, StdRng};

const FILE_PATH: &'static str = "iris.data";
const PERCENT_TEST_ROWS: f64 = 0.30;

#[derive(RustcDecodable)]
struct IrisFlower {
    sepal_length: f32,
    sepal_width: f32,
    petal_length: f32,
    petal_width: f32,
}

fn main() {
    let mut csv_reader = csv::Reader::from_file(FILE_PATH).unwrap();

    let seed: &[_] = &[1, 2];
    let mut rng: StdRng = SeedableRng::from_seed(seed);

	// Learn
    for decoded_row in csv_reader.decode() {
        if rng.gen_range(0.0, 1.0) > PERCENT_TEST_ROWS  {
			let iris_flower: IrisFlower = decoded_row.unwrap();

			println!("sl: {}", iris_flower.petal_length);
        }
    }

	rng.reseed(seed);

	// Test
    for decoded_row in csv_reader.decode() {
        if rng.gen_range(0.0, 1.0) <= PERCENT_TEST_ROWS  {
			let iris_flower: IrisFlower = decoded_row.unwrap();

			println!("sl: {}", iris_flower.petal_length);
        }
    }
}
