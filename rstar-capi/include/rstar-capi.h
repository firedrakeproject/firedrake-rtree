#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


typedef struct RTreeH RTreeH;

void rtree_create(struct RTreeH **tree, uint32_t dim);

void rtree_free(struct RTreeH *tree);

void rtree_get_dimension(const struct RTreeH *tree, uint32_t *dim);
