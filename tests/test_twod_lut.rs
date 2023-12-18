use look_up_table::TwoDLookUpTable;
use rstest::{fixture, rstest};
type IncrSurface = TwoDLookUpTable<5, 5>;

#[fixture]
fn simple_increasing_surface() -> IncrSurface {
    TwoDLookUpTable::new(
        std::array::from_fn(|i| (i + 1) as f64),
        std::array::from_fn(|i| (i + 1) as f64),
        [
            [1.0, 2.0, 3.0, 4.0, 5.0],
            [6.0, 7.0, 8.0, 9.0, 10.0],
            [11.0, 12.0, 13.0, 14.0, 15.0],
            [16.0, 17.0, 18.0, 19.0, 20.0],
            [21.0, 22.0, 23.0, 24.0, 25.0],
        ],
    )
    .unwrap()
}

// All the corner out of bounds should produce corner values.
#[rstest]
#[case(-2.0, -3.0, 1.0)]
#[case(7.0, -1.0, 5.0)]
#[case(10.0, 10.0, 25.0)]
#[case(-3.0, 8.0, 21.0)]
fn when_x_y_values_are_out_of_bounds_return_corner_values(
    simple_increasing_surface: IncrSurface,
    #[case] x: f64,
    #[case] y: f64,
    #[case] expected: f64,
) {
    let actual = simple_increasing_surface.get(&x, &y);

    assert!((actual - expected).abs() < 0.000001);
}

#[test]
#[should_panic(expected = "X and Y values should be in strictly increasing order")]
fn when_x_or_y_values_are_not_increasing_dont_construct_object() {
    TwoDLookUpTable::new([3.0, 1.0, 2.0], [1.0; 2], [[1.0; 2]; 3]).unwrap();
}

#[test]
#[should_panic(expected = "At least two values should be provided for x and y axes")]
fn when_x_or_y_values_are_single_element_arrays_dont_construct_object() {
    TwoDLookUpTable::new([1.0], [0.0], [[0.0]; 1]).unwrap();
}

#[test]
#[should_panic(expected = "Cannot create a Lookup Table containing NaNs or Infinities")]
fn when_x_or_y_surface_values_contain_nans_or_infinities_dont_construct_object() {
    TwoDLookUpTable::new([f64::NAN, 1.0, 2.0], [f64::NEG_INFINITY; 3], [[1.0; 3]; 3]).unwrap();
}

#[test]
fn when_x_y_values_are_within_bounds_then_perform_bilinear_interpolation() {
    let lut = TwoDLookUpTable::new([14.0, 15.0], [20.0, 21.0], [[91.0, 210.0], [162.0, 95.0]]).unwrap();

    let expected = 131.7;
    let actual = lut.get(&14.5, &20.2);

    assert!((actual - expected).abs() < 0.00001);
}

// TODO: Write tests for interpolation in single direction.
