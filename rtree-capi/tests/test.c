#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

#include "rtree-capi.h"


bool test_create_and_free(void) {
    RTreeH *tree = NULL;
    const uint32_t dim = 2;
    rtree_create(&tree, dim);
    if (tree == NULL) {
        return false;
    }
    rtree_free(tree);
    return true;
}


bool test_null(void) {
    // Test that passing null pointers returns NullPointer error
    RTreeError err = rtree_create(NULL, 2);
    if (err != NullPointer) {
        fprintf(stderr, "Expected NullPointer error for rtree_create with null pointer\n");
        return false;
    }

    err = rtree_free(NULL);
    if (err != NullPointer) {
        fprintf(stderr, "Expected NullPointer error for rtree_free with null pointer\n");
        return false;
    }

    return true;
}

bool test_invalid_dimension(void) {
    RTreeH *tree = NULL;
    RTreeError err = rtree_create(&tree, 0);
    if (err != InvalidDimension) {
        fprintf(stderr, "Expected InvalidDimension error for rtree_create with dimension 0\n");
        return false;
    }
    err = rtree_create(&tree, 4);
    if (err != InvalidDimension) {
        fprintf(stderr, "Expected InvalidDimension error for rtree_create with dimension 4\n");
        return false;
    }
    return true;
}


bool test_get_dimension(void) {
    RTreeH *tree3d = NULL;
    const uint32_t dim3d = 3;
    rtree_create(&tree3d, dim3d);
    if (tree3d == NULL) {
        return false;
    }
    uint32_t got_dim3d = 0;
    rtree_get_dimension(tree3d, &got_dim3d);
    rtree_free(tree3d);
    if (got_dim3d != dim3d) {
        fprintf(stderr, "Expected dimension %u, got %u\n", dim3d, got_dim3d);
        return false;
    }

    RTreeH *tree2d = NULL;
    const uint32_t dim2d = 2;
    rtree_create(&tree2d, dim2d);
    if (tree2d == NULL) {
        return false;
    }
    uint32_t got_dim2d = 0;
    rtree_get_dimension(tree2d, &got_dim2d);
    rtree_free(tree2d);
    if (got_dim2d != dim2d) {
        fprintf(stderr, "Expected dimension %u, got %u\n", dim2d, got_dim2d);
        return false;
    }
    return true;
}


bool test_bulk_load(void) {
    const size_t N = 2;
    const uint32_t dim = 2;
    double mins[4] = {0.0, 0.0, 1.0, 1.0};
    double maxs[4] = {2.0, 2.0, 3.0, 3.0};
    int64_t ids[2] = {1, 2};
    RTreeH *tree = NULL;
    rtree_bulk_load(&tree, mins, maxs, ids, N, dim);
    if (tree == NULL) {
        return false;
    }

    // test rtree_size
    size_t size = 0;
    rtree_size(tree, &size);
    if (size != N) {
        fprintf(stderr, "Expected tree size %zu, got %zu\n", N, size);
        rtree_free(tree);
        return false;
    }

    size_t depth = 0;
    rtree_depth(tree, &depth);
    if (depth != 1) {
        fprintf(stderr, "Expected tree depth 1, got %zu\n", depth);
        rtree_free(tree);
        return false;
    }

    double point1[2] = {1.5, 1.5};
    double point2[2] = {0.0, 0.0};
    double point3[2] = {-1.0, 0.0};

    int64_t *ids_out1 = NULL;
    size_t nids_out1 = 0;
    rtree_locate_all_at_point(tree, point1, &ids_out1, &nids_out1);
    if (nids_out1 != 2 || ids_out1[0] != 2 || ids_out1[1] != 1) {
        fprintf(stderr, "Expected to find ids [2, 1] at point1");
        rtree_free_ids(ids_out1, nids_out1);
        rtree_free(tree);
        return false;
    } else {
        rtree_free_ids(ids_out1, nids_out1);
    }

    int64_t *ids_out2 = NULL;
    size_t nids_out2 = 0;
    rtree_locate_all_at_point(tree, point2, &ids_out2, &nids_out2);
    if (nids_out2 != 1 || ids_out2[0] != 1) {
        fprintf(stderr, "Expected to find id [1] at point2");
        rtree_free_ids(ids_out2, nids_out2);
        rtree_free(tree);
        return false;
    } else {
        rtree_free_ids(ids_out2, nids_out2);
    }

    int64_t *ids_out3 = NULL;
    size_t nids_out3 = 0;
    rtree_locate_all_at_point(tree, point3, &ids_out3, &nids_out3);
    if (nids_out3 != 0) {
        fprintf(stderr, "Expected to find no ids at point3");
        rtree_free_ids(ids_out3, nids_out3);
        rtree_free(tree);
        return false;
    } else {
        rtree_free_ids(ids_out3, nids_out3);
    }

    rtree_free(tree);
    return true;
}


bool test_nodes(void) {
    const size_t N = 2;
    const uint32_t dim = 2;
    double mins[4] = {0.0, 0.0, 1.0, 1.0};
    double maxs[4] = {2.0, 2.0, 3.0, 3.0};
    int64_t ids[2] = {1, 2};
    RTreeH *tree = NULL;
    rtree_bulk_load(&tree, mins, maxs, ids, N, dim);
    if (tree == NULL) {
        return false;
    }

    RTreeNodeH *root = NULL;
    rtree_root_node(tree, &root);
    if (root == NULL) {
        rtree_free(tree);
        return false;
    }

    double root_min[2];
    double root_max[2];
    rtree_node_envelope(root, root_min, root_max);
    if (root_min[0] != 0.0 || root_min[1] != 0.0 || root_max[0] != 3.0 || root_max[1] != 3.0) {
        fprintf(stderr, "Expected root envelope to be [0, 0], [3, 3], got [%f, %f], [%f, %f]\n",
            root_min[0], root_min[1], root_max[0], root_max[1]);
        rtree_node_free(root);
        rtree_free(tree);
        return false;
    }

    double *root_mins = NULL;
    double *root_maxs = NULL;
    size_t nboxes_root = 0;
    RTreeError err = rtree_collect_bounding_boxes(tree, 0, &root_mins, &root_maxs, &nboxes_root);
    if (err != Success || nboxes_root != 1) {
        fprintf(stderr, "Expected 1 collected bounding box at level 0, got %zu\n", nboxes_root);
        rtree_free_bounding_boxes(root_mins, root_maxs, nboxes_root, dim);
        rtree_node_free(root);
        rtree_free(tree);
        return false;
    }
    if (root_mins[0] != root_min[0] || root_mins[1] != root_min[1] ||
        root_maxs[0] != root_max[0] || root_maxs[1] != root_max[1]) {
        fprintf(stderr, "Expected collected bounding box at level 0 to match root envelope\n");
        rtree_free_bounding_boxes(root_mins, root_maxs, nboxes_root, dim);
        rtree_node_free(root);
        rtree_free(tree);
        return false;
    }
    rtree_free_bounding_boxes(root_mins, root_maxs, nboxes_root, dim);

    RTreeNodeH **children = NULL;
    size_t nchildren = 0;
    rtree_node_children(root, &children, &nchildren);
    if (nchildren != 2) {
        fprintf(stderr, "Expected root to have 2 children, got %zu\n", nchildren);
        rtree_node_children_free(children, nchildren);
        rtree_node_free(root);
        rtree_free(tree);
        return false;
    }

    double child1_min[2];
    double child1_max[2];
    rtree_node_envelope(children[0], child1_min, child1_max);
    if (child1_min[0] != 0.0 || child1_min[1] != 0.0 || child1_max[0] != 2.0 || child1_max[1] != 2.0) {
        fprintf(stderr, "Expected child1 envelope to be [0, 0], [2, 2], got [%f, %f], [%f, %f]\n",
            child1_min[0], child1_min[1], child1_max[0], child1_max[1]);
        rtree_node_children_free(children, nchildren);
        rtree_node_free(root);
        rtree_free(tree);
        return false;
    }
    double child2_min[2];
    double child2_max[2];
    rtree_node_envelope(children[1], child2_min, child2_max);
    if (child2_min[0] != 1.0 || child2_min[1] != 1.0 || child2_max[0] != 3.0 || child2_max[1] != 3.0) {
        fprintf(stderr, "Expected child2 envelope to be [1, 1], [3, 3], got [%f, %f], [%f, %f]\n",
            child2_min[0], child2_min[1], child2_max[0], child2_max[1]);
        rtree_node_children_free(children, nchildren);
        rtree_node_free(root);
        rtree_free(tree);
        return false;
    }

    double *collected_mins = NULL;
    double *collected_maxs = NULL;
    size_t nboxes = 0;
    RTreeError err1 = rtree_collect_bounding_boxes(tree, 1, &collected_mins, &collected_maxs, &nboxes);
    if (err1 != Success || nboxes != nchildren) {
        fprintf(stderr, "Expected %zu collected bounding boxes at level 1, got %zu\n",
            nchildren, nboxes);
        rtree_free_bounding_boxes(collected_mins, collected_maxs, nboxes, dim);
        rtree_node_children_free(children, nchildren);
        rtree_node_free(root);
        rtree_free(tree);
        return false;
    }
    bool found_child1 = false;
    bool found_child2 = false;
    for (size_t i = 0; i < nboxes; i++) {
        if (collected_mins[2*i] == child1_min[0] &&
            collected_mins[2*i + 1] == child1_min[1] &&
            collected_maxs[2*i] == child1_max[0] &&
            collected_maxs[2*i + 1] == child1_max[1]) {
            found_child1 = true;
        }
        if (collected_mins[2*i] == child2_min[0] &&
            collected_mins[2*i + 1] == child2_min[1] &&
            collected_maxs[2*i] == child2_max[0] &&
            collected_maxs[2*i + 1] == child2_max[1]) {
            found_child2 = true;
        }
    }
    if (!found_child1 || !found_child2) {
        fprintf(stderr, "Expected collected bounding boxes to contain both child envelopes\n");
        rtree_free_bounding_boxes(collected_mins, collected_maxs, nboxes, dim);
        rtree_node_children_free(children, nchildren);
        rtree_node_free(root);
        rtree_free(tree);
        return false;
    }
    rtree_free_bounding_boxes(collected_mins, collected_maxs, nboxes, dim);

    RTreeNodeH **child1children = NULL;
    size_t nchild1children = 0;
    rtree_node_children(children[0], &child1children, &nchild1children);
    if (nchild1children != 0) {
        fprintf(stderr, "Expected child1 to have 0 children, got %zu\n", nchild1children);
        rtree_node_children_free(child1children, nchild1children);
        rtree_node_children_free(children, nchildren);
        rtree_node_free(root);
        rtree_free(tree);
        return false;
    }
    rtree_node_children_free(child1children, nchild1children);

    rtree_node_children_free(children, nchildren);
    rtree_node_free(root);
    rtree_free(tree);
    return true;
}


bool test_rtree_1d(void) {
    const size_t N = 4;
    const uint32_t dim = 1;
    double mins[4] = {0.0, 1.0, 2.0, 3.0};
    double maxs[4] = {1.0, 2.0, 4.0, 4.0};
    int64_t ids[4] = {1, 2, 3, 4};
    RTreeH *tree = NULL;
    rtree_bulk_load(&tree, mins, maxs, ids, N, dim);
    if (tree == NULL) {
        return false;
    }
    
    // test rtree_size
    size_t size = 0;
    rtree_size(tree, &size);
    if (size != N) {
        fprintf(stderr, "Expected tree size %zu, got %zu\n", N, size);
        rtree_free(tree);
        return false;
    }

    // test rtree_depth
    size_t depth = 0;
    rtree_depth(tree, &depth);
    if (depth != 2) {
        fprintf(stderr, "Expected tree depth 2, got %zu\n", depth);
        rtree_free(tree);
        return false;
    }

    double point1[1] = {0.5};
    double point2[1] = {1.5};
    double point3[1] = {3.5};
    double point4[1] = {4.5};

    int64_t *ids_out1 = NULL;
    size_t nids_out1 = 0;
    rtree_locate_all_at_point(tree, point1, &ids_out1, &nids_out1);
    if (nids_out1 != 1 || ids_out1[0] != 1) {
        fprintf(stderr, "Expected to find id [1] at point1");
        rtree_free_ids(ids_out1, nids_out1);
        rtree_free(tree);
        return false;
    } else {
        rtree_free_ids(ids_out1, nids_out1);
    }

    int64_t *ids_out2 = NULL;
    size_t nids_out2 = 0;
    rtree_locate_all_at_point(tree, point2, &ids_out2, &nids_out2);
    if (nids_out2 != 1 || ids_out2[0] != 2) {
        fprintf(stderr, "Expected to find id [2] at point2");
        rtree_free_ids(ids_out2, nids_out2);
        rtree_free(tree);
        return false;
    } else {
        rtree_free_ids(ids_out2, nids_out2);
    }

    int64_t *ids_out3 = NULL;
    size_t nids_out3 = 0;
    rtree_locate_all_at_point(tree, point3, &ids_out3, &nids_out3);
    if (nids_out3 != 2 || ids_out3[1] != 4 || ids_out3[0] != 3) {
        fprintf(stderr, "Expected to find ids [3, 4] at point3");
        rtree_free_ids(ids_out3, nids_out3);
        rtree_free(tree);
        return false;
    } else {
        rtree_free_ids(ids_out3, nids_out3);
    }
    
    int64_t *ids_out4 = NULL;
    size_t nids_out4 = 0;
    rtree_locate_all_at_point(tree, point4, &ids_out4, &nids_out4);
    if (nids_out4 != 0) {
        fprintf(stderr, "Expected to find no ids at point4");
        rtree_free_ids(ids_out4, nids_out4);
        rtree_free(tree);
        return false;
    } else {
        rtree_free_ids(ids_out4, nids_out4);
    }

    rtree_free(tree);
    return true;
}

bool test_rtree_node_1d(void) {
    const size_t N = 5;
    const uint32_t dim = 1;
    double mins[5] = {0.0, 0.5, 1.0, -1.0, -2.0};
    double maxs[5] = {1.0, 1.5, 2.0, 0.5, -1.0};
    int64_t ids[5] = {0, 1, 2, 3, 4};
    RTreeH *tree = NULL;
    rtree_bulk_load(&tree, mins, maxs, ids, N, dim);
    if (tree == NULL) {
        return false;
    }

    // Get root node
    RTreeNodeH *root = NULL;
    rtree_root_node(tree, &root);
    if (root == NULL) {
        rtree_free(tree);
        return false;
    }

    // Get envelope of root node
    double root_min[1];
    double root_max[1];
    rtree_node_envelope(root, root_min, root_max);
    if (root_min[0] != -2.0 || root_max[0] != 2.0) {
        fprintf(stderr, "Expected root envelope to be [-2], [2], got [%f], [%f]\n",
            root_min[0], root_max[0]);
        rtree_node_free(root);
        rtree_free(tree);
        return false;
    }

    // Get children of root node
    struct RTreeNodeH **children = NULL;
    size_t nchildren = 0;
    rtree_node_children(root, &children, &nchildren);
    if (nchildren != 2) {
        fprintf(stderr, "Expected root to have 2 children, got %zu\n", nchildren);
        rtree_node_children_free(children, nchildren);
        rtree_node_free(root);
        rtree_free(tree);
        return false;
    }

    // Get envelopes of child nodes
    double child1_min[1];
    double child1_max[1];
    rtree_node_envelope(children[0], child1_min, child1_max);
    if (child1_min[0] != -2.0 || child1_max[0] != -1.0) {
        fprintf(stderr, "Expected child1 envelope to be [-2], [-1], got [%f], [%f]\n",
            child1_min[0], child1_max[0]);
        rtree_node_children_free(children, nchildren);
        rtree_node_free(root);
        rtree_free(tree);
        return false;
    }

    double child2_min[1];
    double child2_max[1];
    rtree_node_envelope(children[1], child2_min, child2_max);
    if (child2_min[0] != 0.5 || child2_max[0] != 2.0) {
        fprintf(stderr, "Expected child2 envelope to be [0.5], [2], got [%f], [%f]\n",
            child2_min[0], child2_max[0]);
        rtree_node_children_free(children, nchildren);
        rtree_node_free(root);
        rtree_free(tree);
        return false;
    }

    double *collected_mins = NULL;
    double *collected_maxs = NULL;
    size_t nboxes = 0;
    RTreeError err = rtree_collect_bounding_boxes(tree, 1, &collected_mins, &collected_maxs, &nboxes);
    if (err != Success || nboxes != nchildren) {
        fprintf(stderr, "Expected %zu collected 1D bounding boxes at level 1, got %zu\n",
            nchildren, nboxes);
        rtree_free_bounding_boxes(collected_mins, collected_maxs, nboxes, dim);
        rtree_node_children_free(children, nchildren);
        rtree_node_free(root);
        rtree_free(tree);
        return false;
    }
    bool found_child1 = false;
    bool found_child2 = false;
    for (size_t i = 0; i < nboxes; i++) {
        if (collected_mins[i] == child1_min[0] && collected_maxs[i] == child1_max[0]) {
            found_child1 = true;
        }
        if (collected_mins[i] == child2_min[0] && collected_maxs[i] == child2_max[0]) {
            found_child2 = true;
        }
    }
    if (!found_child1 || !found_child2) {
        fprintf(stderr, "Expected collected 1D bounding boxes to contain both child envelopes\n");
        rtree_free_bounding_boxes(collected_mins, collected_maxs, nboxes, dim);
        rtree_node_children_free(children, nchildren);
        rtree_node_free(root);
        rtree_free(tree);
        return false;
    }
    rtree_free_bounding_boxes(collected_mins, collected_maxs, nboxes, dim);

    // Get children of child1 node
    RTreeNodeH **child1children = NULL;
    size_t nchild1children = 0;
    rtree_node_children(children[0], &child1children, &nchild1children);
    if (nchild1children != 0) {
        fprintf(stderr, "Expected child1 to have 0 children, got %zu\n", nchild1children);
        rtree_node_children_free(child1children, nchild1children);
        rtree_node_children_free(children, nchildren);
        rtree_node_free(root);
        rtree_free(tree);
        return false;
    }
    rtree_node_children_free(child1children, nchild1children);

    rtree_node_children_free(children, nchildren);
    rtree_node_free(root);
    rtree_free(tree);
    return true;
}

bool test_rtree_empty(void) {
    const size_t N = 0;
    const uint32_t dim = 2;
    double *mins = NULL;
    double *maxs = NULL;
    int64_t *ids = NULL;
    RTreeH *tree = NULL;
    rtree_bulk_load(&tree, mins, maxs, ids, N, dim);
    if (tree == NULL) {
        fprintf(stderr, "Expected to create empty tree, got null pointer\n");
        return false;
    }

    // test rtree_size
    size_t size = 0;
    rtree_size(tree, &size);
    if (size != N) {
        fprintf(stderr, "Expected tree size %zu, got %zu\n", N, size);
        rtree_free(tree);
        return false;
    }

    size_t depth = 0;
    rtree_depth(tree, &depth);
    if (depth != 0) {
        fprintf(stderr, "Expected empty tree depth 0, got %zu\n", depth);
        rtree_free(tree);
        return false;
    }

    // Query empty tree
    double point[2] = {0.0, 0.0};
    int64_t *ids_out = NULL;
    size_t nids_out = 0;
    rtree_locate_all_at_point(tree, point, &ids_out, &nids_out);
    if (nids_out != 0) {
        fprintf(stderr, "Expected to find no ids at point in empty tree");
        rtree_free_ids(ids_out, nids_out);
        rtree_free(tree);
        return false;
    } else {
        rtree_free_ids(ids_out, nids_out);
    }

    // Check root node of empty tree
    RTreeNodeH *root = NULL;
    rtree_root_node(tree, &root);
    if (root == NULL) {
        fprintf(stderr, "Expected root node of empty tree to be non-null\n");
        rtree_free(tree);
        return false;
    }

    // Check envelope of root node of empty tree
    double root_min[2] = {1.0, 1.0};  // initialize to something
    double root_max[2] = {1.0, 1.0};
    RTreeError err = rtree_node_envelope(root, root_min, root_max);
    if (err != EmptyNodeEnvelope) {
        fprintf(stderr, "Expected EmptyNodeEnvelope error for envelope of root node of empty tree\n");
        rtree_node_free(root);
        rtree_free(tree);
        return false;
    }
    for (size_t i = 0; i < dim; i++) {
        if (root_min[i] != 1.0 || root_max[i] != 1.0) {
            fprintf(stderr, "Expected root_min and root_max to be unchanged for empty node, got root_min[%zu] = %f, root_max[%zu] = %f\n",
                i, root_min[i], i, root_max[i]);
            rtree_node_free(root);
            rtree_free(tree);
            return false;
        }
    }

    // Get children of root node of empty tree
    RTreeNodeH **children = NULL;
    size_t nchildren = 0;
    rtree_node_children(root, &children, &nchildren);
    if (nchildren != 0) {
        fprintf(stderr, "Expected root of empty tree to have 0 children, got %zu\n", nchildren);
        rtree_node_children_free(children, nchildren);
        rtree_node_free(root);
        rtree_free(tree);
        return false;
    }

    rtree_node_children_free(children, nchildren);
    rtree_node_free(root);
    rtree_free(tree);
    return true;
}

bool test_rtree_empty_1d(void) {
    const size_t N = 0;
    const uint32_t dim = 1;
    double *mins = NULL;
    double *maxs = NULL;
    int64_t *ids = NULL;
    RTreeH *tree = NULL;
    rtree_bulk_load(&tree, mins, maxs, ids, N, dim);
    if (tree == NULL) {
        fprintf(stderr, "Expected to create empty tree, got null pointer\n");
        return false;
    }
    // test rtree_size
    size_t size = 0;
    rtree_size(tree, &size);
    if (size != N) {
        fprintf(stderr, "Expected tree size %zu, got %zu\n", N, size);
        rtree_free(tree);
        return false;
    }

    size_t depth = 0;
    rtree_depth(tree, &depth);
    if (depth != 0) {
        fprintf(stderr, "Expected empty tree depth 0, got %zu\n", depth);
        rtree_free(tree);
        return false;
    }

    // Query empty tree
    double point[1] = {0.0};
    int64_t *ids_out = NULL;
    size_t nids_out = 0;
    rtree_locate_all_at_point(tree, point, &ids_out, &nids_out);
    if (nids_out != 0) {
        fprintf(stderr, "Expected to find no ids at point in empty tree");
        rtree_free_ids(ids_out, nids_out);
        rtree_free(tree);
        return false;
    } else {
        rtree_free_ids(ids_out, nids_out);
    }

    // Check root node of empty tree
    RTreeNodeH *root = NULL;
    rtree_root_node(tree, &root);
    if (root == NULL) {
        fprintf(stderr, "Expected root node of empty tree to be non-null\n");
        rtree_free(tree);
        return false;
    }

    // Check envelope of root node of empty tree
    double root_min[1] = {1.0};  // initialize to something
    double root_max[1] = {1.0};
    RTreeError err = rtree_node_envelope(root, root_min, root_max);
    if (err != EmptyNodeEnvelope) {
        fprintf(stderr, "Expected EmptyNodeEnvelope error for envelope of root node of empty tree\n");
        rtree_node_free(root);
        rtree_free(tree);
        return false;
    }
    if (root_min[0] != 1.0 || root_max[0] != 1.0) {
        fprintf(stderr, "Expected root_min and root_max to be unchanged for empty node, got root_min[0] = %f, root_max[0] = %f\n",
            root_min[0], root_max[0]);
        rtree_node_free(root);
        rtree_free(tree);
        return false;
    }

    // Get children of root node of empty tree
    RTreeNodeH **children = NULL;
    size_t nchildren = 0;
    rtree_node_children(root, &children, &nchildren);
    if (nchildren != 0) {
        fprintf(stderr, "Expected root of empty tree to have 0 children, got %zu\n", nchildren);
        rtree_node_children_free(children, nchildren);
        rtree_node_free(root);
        rtree_free(tree);
        return false;
    }

    rtree_node_children_free(children, nchildren);
    rtree_node_free(root);
    rtree_free(tree);
    return true;
}

bool test_rtree_singleton_depth(void) {
    const size_t N = 1;
    const uint32_t dim = 2;
    double mins[2] = {0.0, 0.0};
    double maxs[2] = {1.0, 1.0};
    int64_t ids[1] = {1};
    RTreeH *tree = NULL;
    rtree_bulk_load(&tree, mins, maxs, ids, N, dim);
    if (tree == NULL) {
        fprintf(stderr, "Expected to create tree, got null pointer\n");
        return false;
    }

    // test rtree_depth
    size_t depth = 0;
    rtree_depth(tree, &depth);
    if (depth != 1) {
        fprintf(stderr, "Expected tree depth 1 for singleton tree, got %zu\n", depth);
        rtree_free(tree);
        return false;
    }
    rtree_free(tree);

    const uint32_t dim1d = 1;
    double mins1d[1] = {0.0};
    double maxs1d[1] = {1.0};
    RTreeH *tree1d = NULL;
    rtree_bulk_load(&tree1d, mins1d, maxs1d, ids, N, dim1d);
    if (tree1d == NULL) {
        fprintf(stderr, "Expected to create 1d tree, got null pointer\n");
        return false;
    }

    size_t depth1d = 0;
    rtree_depth(tree1d, &depth1d);
    if (depth1d != 1) {
        fprintf(stderr, "Expected tree depth 1 for singleton 1d tree, got %zu\n", depth1d);
        rtree_free(tree1d);
        return false;
    }
    rtree_free(tree1d);

    return true;
}

bool test_rtree_depth(void) {
    // Default MAX_SIZE is 6, so bulk loading 6 should create a tree of depth 1,
    // while bulk loading 7 should create a tree of depth 2.
    const size_t N = 6;
    const uint32_t dim = 2;
    double mins[12];
    double maxs[12];
    int64_t ids[6];
    for (size_t i = 0; i < N; i++) {
        mins[2*i] = (double)i;
        mins[2*i + 1] = (double)i;
        maxs[2*i] = (double)i + 0.5;
        maxs[2*i + 1] = (double)i + 0.5;
        ids[i] = i + 1;
    }
    RTreeH *tree = NULL;
    rtree_bulk_load(&tree, mins, maxs, ids, N, dim);
    if (tree == NULL) {
        fprintf(stderr, "Expected to create tree, got null pointer\n");
        return false;
    }
    size_t depth = 0;
    rtree_depth(tree, &depth);
    if (depth != 1) {
        fprintf(stderr, "Expected tree depth 1 for 6 items, got %zu\n", depth);
        rtree_free(tree);
        return false;
    }
    rtree_free(tree);

    const size_t N1 = 7;
    double mins1[14];
    double maxs1[14];
    int64_t ids1[7];
    for (size_t i = 0; i < N1; i++) {
        mins1[2*i] = (double)i;
        mins1[2*i + 1] = (double)i;
        maxs1[2*i] = (double)i + 0.5;
        maxs1[2*i + 1] = (double)i + 0.5;
        ids1[i] = i + 1;
    }
    RTreeH *tree1 = NULL;
    rtree_bulk_load(&tree1, mins1, maxs1, ids1, N1, dim);
    if (tree1 == NULL) {
        fprintf(stderr, "Expected to create tree, got null pointer\n");
        return false;
    }
    size_t depth1 = 0;
    rtree_depth(tree1, &depth1);
    if (depth1 != 2) {
        fprintf(stderr, "Expected tree depth 2 for 7 items, got %zu\n", depth1);
        rtree_free(tree1);
        return false;
    }

    rtree_free(tree1);
    return true;
}

bool test_locate_all_at_points(void) {
    const size_t N = 2;
    const uint32_t dim = 2;
    double mins[4] = {0.0, 0.0, 1.0, 1.0};
    double maxs[4] = {2.0, 2.0, 3.0, 3.0};
    int64_t ids[2] = {1, 2};
    RTreeH *tree = NULL;
    rtree_bulk_load(&tree, mins, maxs, ids, N, dim);
    if (tree == NULL) {
        return false;
    }

    const size_t n_points = 3;
    double points[6] = {
        1.5, 1.5,   // 2, 1
        0.0, 0.0,   // 1
        -1.0, 0.0,  // none
    };

    int64_t *ids_out = NULL;
    size_t *offsets_out = NULL;
    RTreeError err = rtree_locate_all_at_points(tree, points, n_points, &ids_out, &offsets_out);
    if (err != Success) {
        fprintf(stderr, "rtree_locate_all_at_points returned error %d\n", err);
        rtree_free(tree);
        return false;
    }

    // offsets should be [0, 2, 3, 3].
    size_t expected_offsets[4] = {0, 2, 3, 3};
    for (size_t i = 0; i < n_points + 1; i++) {
        if (offsets_out[i] != expected_offsets[i]) {
            fprintf(stderr, "Expected offsets[%zu] = %zu, got %zu\n",
                i, expected_offsets[i], offsets_out[i]);
            rtree_free_ids(ids_out, offsets_out[n_points]);
            rtree_free_offsets(offsets_out, n_points + 1);
            rtree_free(tree);
            return false;
        }
    }

    int64_t expected_ids[3] = {2, 1, 1};
    for (size_t i = 0; i < offsets_out[n_points]; i++) {
        if (ids_out[i] != expected_ids[i]) {
            fprintf(stderr, "Expected ids[%zu] = %lld, got %lld\n",
                i, expected_ids[i], ids_out[i]);
            rtree_free_ids(ids_out, offsets_out[n_points]);
            rtree_free_offsets(offsets_out, n_points + 1);
            rtree_free(tree);
            return false;
        }
    }

    rtree_free_ids(ids_out, offsets_out[n_points]);
    rtree_free_offsets(offsets_out, n_points + 1);

    // A zero-point query should return valid offsets = [0] and no ids.
    int64_t *ids_out0 = NULL;
    size_t *offsets_out0 = NULL;
    err = rtree_locate_all_at_points(tree, NULL, 0, &ids_out0, &offsets_out0);
    if (err != Success) {
        fprintf(stderr, "rtree_locate_all_at_points with 0 points returned error %d\n", err);
        rtree_free(tree);
        return false;
    }
    if (offsets_out0[0] != 0) {
        fprintf(stderr, "Expected offsets[0] = 0 for zero-point query, got %zu\n", offsets_out0[0]);
        rtree_free_ids(ids_out0, offsets_out0[0]);
        rtree_free_offsets(offsets_out0, 1);
        rtree_free(tree);
        return false;
    }
    rtree_free_ids(ids_out0, offsets_out0[0]);
    rtree_free_offsets(offsets_out0, 1);

    rtree_free(tree);
    return true;
}


void run_test(
    bool (test)(void),
    const char *test_name,
    bool *passed
) {
    if (!test()) {
        *passed = false;
        fprintf(stderr, "Test failed: %s\n", test_name);
    } else {
        fprintf(stdout, "Test passed: %s\n", test_name);
    }
}

int main(void) {
    bool passed = true;

    run_test(test_create_and_free, "test_create_and_free", &passed);
    run_test(test_null, "test_null", &passed);
    run_test(test_get_dimension, "test_get_dimension", &passed);
    run_test(test_bulk_load, "test_bulk_load", &passed);
    run_test(test_locate_all_at_points, "test_locate_all_at_points", &passed);
    run_test(test_nodes, "test_nodes", &passed);
    run_test(test_rtree_1d, "test_rtree_1d", &passed);
    run_test(test_rtree_node_1d, "test_rtree_node_1d", &passed);
    run_test(test_rtree_empty, "test_rtree_empty", &passed);
    run_test(test_rtree_empty_1d, "test_rtree_empty_1d", &passed);
    run_test(test_invalid_dimension, "test_invalid_dimension", &passed);
    run_test(test_rtree_singleton_depth, "test_rtree_singleton_depth", &passed);
    run_test(test_rtree_depth, "test_rtree_depth", &passed);

    if (passed) {
        fprintf(stdout, "All tests passed\n");
        return 0;
    } else {
        fprintf(stderr, "Some tests failed\n");
        return 1;
    }
}
