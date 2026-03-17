pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    fn contains(&self, p: f64) -> bool {
        self.min <= p && p <= self.max
    }
}

#[derive(Debug, PartialEq)]
pub struct NodeInterval {
    pub center: f64,
    pub left: Option<Box<NodeInterval>>,
    pub right: Option<Box<NodeInterval>>,
    pub overlapping_by_min: Vec<(f64, f64, usize)>,
    pub overlapping_by_max: Vec<(f64, f64, usize)>,
}

// This follows the algorithm described in https://en.wikipedia.org/wiki/Interval_tree
fn build_node(intervals: Vec<(f64, f64, usize)>) -> NodeInterval {
    assert!(!intervals.is_empty(), "Cannot build an interval tree node from an empty list of intervals");
    let min = intervals.iter().map(|i| i.0).fold(f64::INFINITY, f64::min);
    let max = intervals.iter().map(|i| i.1).fold(f64::NEG_INFINITY, f64::max);
    let center = (min + max) / 2.0;

    let mut s_left = Vec::new();
    let mut s_right = Vec::new();
    let mut s_center = Vec::new();

    for interval in intervals {
        if interval.1 < center {
            s_left.push(interval);
        } else if interval.0 > center {
            s_right.push(interval);
        } else {
            s_center.push(interval);
        }
    }

    let left = if s_left.is_empty() {
        None
    } else {
        Some(Box::new(build_node(s_left)))
    };
    let right = if s_right.is_empty() {
        None
    } else {
        Some(Box::new(build_node(s_right)))
    };

    let mut overlapping_by_min = s_center.clone();
    overlapping_by_min.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let mut overlapping_by_max = s_center.clone();
    overlapping_by_max.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    NodeInterval {
        center,
        left,
        right,
        overlapping_by_min,
        overlapping_by_max,
    }
}


pub struct IntervalTree {
    root: Option<NodeInterval>,
    size: usize,
}


impl IntervalTree {
    pub fn bulk_load(
        mins: &[f64],
        maxs: &[f64],
        ids: &[usize],
    ) -> Self {
        let n = mins.len();
        assert!(
            n == maxs.len() && n == ids.len(),
            "Inputs must have the same length"
        );
        if n == 0 {
            return Self { root: None, size: 0 };
        }
        let elements: Vec<(f64, f64, usize)> = (0..n).map(|i| (mins[i], maxs[i], ids[i])).collect();
        Self {
            root: Some(build_node(elements)),
            size: n,
        }
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

#[test]
fn test_build_node() {
    let intervals = vec![
        (0.0, 1.0, 0),
        (0.5, 1.5, 1),
        (1.0, 2.0, 2),
        (-1.0, 0.5, 3),
        (-2.0, -1.0, 4),
    ];
    let node = build_node(intervals);
    assert_eq!(node.center, 0.0);
    assert_eq!(node.overlapping_by_min, vec![
        (-1.0, 0.5, 3),
        (0.0, 1.0, 0),
    ]);
    assert_eq!(node.overlapping_by_max, vec![
        (-1.0, 0.5, 3),
        (0.0, 1.0, 0),
    ]);
    let left_node = node.left.as_ref().unwrap();
    assert_eq!(left_node.left, None);
    assert_eq!(left_node.right, None);
    assert_eq!(left_node.center, -1.5);
    assert_eq!(left_node.overlapping_by_min, vec![
        (-2.0, -1.0, 4),
    ]);
    assert_eq!(left_node.overlapping_by_max, vec![
        (-2.0, -1.0, 4),
    ]);
    let right_node = node.right.as_ref().unwrap();
    assert_eq!(right_node.left, None);
    assert_eq!(right_node.right, None);
    assert_eq!(right_node.center, 1.25);
    assert_eq!(right_node.overlapping_by_min, vec![
        (0.5, 1.5, 1),
        (1.0, 2.0, 2),
    ]);
    assert_eq!(right_node.overlapping_by_max, vec![
        (0.5, 1.5, 1),
        (1.0, 2.0, 2),
    ]);
}

#[test]
fn test_interval_tree_bulk_load() {
    let mins = vec![0.0, 0.5, 1.0, -1.0, -2.0];
    let maxs = vec![1.0, 1.5, 2.0, 0.5, -1.0];
    let ids = vec![0, 1, 2, 3, 4];
    let tree = IntervalTree::bulk_load(&mins, &maxs, &ids);
    assert_eq!(tree.size, 5);
    let root = tree.root.as_ref().unwrap();
    assert_eq!(root.center, 0.0);
    let left_node = root.left.as_ref().unwrap();
    assert_eq!(left_node.center, -1.5);
    assert_eq!(left_node.left, None);
    assert_eq!(left_node.right, None);
    let right_node = root.right.as_ref().unwrap();
    assert_eq!(right_node.center, 1.25);
    assert_eq!(right_node.left, None);
    assert_eq!(right_node.right, None);
}

#[test]
fn test_interval_tree_empty_bulk_load() {
    let mins = vec![];
    let maxs = vec![];
    let ids = vec![];
    let tree = IntervalTree::bulk_load(&mins, &maxs, &ids);
    assert_eq!(tree.size, 0);
    assert_eq!(tree.root, None);
}
