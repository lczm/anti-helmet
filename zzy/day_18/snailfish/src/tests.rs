//!
//! Anti Helmet
//! Advent of Code
//! Day 18: Snailfish
//! Unit Tests
//!

use super::*;

#[test]
fn test_number_parse() {
    let str_exprs = ["9", "[1,2]", "[[[1,2],3],[5,6]]"];

    use Number::*;
    let expected_nums = [
        Regular(9),
        Pair(Box::new(Regular(1)), Box::new(Regular(2))),
        Pair(
            Box::new(Pair(
                Box::new(Pair(Box::new(Regular(1)), Box::new(Regular(2)))),
                Box::new(Regular(3)),
            )),
            Box::new(Pair(Box::new(Regular(5)), Box::new(Regular(6)))),
        ),
    ];

    for (str_expr, expected) in str_exprs.iter().zip(expected_nums.iter()) {
        assert_eq!(Number::parse(str_expr), *expected);
    }
}

#[test]
fn test_number_explode() {
    let explode_cases: Vec<_> = [
        ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
        ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
        ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
    ]
    .iter()
    .map(|(input, expected)| (Number::parse(input), Number::parse(expected)))
    .collect();

    for (num, expected) in explode_cases.into_iter() {
        let result = num.explode(0);
        assert!(result.is_reduced);
        assert_eq!(result.number, expected);
    }
}

#[test]
fn test_number_split() {
    let split_cases: Vec<_> = [
        ("[10,2]", "[[5,5],2]"),
        ("[2,11]", "[2,[5,6]]"),
        ("[10,11]", "[[5,5],11]"),
    ]
    .iter()
    .map(|(input, expected)| (Number::parse(input), Number::parse(expected)))
    .collect();

    for (num, expected) in split_cases.into_iter() {
        let (actual, is_reduced) = num.split();
        assert!(is_reduced);
        assert_eq!(actual, expected);
    }
}

#[test]
fn test_number_reduce() {
    let num = Number::parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
    let expected = Number::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    assert_eq!(num.reduce(), expected);
}

#[test]
fn test_number_magnitude() {
    assert_eq!(
        Number::parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").magnitude(),
        3488
    );
}
