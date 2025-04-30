#!/bin/bash
#
# Called from the Makefile to build all (or some) components
#
# ./script/build_components.sh [WASI_BUILD_DIR]
#
# WASI_BUILD_DIR: the directory to build the component in
# e.g. ./script/build_components.sh components/golang-evm-price-oracle
#

# Extract arguments
WASI_BUILD_DIR="$1"

RECIPE="wasi-build"
MAKEFILE_DIRS=`find components/* -maxdepth 1 -name "Makefile" -o -name "makefile"`

for makefile_path in $MAKEFILE_DIRS; do
    if grep -q "^${RECIPE}:" "$makefile_path" 2>/dev/null; then
        if [ "$WASI_BUILD_DIR" != "" ] && [[ "$makefile_path" != *"$WASI_BUILD_DIR"* ]]; then
            continue
        fi;
        parent_dir=$(dirname "$makefile_path")
        make -s -C "$parent_dir" $RECIPE
    else
        echo "Recipe '$RECIPE' not found in $dir"
    fi;
done
