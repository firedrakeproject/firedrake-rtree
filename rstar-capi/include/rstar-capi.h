#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


typedef struct RTreeH RTreeH;

void rtree_bulk_load(struct RTreeH **tree,
                     const double *mins,
                     const double *maxs,
                     const size_t *data,
                     size_t n,
                     uint32_t dim);

void rtree_create(struct RTreeH **tree, uint32_t dim);

void rtree_free(struct RTreeH *tree);

void rtree_get_dimension(const struct RTreeH *tree, uint32_t *dim);

void rtree_locate_all_at_point(const struct RTreeH *tree,
                               const double *point,
                               size_t **ids_out,
                               size_t *nids_out);
