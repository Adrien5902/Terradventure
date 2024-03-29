use rand::{prelude::SliceRandom, thread_rng};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RandomWeightedTable<T>
where
    T: Clone,
{
    rolls: usize,
    rates: Vec<RandomWeightedRate<T>>,
}

impl<T> Default for RandomWeightedTable<T>
where
    T: Clone,
{
    fn default() -> Self {
        Self {
            rates: Vec::new(),
            rolls: 0,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct RandomWeightedRate<T>
where
    T: Clone,
{
    pub data: T,
    pub weight: u32,
}

impl<T> RandomWeightedTable<T>
where
    T: Clone,
{
    pub fn new(rolls: usize, rates: Vec<RandomWeightedRate<T>>) -> Self {
        Self { rates, rolls }
    }

    pub fn new_empty() -> Self {
        Self {
            rolls: 0,
            rates: Vec::new(),
        }
    }

    pub fn get_random(&self) -> Vec<T> {
        self.rates
            .choose_multiple_weighted(&mut thread_rng(), self.rolls, |item| item.weight)
            .unwrap()
            .map(|item| item.data.clone())
            .collect::<Vec<_>>()
    }
}
