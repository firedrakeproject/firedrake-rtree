pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    fn contains(&self, p: f64) -> bool {
        self.min <= p && p <= self.max
    }
}






#[test]
fn test_interval_contains() {
    let interval = Interval { min: 0.0, max: 1.0 };
    assert!(interval.contains(0.5));
    assert!(interval.contains(0.0));
    assert!(interval.contains(1.0));
    assert!(!interval.contains(-0.1));
    assert!(!interval.contains(1.1));
}