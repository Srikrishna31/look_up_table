// #![feature(test)]
// extern crate test;
// use test::Bencher;

use look_up_table::OneDLookUpTable;
use rstest::{fixture, rstest};

type IncrFunc = OneDLookUpTable<5>;

#[fixture]
fn simple_increasing_function() -> IncrFunc {
    OneDLookUpTable::new(std::array::from_fn(|i| (i + 1) as f64), std::array::from_fn(|i| (i + 1) as f64)).unwrap()
}

type RandFunc = OneDLookUpTable<6>;

#[fixture]
fn random_function() -> RandFunc {
    OneDLookUpTable::new([1.0, 2.0, 7.0, 9.0, 13.0, 20.0], [8.0, 4.0, 6.0, 10.0, 3.0, 2.0]).unwrap()
}

#[rstest]
fn when_x_value_is_to_the_right_of_interval_return_last_value_in_y_table(simple_increasing_function: IncrFunc) {
    let expected = 5.0;
    let actual = simple_increasing_function.get(&10.0);

    assert!((actual - expected).abs() < f64::EPSILON);
}

#[rstest]
fn when_x_value_is_to_the_left_of_interval_return_first_value_in_y_table(simple_increasing_function: IncrFunc) {
    let expected = 1.0;
    let actual = simple_increasing_function.get(&-1.0);

    assert!((actual - expected).abs() < f64::EPSILON);
}

#[rstest]
fn when_x_value_is_in_interval_return_appropriate_first_value_in_y_table(simple_increasing_function: IncrFunc) {
    let expected = 3.0;
    let actual = simple_increasing_function.get(&3.0);

    assert!((actual - expected).abs() < f64::EPSILON);
}

#[test]
#[should_panic(expected = "X values should be in strictly increasing order")]
fn when_x_values_are_same_dont_construct_object() {
    OneDLookUpTable::new([2.0; 6], [2.0; 6]).unwrap();
}

#[test]
#[should_panic(expected = "At least two values should be provided")]
fn when_given_single_x_value_dont_construct_object() {
    OneDLookUpTable::new([2.6; 1], [3.2; 1]).unwrap();
}

#[rstest]
#[should_panic(expected = "Cannot create a Lookup Table containing NaNs or Infinities")]
fn when_x_or_y_values_contain_nan_or_infinities_dont_construct_object(
    #[values([f64::NAN;5], [0.0, 1.5, f64::INFINITY, 4.5, 2.3], [1.1, 2.2, 3.3, 4.4, 5.5])] x: [f64; 5],
    #[values([f64::NAN;5], [0.0, 0.0, 0.0, f64::NAN, 0.05], [0.0, 1.5, f64::INFINITY, f64::INFINITY, f64::NAN])]
    y: [f64; 5],
) {
    OneDLookUpTable::new(x, y).unwrap();
}

#[rstest]
#[case(10.0, 8.25)]
#[case(16.0, 2.5714)]
fn when_x_is_in_range_return_interpolated_value(random_function: RandFunc, #[case] input: f64, #[case] expected: f64) {
    let actual = random_function.get(&input);
    assert!((actual - expected).abs() < 0.0001);
}

// Currently benchmarking is not supported on stable channel.
// #[bench]
// fn bench_when_same_x_value_is_queried_lookup_should_be_constant(b: &mut Bencher) {
//     // The idea behind this test is that, when we query the same value again and again, the lookup
//     // cost should approach constant value.
//     let mut rand_func = random_function();
//
//     b.iter(move || rand_func.get(16.0));
// }
