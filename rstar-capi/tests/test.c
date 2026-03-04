#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

#include "rstar-capi.h"


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
        fprintf(stderr, "Expected to find ids [1, 2] at point1");
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
    run_test(test_get_dimension, "test_get_dimension", &passed);
    run_test(test_bulk_load, "test_bulk_load", &passed);

    if (passed) {
        fprintf(stdout, "All tests passed\n");
        return 0;
    } else {
        fprintf(stderr, "Some tests failed\n");
        return 1;
    }
}
