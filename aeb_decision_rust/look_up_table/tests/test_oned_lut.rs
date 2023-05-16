use look_up_table::OneDLookUpTable;
use rstest::{fixture, rstest};

type IncrFunc = OneDLookUpTable<5>;

#[fixture]
fn simple_increasing_function() -> IncrFunc {
    OneDLookUpTable::new(
        std::array::from_fn(|i| (i + 1) as f64),
        std::array::from_fn(|i| (i + 1) as f64),
    )
    .unwrap()
}

type RandFunc = OneDLookUpTable<6>;

#[fixture]
fn random_function() -> RandFunc {
    OneDLookUpTable::new(
        [1.0, 2.0, 7.0, 9.0, 13.0, 20.0],
        [8.0, 4.0, 6.0, 10.0, 3.0, 2.0],
    )
    .unwrap()
}

#[rstest]
fn when_x_value_is_to_the_right_of_interval_return_last_value_in_y_table(
    simple_increasing_function: IncrFunc,
) {
    let expected = 5.0;
    let actual = simple_increasing_function.get(10.0);

    assert!((actual - expected).abs() < f64::EPSILON);
}

#[rstest]
fn when_x_value_is_to_the_left_of_interval_return_first_value_in_y_table(
    simple_increasing_function: IncrFunc,
) {
    let expected = 1.0;
    let actual = simple_increasing_function.get(-1.0);

    assert!((actual - expected).abs() < f64::EPSILON);
}

#[rstest]
fn when_x_value_is_in_interval_return_appropriate_first_value_in_y_table(
    simple_increasing_function: IncrFunc,
) {
    let expected = 3.0;
    let actual = simple_increasing_function.get(3.0);

    assert!((actual - expected).abs() < f64::EPSILON);
}
