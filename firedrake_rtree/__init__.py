from pathlib import Path

_firedrake_rtree_dir = Path(__file__).parent


def get_library() -> str:
    """Return the path to the rtree-capi shared object."""
    return str(next(_firedrake_rtree_dir.joinpath("firedrake_rtree").glob("*firedrake_rtree*")))


def get_include() -> str:
    """Return the directory containing rtree-capi.h."""
    return str(_firedrake_rtree_dir.joinpath("include"))

