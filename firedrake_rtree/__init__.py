import importlib.resources
import os
import sysconfig


def get_include() -> str:
    """Return the directory containing rtree-capi.h."""
    with importlib.resources.as_file(
        importlib.resources.files("firedrake_rtree") / "include"
    ) as include_dir:
        return str(include_dir)


def get_lib() -> str:
    """Return the directory containing the rtree-capi shared object."""
    return os.path.dirname(os.path.abspath(__file__))


def get_lib_filename() -> str:
    """Return the path to the rtree-capi shared object."""
    sufix = sysconfig.get_config_var("EXT_SUFFIX")
    return os.path.join(get_lib(), f"firedrake_rtree{sufix}")
