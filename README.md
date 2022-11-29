# vpp-stat-client

This is a work-in-progress crate to access the VPP (https://fd.io/) statistics
segment, using the bingins to the C library that the VPP provides.

## Building

To successfully compile, it requires the VPP_LIB_DIR environment variable to 
be set to the directory where the .so files are, and VPP_INC_DIR to be set
where the include files are. For running, the LD_LIBRARY_PATH might need
to also be set, such that the .so files were found by the linker.

If you are building locally using "make build", with ~/vpp being your VPP directory
as your checkout directory, then the values will be as follows:

```
export VPP_LIB_DIR=${HOME}/vpp/build-root/install-vpp_debug-native/vpp/lib/x86_64-linux-gnu/
export VPP_INC_DIR=${HOME}/vpp/build-root/install-vpp_debug-native/vpp/include/
export LD_LIBRARY_PATH=${VPP_LIB_DIR}
```

## Running examples

This example will show the naive usage of stats in order to print the data:

```
cargo run --example dump-all
```

