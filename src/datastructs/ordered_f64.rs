/// A lightweight wrapper around a `f64` for use when
/// order (`Ord`) is required.
///
/// Using a this structure excludes `f64` 'NaN' values
/// from appearing thus allowing `Ord` to be implemented
/// which is required when sorting is needed
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
struct OrderedF64 {
    float: f64,
}

impl Eq for OrderedF64 {}

impl Ord for OrderedF64 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.float
            .partial_cmp(&other.float)
            .expect("NaN values are not allowed.")
    }
}

#[allow(dead_code)]
impl OrderedF64 {
    /// create a new OrderedF64, ensuring no NaNs are inserted.
    fn new(value: f64) -> Self {
        if value.is_nan() {
            panic!("NaN values are not allowed.");
        }
        OrderedF64 { float: value }
    }
}
