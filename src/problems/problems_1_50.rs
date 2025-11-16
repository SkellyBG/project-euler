use std::{cmp::max, collections::HashMap};

use itertools::Itertools;
use project_euler::{is_palindrome, is_prime, prime_factorise};

use super::Problem;

pub struct Problem5;

impl Problem for Problem5 {
    fn solve(&self) -> String {
        let map = (1..=20)
            .flat_map(prime_factorise)
            .fold(HashMap::new(), |mut map, factors| {
                map.entry(factors.0)
                    .and_modify(|entry| *entry = max(*entry, factors.1))
                    .or_insert(factors.1);

                map
            });

        map.iter()
            .map(|(a, b)| a.pow(*b))
            .product::<i64>()
            .to_string()
    }
}

pub struct Problem4;

impl Problem for Problem4 {
    fn solve(&self) -> String {
        (100..=999)
            .cartesian_product(100..=999)
            .map(|(a, b)| a * b)
            .filter(|product| is_palindrome(&product.to_string()))
            .max()
            .unwrap()
            .to_string()
    }
}

pub struct Problem3;

impl Problem for Problem3 {
    fn solve(&self) -> String {
        let num: i64 = 600851475143;
        let largest_factor = (1..num.isqrt())
            .filter(|factor| num % factor == 0)
            .filter(|factor| is_prime(*factor))
            .next_back()
            .unwrap_or(1);

        largest_factor.to_string()
    }
}
pub struct Problem2;

impl Problem for Problem2 {
    fn solve(&self) -> String {
        let mut sum = 0;
        let mut first = 1;
        let mut second = 1;

        loop {
            let cur = first + second;
            if cur > 4000000 {
                break;
            }

            if cur % 2 == 0 {
                sum += cur;
            }

            first = second;
            second = cur;
        }

        sum.to_string()
    }
}

pub struct Problem1;

impl Problem for Problem1 {
    fn solve(&self) -> String {
        let sum: u32 = (1..1000).filter(|n| n % 3 == 0 || n % 5 == 0).sum();

        sum.to_string()
    }
}
