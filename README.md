# firedrake-rtree

A C api for point location querying using a spatial index.

For 1D, an interval tree is used.
For 2D and higher, an R-tree is used.
The [rstar](https://github.com/georust/rstar) library is used for the R-tree.
