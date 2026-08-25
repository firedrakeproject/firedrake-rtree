#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


enum RTreeError {
  Success = 0,
  NullPointer = 1,
  InvalidDimension = 2,
  EmptyNodeEnvelope = 3,
};
typedef uint16_t RTreeError;

typedef struct RTreeH RTreeH;

typedef struct RTreeNodeH RTreeNodeH;

/**
 * Returns a new tree containing the given objects. The input arrays must have the same length.
 * Returns an empty tree if the input arrays are empty.
 * Supported dimensions are currently 1, 2, and 3. Returns an InvalidDimension error for unsupported dimensions.
 * You must free the returned tree with `rtree_free`.
 */
RTreeError rtree_bulk_load(struct RTreeH **tree,
                           const double *mins,
                           const double *maxs,
                           const int64_t *ids,
                           size_t n,
                           uint32_t dim);

/**
 * Returns bounding boxes at the specified level of the tree.
 */
RTreeError rtree_collect_bounding_boxes(const struct RTreeH *tree,
                                        size_t level,
                                        double **mins_out,
                                        double **maxs_out,
                                        size_t *n_boxes_out);

/**
 * Returns a new empty tree with the given dimension.
 */
RTreeError rtree_create(struct RTreeH **tree, uint32_t dim);

/**
 * Returns the depth of the tree, defined as the number of edges in the longest path from the root to a leaf.
 * An empty tree has depth 0.
 */
RTreeError rtree_depth(const struct RTreeH *tree,
                       size_t *depth_out);

/**
 * Frees the given tree.
 */
RTreeError rtree_free(struct RTreeH *tree);

/**
 * Frees the bounding boxes returned by `rtree_collect_bounding_boxes`.
 */
RTreeError rtree_free_bounding_boxes(double *mins, double *maxs, size_t n_boxes, uint32_t dim);

/**
 * Frees the ids returned by `rtree_locate_all_at_point`.
 */
RTreeError rtree_free_ids(int64_t *ids, size_t n);

/**
 * Frees the offsets returned by `rtree_locate_all_at_points`.
 */
RTreeError rtree_free_offsets(size_t *offsets, size_t n);

/**
 * Frees the point indices returned by `rtree_locate_points_grouped_by_id_unique`.
 */
RTreeError rtree_free_point_indices(size_t *point_indices, size_t n);

/**
 * Returns the dimension of the tree.
 */
RTreeError rtree_get_dimension(const struct RTreeH *tree, uint32_t *dim);

/**
 * Returns the ids of all objects in the tree that contain the given point.
 * If no objects contain the point, returns nids_out = 0.
 * You must free the returned ids with `rtree_free_ids`.
 */
RTreeError rtree_locate_all_at_point(const struct RTreeH *tree,
                                     const double *point,
                                     int64_t **ids_out,
                                     size_t *nids_out);

/**
 * Returns the ids of all objects in the tree that contain each of the given points.
 * `points` is an array of `n_points * dim` doubles
 * The candidate ids are returned in `ids_out`, and `offsets_out`
 * is an array of length `n_points + 1` such that the ids for point `i` are
 * `ids_out[offsets_out[i]..offsets_out[i + 1]]`.
 * You must free `ids_out` (with length `offsets_out[n_points]`) with `rtree_free_ids`,
 * and `offsets_out` (with length `n_points + 1`) with `rtree_free_offsets`.
 */
RTreeError rtree_locate_all_at_points(const struct RTreeH *tree,
                                      const double *points,
                                      size_t n_points,
                                      int64_t **ids_out,
                                      size_t **offsets_out);

/**
 * Returns the unique IDs of objects containing each point.
 * IDs are sorted in ascending order within each point's offset.
 * Only useful if your RTree contains multiple bounding boxes sharing the same ID.
 * You must free `ids_out` with `rtree_free_ids` and `offsets_out` with `rtree_free_offsets`.
 */
RTreeError rtree_locate_all_at_points_unique(const struct RTreeH *tree,
                                             const double *points,
                                             size_t n_points,
                                             int64_t **ids_out,
                                             size_t **offsets_out);

/**
 * Returns unique object IDs grouped across all points.
 * IDs are sorted in ascending order. `offsets_out` has length `n_ids_out + 1`,
 * and the point indices for `ids_out[i]` are `point_indices_out[offsets_out[i]..offsets_out[i + 1]]`.
 * You must free `ids_out` with `rtree_free_ids`, `offsets_out` with
 * `rtree_free_offsets`, and `point_indices_out` with `rtree_free_point_indices`.
 */
RTreeError rtree_locate_points_grouped_by_id_unique(const struct RTreeH *tree,
                                                    const double *points,
                                                    size_t n_points,
                                                    int64_t **ids_out,
                                                    size_t **offsets_out,
                                                    size_t **point_indices_out,
                                                    size_t *n_ids_out);

/**
 * Returns the child nodes of a given node. You must free the returned child nodes with `rtree_node_children_free`.
 * If the node is a leaf, or a root node of an empty tree, returns nchildren = 0.
 */
RTreeError rtree_node_children(const struct RTreeNodeH *node,
                               struct RTreeNodeH ***children,
                               size_t *nchildren);

/**
 * Frees the child nodes returned by `rtree_node_children`.
 */
RTreeError rtree_node_children_free(struct RTreeNodeH **children, size_t n);

/**
 * Returns the minimum bounding box that covers all the boxes in the node.
 * Returns an EmptyNodeEnvelope error if given a root node of an empty tree, which has no envelope.
 */
RTreeError rtree_node_envelope(const struct RTreeNodeH *node, double *min_out, double *max_out);

/**
 * Frees the node returned by `rtree_root_node`.
 */
RTreeError rtree_node_free(struct RTreeNodeH *node);

/**
 * Returns the root node of a tree. You must free the returned node with `rtree_node_free`.
 * If the tree is empty, returns an EmptyNode.
 */
RTreeError rtree_root_node(const struct RTreeH *tree, struct RTreeNodeH **node);

/**
 * Returns the size of the tree, defined as the number of objects in the tree.
 * An empty tree has size 0.
 */
RTreeError rtree_size(const struct RTreeH *tree, size_t *size_out);
