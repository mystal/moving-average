extern crate num_traits;

use std::collections::VecDeque;

use num_traits::{Num, NumCast, zero};


pub struct MovingAverage<T>
    where T: Num + NumCast + Copy {
    data: VecDeque<T>,
    running_sum: T,
    capacity: usize,
}

impl<T> MovingAverage<T>
    where T: Num + NumCast + Copy {
    pub fn new(capacity: usize) -> Self {
        MovingAverage {
            data: VecDeque::with_capacity(capacity),
            running_sum: zero(),
            capacity: capacity,
        }
    }

    pub fn add(&mut self, value: T) {
        if self.len() == self.capacity() {
            self.running_sum = self.running_sum - self.data.pop_front()
                .expect("Internal buffer should have at least one element.");
        }
        self.data.push_back(value);
        self.running_sum = self.running_sum + value;
    }

    pub fn average(&self) -> f64 {
        if self.data.len() > 0 {
            self.running_sum.to_f64().unwrap() / self.data.len() as f64
        } else {
            self.running_sum.to_f64().unwrap()
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let m: MovingAverage<u32> = MovingAverage::new(10);
        assert_eq!(m.len(), 0);
        assert_eq!(m.capacity(), 10);
        assert_eq!(m.average(), 0.0);
    }

    #[test]
    fn test_add() {
        let mut m: MovingAverage<u32> = MovingAverage::new(10);
        m.add(3);
        assert_eq!(m.len(), 1);
        assert_eq!(m.average(), 3.0);
    }

    #[test]
    fn test_add_roll_u32() {
        let mut m: MovingAverage<u32> = MovingAverage::new(2);
        m.add(5);
        assert_eq!(m.len(), 1);
        assert_eq!(m.average(), 5.0);
        m.add(5);
        assert_eq!(m.len(), 2);
        assert_eq!(m.average(), 5.0);
        m.add(1);
        assert_eq!(m.len(), 2);
        assert_eq!(m.average(), 3.0);
    }

    #[test]
    fn test_add_roll_f64() {
        let mut m: MovingAverage<f64> = MovingAverage::new(2);
        m.add(5.0);
        assert_eq!(m.len(), 1);
        assert_eq!(m.average(), 5.0);
        m.add(5.0);
        assert_eq!(m.len(), 2);
        assert_eq!(m.average(), 5.0);
        m.add(1.0);
        assert_eq!(m.len(), 2);
        assert_eq!(m.average(), 3.0);
    }
}
