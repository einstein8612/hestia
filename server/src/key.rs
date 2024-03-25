use chrono::{NaiveDate, NaiveDateTime, Utc};
use rand::prelude::*;

const HESTIA_EPOCH: NaiveDateTime = match NaiveDate::from_ymd_opt(2023, 1, 1) {
    Some(date) => match date.and_hms_opt(0, 0, 0) {
        Some(datetime) => datetime,
        None => panic!("error building epoch"),
    },
    None => panic!("error building epoch"),
};

///
/// Generate a new key
///
/// This function generates a new key using the following algorithm:
/// 1. Generate a random 16-bit number
/// 2. Generate a 48-bit number representing the number of milliseconds since the Hestia epoch (2022-01-01T00:00:00Z)
/// 3. Combine the two numbers into a 64-bit number
/// 4. Encode the 64-bit number using base62 encoding
/// 
/// This guarantees that the key is unique, granted there are no collisions in the random number generation
/// at time x. There is a 50% possiblity of a collision if 110000 keys are generated at the same millisecond, assuming
/// a uniform distribution of random numbers.
/// 
/// For our usecase of generating keys for bins, this is acceptable. Settling for a randomness based algorithm
/// for key generation is mostly due to the fact bin keys shouldn't be guessable. 
///
pub fn generate_key() -> String {
    // let entropy = (self.random.next_u32() as u64) << 48;
    let entropy = (thread_rng().next_u32() as u64) << 48;
    let ms_since_epoch = ((Utc::now().naive_utc()
                                .signed_duration_since(HESTIA_EPOCH)
                                .num_milliseconds() as u64) << 16) >> 16;
    let number_representation = entropy | ms_since_epoch;
    base62::encode(number_representation)
}