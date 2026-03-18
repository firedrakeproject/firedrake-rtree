#![forbid(unsafe_code)]

#[derive(Debug, PartialEq)]
pub struct IntervalTreeNode {
    pub center: f64,
    pub left: Option<Box<IntervalTreeNode>>,
    pub right: Option<Box<IntervalTreeNode>>,
    pub overlapping_by_min: Vec<(f64, f64, usize)>,
    pub overlapping_by_max: Vec<(f64, f64, usize)>,
}

/// Builds an interval tree node from a list of intervals. Each interval is represented as a tuple of (min, max, id).
///
/// # Panics
///
/// Panics if the input list of intervals is empty.
fn build_node(intervals: Vec<(f64, f64, usize)>) -> IntervalTreeNode {
    // This follows the algorithm described in https://en.wikipedia.org/wiki/Interval_tree
    assert!(
        !intervals.is_empty(),
        "Cannot build an interval tree node from an empty list of intervals"
    );
    let min = intervals.iter().map(|i| i.0).fold(f64::INFINITY, f64::min);
    let max = intervals
        .iter()
        .map(|i| i.1)
        .fold(f64::NEG_INFINITY, f64::max);
    assert!(
        min.is_finite() && max.is_finite(),
        "Interval endpoints must be finite"
    );
    let center = f64::midpoint(min, max);

    let mut s_left = Vec::new();
    let mut s_right = Vec::new();
    let mut s_center = Vec::new();

    for interval in intervals {
        let left = interval.0;
        let right = interval.1;
        assert!(
            left <= right,
            "Invalid interval with min > max: ({left}, {right})",
        );
        if right < center {
            s_left.push(interval);
        } else if left > center {
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
    overlapping_by_min.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut overlapping_by_max = s_center.clone();
    overlapping_by_max.sort_by(|a, b| a.1.total_cmp(&b.1));

    IntervalTreeNode {
        center,
        left,
        right,
        overlapping_by_min,
        overlapping_by_max,
    }
}

#[derive(Debug, PartialEq)]
pub struct IntervalTree {
    root: Option<IntervalTreeNode>,
    size: usize,
}

impl IntervalTree {
    /// Creates an interval tree from the given arrays of minimums, maximums, and ids. The lengths of the input arrays must be the same.
    /// Returns an empty tree if the input arrays are empty.
    ///
    /// # Panics
    ///
    /// Panics if the input arrays have different lengths.
    #[must_use]
    pub fn bulk_load(mins: &[f64], maxs: &[f64], ids: &[usize]) -> Self {
        let n = mins.len();
        assert!(
            n == maxs.len() && n == ids.len(),
            "Inputs must have the same length"
        );
        if n == 0 {
            return Self {
                root: None,
                size: 0,
            };
        }
        let elements: Vec<(f64, f64, usize)> = (0..n).map(|i| (mins[i], maxs[i], ids[i])).collect();
        Self {
            root: Some(build_node(elements)),
            size: n,
        }
    }

    /// Locates all intervals that contain the given point `p`. Returns a vector of the ids of the matching intervals.
    /// Returns an empty vector if no intervals contain the point.
    #[must_use]
    pub fn locate_all_at_point(&self, p: f64) -> Vec<usize> {
        // Pre-order traversal of the interval tree
        let mut result = Vec::new();
        let mut stack = Vec::new();
        if let Some(root) = &self.root {
            stack.push(root);
        }
        while let Some(node) = stack.pop() {
            if p < node.center {
                if let Some(left) = &node.left {
                    stack.push(left);
                }
                for interval in &node.overlapping_by_min {
                    if p < interval.0 {
                        continue;
                    }
                    result.push(interval.2);
                }
            } else {
                if let Some(right) = &node.right {
                    stack.push(right);
                }
                for interval in &node.overlapping_by_max {
                    if p > interval.1 {
                        continue;
                    }
                    result.push(interval.2);
                }
            }
        }
        result
    }

    /// Returns the number of intervals in the tree.
    #[must_use]
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns a reference to the root node of the tree, or `None` if the tree is empty.
    #[must_use]
    pub fn root(&self) -> Option<&IntervalTreeNode> {
        self.root.as_ref()
    }
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
    assert_eq!(
        node.overlapping_by_min,
        vec![(-1.0, 0.5, 3), (0.0, 1.0, 0),]
    );
    assert_eq!(
        node.overlapping_by_max,
        vec![(-1.0, 0.5, 3), (0.0, 1.0, 0),]
    );
    let left_node = node.left.as_ref().unwrap();
    assert_eq!(left_node.left, None);
    assert_eq!(left_node.right, None);
    assert_eq!(left_node.center, -1.5);
    assert_eq!(left_node.overlapping_by_min, vec![(-2.0, -1.0, 4),]);
    assert_eq!(left_node.overlapping_by_max, vec![(-2.0, -1.0, 4),]);
    let right_node = node.right.as_ref().unwrap();
    assert_eq!(right_node.left, None);
    assert_eq!(right_node.right, None);
    assert_eq!(right_node.center, 1.25);
    assert_eq!(
        right_node.overlapping_by_min,
        vec![(0.5, 1.5, 1), (1.0, 2.0, 2),]
    );
    assert_eq!(
        right_node.overlapping_by_max,
        vec![(0.5, 1.5, 1), (1.0, 2.0, 2),]
    );
}

#[test]
fn test_interval_tree_bulk_load() {
    let mins = vec![0.0, 0.5, 1.0, -1.0, -2.0];
    let maxs = vec![1.0, 1.5, 2.0, 0.5, -1.0];
    let ids = vec![0, 1, 2, 3, 4];
    let tree = IntervalTree::bulk_load(&mins, &maxs, &ids);
    assert_eq!(tree.size, 5);
    let root = tree.root().unwrap();
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
    assert_eq!(tree.size(), 0);
    assert_eq!(tree.root(), None);
}

#[test]
fn test_interval_tree_locate_all_at_point() {
    let mins = vec![0.0, 0.5, 1.0, -1.0, -2.0];
    let maxs = vec![1.0, 1.5, 2.0, 0.5, -1.0];
    let ids = vec![0, 1, 2, 3, 4];
    let tree = IntervalTree::bulk_load(&mins, &maxs, &ids);
    assert_eq!(tree.size(), 5);
    let result = tree.locate_all_at_point(0.25);
    assert_eq!(result.len(), 2);
    assert!(result.contains(&0));
    assert!(result.contains(&3));
    let result = tree.locate_all_at_point(0.75);
    assert_eq!(result.len(), 2);
    assert!(result.contains(&0));
    assert!(result.contains(&1));
    let result = tree.locate_all_at_point(0.5);
    assert_eq!(result.len(), 3);
    assert!(result.contains(&0));
    assert!(result.contains(&1));
    assert!(result.contains(&3));
    let result = tree.locate_all_at_point(1.25);
    assert_eq!(result.len(), 2);
    assert!(result.contains(&1));
    assert!(result.contains(&2));
    let result = tree.locate_all_at_point(-1.5);
    assert_eq!(result.len(), 1);
    assert!(result.contains(&4));
    // test points on the boundaries of intervals
    let result = tree.locate_all_at_point(0.0);
    assert_eq!(result.len(), 2);
    assert!(result.contains(&0));
    assert!(result.contains(&3));
    let result = tree.locate_all_at_point(1.0);
    assert_eq!(result.len(), 3);
    assert!(result.contains(&0));
    assert!(result.contains(&1));
    assert!(result.contains(&2));
    // test points outside all intervals
    let result = tree.locate_all_at_point(2.5);
    assert_eq!(result.len(), 0);
    let result = tree.locate_all_at_point(-3.0);
    assert_eq!(result.len(), 0);
}

#[test]
fn test_interval_tree_locate_all_at_point_empty_tree() {
    let mins = vec![];
    let maxs = vec![];
    let ids = vec![];
    let tree = IntervalTree::bulk_load(&mins, &maxs, &ids);
    let result = tree.locate_all_at_point(0.0);
    assert_eq!(result.len(), 0);
}

#[test]
fn test_interval_tree_locate_all_at_point_single_interval() {
    let mins = vec![0.0];
    let maxs = vec![1.0];
    let ids = vec![0];
    let tree = IntervalTree::bulk_load(&mins, &maxs, &ids);
    assert_eq!(tree.size(), 1);
    let result = tree.locate_all_at_point(0.5);
    assert_eq!(result.len(), 1);
    assert!(result.contains(&0));
    let result = tree.locate_all_at_point(-0.5);
    assert_eq!(result.len(), 0);
    let result = tree.locate_all_at_point(1.5);
    assert_eq!(result.len(), 0);
    let result = tree.locate_all_at_point(0.0);
    assert_eq!(result.len(), 1);
    assert!(result.contains(&0));
    let result = tree.locate_all_at_point(1.0);
    assert_eq!(result.len(), 1);
    assert!(result.contains(&0));
    let root_node = tree.root().unwrap();
    assert_eq!(root_node.center, 0.5);
    assert_eq!(root_node.left, None);
    assert_eq!(root_node.right, None);
}

#[test]
fn test_interval_tree_degenerate_intervals() {
    let mins = vec![0.0, 0.0, 0.0, 0.0, 0.0];
    let maxs = vec![1.0, 1.0, 1.0, 1.0, 1.0];
    let ids = vec![0, 1, 2, 3, 4];
    let tree = IntervalTree::bulk_load(&mins, &maxs, &ids);
    assert_eq!(tree.size(), 5);
    let root = tree.root().unwrap();
    assert_eq!(root.center, 0.5);
    assert_eq!(root.left, None);
    assert_eq!(root.right, None);
    let result = tree.locate_all_at_point(0.5);
    assert_eq!(result.len(), 5);
    assert!(result.contains(&0));
    assert!(result.contains(&1));
    assert!(result.contains(&2));
    assert!(result.contains(&3));
    assert!(result.contains(&4));
    let result = tree.locate_all_at_point(0.0);
    assert_eq!(result.len(), 5);
    let result = tree.locate_all_at_point(1.0);
    assert_eq!(result.len(), 5);
    let result = tree.locate_all_at_point(-0.5);
    assert_eq!(result.len(), 0);
    let result = tree.locate_all_at_point(1.5);
    assert_eq!(result.len(), 0);
}

#[test]
fn test_interval_tree_invalid_interval() {
    let mins = vec![0.0, 1.0];
    let maxs = vec![1.0, 0.0];
    let ids = vec![0, 1];
    std::panic::catch_unwind(|| IntervalTree::bulk_load(&mins, &maxs, &ids))
        .expect_err("Expected panic due to invalid interval with min > max");
}

#[test]
fn test_interval_tree_nan() {
    let mins = vec![0.0, 2.0];
    let maxs = vec![1.0, f64::NAN];
    let ids = vec![0, 1];
    std::panic::catch_unwind(|| IntervalTree::bulk_load(&mins, &maxs, &ids))
        .expect_err("Expected panic due to invalid interval with min > max");
}

#[test]
fn test_interval_tree_infinite() {
    let mins = vec![f64::NEG_INFINITY, 1.0, f64::NEG_INFINITY];
    let maxs = vec![1.0, f64::INFINITY, f64::INFINITY];
    let ids = vec![0, 1, 2];
    std::panic::catch_unwind(|| IntervalTree::bulk_load(&mins, &maxs, &ids))
        .expect_err("Expected panic due to invalid interval with infinite endpoints");
}

#[test]
fn test_interval_tree_duplicate_ids() {
    let mins = vec![0.0, 0.5, 0.0, 0.0];
    let maxs = vec![1.0, 1.5, 2.0, 1.0];
    let ids = vec![0, 1, 1, 0];
    let tree = IntervalTree::bulk_load(&mins, &maxs, &ids);
    assert_eq!(tree.size(), 4);
    let result = tree.locate_all_at_point(0.75);
    assert_eq!(result.len(), 4);
    assert!(result.contains(&0));
    assert!(result.contains(&1));
}
