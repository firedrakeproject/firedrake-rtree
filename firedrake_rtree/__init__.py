import importlib.resources

def get_include() -> str:
    """Return the directory containing rtree-capi.h."""
    with importlib.resources.as_file(
        importlib.resources.files("firedrake_rtree") / "include"
    ) as include_dir:
        return str(include_dir)


def get_lib() -> str:
    """Return the directory containing the rtree-capi shared library."""
    with importlib.resources.as_file(
        importlib.resources.files("firedrake_rtree")
    ) as lib_dir:
        return str(lib_dir)


def get_lib_filename() -> str:
    """Return the path to the rtree-capi shared library."""
    from firedrake_rtree import rtree_capi
    return rtree_capi.__file__
