#!/bin/bash

anvil --fork-url https://ethereum-holesky-rpc.publicnode.com &
anvil_pid=$!
trap "kill -9 $anvil_pid && echo -e '\nKilled anvil'" EXIT

# TODO: eigen stuff here

wait
