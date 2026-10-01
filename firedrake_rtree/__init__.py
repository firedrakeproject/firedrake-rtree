from pathlib import Path

_firedrake_rtree_dir = Path(__file__).parent


def get_library() -> Path:
    """Return the path to the rtree-capi shared object."""
    return next(
        _firedrake_rtree_dir.joinpath("firedrake_rtree").glob("*firedrake_rtree*")
    )


def get_include() -> Path:
    """Return the directory containing rtree-capi.h."""
    return _firedrake_rtree_dir.joinpath("include")
