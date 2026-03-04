#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


enum RTreeError {
  Success = 0,
  NullPointer = 1,
  InvalidDimension = 2,
};
typedef uint16_t RTreeError;

typedef struct RTreeH RTreeH;

typedef struct RTreeNodeH RTreeNodeH;

RTreeError rtree_bulk_load(struct RTreeH **tree,
                           const double *mins,
                           const double *maxs,
                           const size_t *ids,
                           size_t n,
                           uint32_t dim);

RTreeError rtree_create(struct RTreeH **tree, uint32_t dim);

RTreeError rtree_free(struct RTreeH *tree);

RTreeError rtree_free_ids(size_t *ids, size_t n);

RTreeError rtree_get_dimension(const struct RTreeH *tree, uint32_t *dim);

RTreeError rtree_locate_all_at_point(const struct RTreeH *tree,
                                     const double *point,
                                     size_t **ids_out,
                                     size_t *nids_out);

RTreeError rtree_node_children_free(struct RTreeNodeH **children, size_t n);

RTreeError rtree_node_free(struct RTreeNodeH *node);

RTreeError rtree_root_node(const struct RTreeH *tree, struct RTreeNodeH **node);
