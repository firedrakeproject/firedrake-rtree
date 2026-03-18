use super::*;

use rand::rngs::SmallRng;
use rand::{RngExt, SeedableRng};

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
#[should_panic(expected = "Cannot build an interval tree node from an empty list of intervals")]
fn test_build_node_empty() {
    let _ = build_node(vec![]);
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
#[should_panic(expected = "Inputs must have the same length")]
fn test_interval_tree_mismatched_input_lengths() {
    let mins = vec![0.0, 0.5];
    let maxs = vec![1.0];
    let ids = vec![0, 1];
    let _ = IntervalTree::bulk_load(&mins, &maxs, &ids);
}

#[test]
#[should_panic(expected = "Invalid interval with min > max: (1, 0)")]
fn test_interval_tree_invalid_interval() {
    let mins = vec![0.0, 1.0];
    let maxs = vec![1.0, 0.0];
    let ids = vec![0, 1];
    let _ = IntervalTree::bulk_load(&mins, &maxs, &ids);
}

#[test]
#[should_panic(expected = "Invalid interval with min > max: (2, NaN)")]
fn test_interval_tree_nan() {
    let mins = vec![0.0, 2.0];
    let maxs = vec![1.0, f64::NAN];
    let ids = vec![0, 1];
    let _ = IntervalTree::bulk_load(&mins, &maxs, &ids);
}

#[test]
#[should_panic(expected = "Interval endpoints must be finite")]
fn test_interval_tree_infinite() {
    let mins = vec![f64::NEG_INFINITY, 1.0, f64::NEG_INFINITY];
    let maxs = vec![1.0, f64::INFINITY, f64::INFINITY];
    let ids = vec![0, 1, 2];
    let _ = IntervalTree::bulk_load(&mins, &maxs, &ids);
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

#[test]
fn test_interval_tree_large_nonoverlapping() {
    let n = 1_000_000;
    let mins: Vec<f64> = (0..n).map(|i| i as f64 * 2.0).collect();
    let maxs: Vec<f64> = (0..n).map(|i| i as f64 * 2.0 + 1.0).collect();
    let ids: Vec<usize> = (0..n).collect();
    let tree = IntervalTree::bulk_load(&mins, &maxs, &ids);
    assert_eq!(tree.size(), n);

    let mut rng = SmallRng::seed_from_u64(0);
    for _ in 0..1_000_000 {
        let p: f64 = rng.random_range(0.0..200_000.0);
        let result = tree.locate_all_at_point(p);
        assert!(result.len() <= 1);

        if let Some(&id) = result.first() {
            let expected_min = id as f64 * 2.0;
            let expected_max = id as f64 * 2.0 + 1.0;
            assert!(p >= expected_min && p <= expected_max);
        }
    }
}

#[test]
fn test_interval_tree_node_envelope() {
    let mins = vec![0.0, 0.5, 1.0, -1.0, -2.0];
    let maxs = vec![1.0, 1.5, 2.0, 0.5, -1.0];
    let ids = vec![0, 1, 2, 3, 4];
    let tree = IntervalTree::bulk_load(&mins, &maxs, &ids);
    let root = tree.root().unwrap();
    assert_eq!(root.min, -2.0);
    assert_eq!(root.max, 2.0);
    let left_node = root.left.as_ref().unwrap();
    assert_eq!(left_node.min, -2.0);
    assert_eq!(left_node.max, -1.0);
    let right_node = root.right.as_ref().unwrap();
    assert_eq!(right_node.min, 0.5);
    assert_eq!(right_node.max, 2.0);
}
