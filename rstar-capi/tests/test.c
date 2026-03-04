#include <assert.h>
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

    if (passed) {
        fprintf(stdout, "All tests passed\n");
        return 0;
    } else {
        fprintf(stderr, "Some tests failed\n");
        return 1;
    }
}
