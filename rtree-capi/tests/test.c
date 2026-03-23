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
    size_t ids[2] = {1, 2};
    RTreeH *tree = NULL;
    rtree_bulk_load(&tree, mins, maxs, ids, N, dim);
    if (tree == NULL) {
        return false;
    }

    double point1[2] = {1.5, 1.5};
    double point2[2] = {0.0, 0.0};
    double point3[2] = {-1.0, 0.0};

    size_t *ids_out1 = NULL;
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

    size_t *ids_out2 = NULL;
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

    size_t *ids_out3 = NULL;
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
    size_t ids[2] = {1, 2};
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
    size_t ids[4] = {1, 2, 3, 4};
    RTreeH *tree = NULL;
    rtree_bulk_load(&tree, mins, maxs, ids, N, dim);
    if (tree == NULL) {
        return false;
    }

    double point1[1] = {0.5};
    double point2[1] = {1.5};
    double point3[1] = {3.5};
    double point4[1] = {4.5};

    size_t *ids_out1 = NULL;
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

    size_t *ids_out2 = NULL;
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

    size_t *ids_out3 = NULL;
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
    
    size_t *ids_out4 = NULL;
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
    size_t ids[5] = {0, 1, 2, 3, 4};
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
    size_t *ids = NULL;
    RTreeH *tree = NULL;
    rtree_bulk_load(&tree, mins, maxs, ids, N, dim);
    if (tree == NULL) {
        fprintf(stderr, "Expected to create empty tree, got null pointer\n");
        return false;
    }

    // Query empty tree
    double point[2] = {0.0, 0.0};
    size_t *ids_out = NULL;
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
        rtree_free(tree);
        return false;
    }
    double root_min[2];
    double root_max[2];
    rtree_node_envelope(root, root_min, root_max);
    if (root_min[0] != 0.0 || root_min[1] != 0.0 || root_max[0] != 0.0 || root_max[1] != 0.0) {
        fprintf(stderr, "Expected root envelope of empty tree to be [0, 0], [0, 0], got [%f, %f], [%f, %f]\n",
            root_min[0], root_min[1], root_max[0], root_max[1]);
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
    run_test(test_nodes, "test_nodes", &passed);
    run_test(test_rtree_1d, "test_rtree_1d", &passed);
    run_test(test_rtree_node_1d, "test_rtree_node_1d", &passed);
    run_test(test_rtree_empty, "test_rtree_empty", &passed);

    if (passed) {
        fprintf(stdout, "All tests passed\n");
        return 0;
    } else {
        fprintf(stderr, "Some tests failed\n");
        return 1;
    }
}
