#!bin/bash

# Check if an argument is provided
if [ "$#" -ne 1 ]; then
	echo "Usage: $0 <dayX>"
	exit 1
fi

# Extract the day argument
DAY=$1

# Path to the binary - adjust if your structure is different
BINARY="./build/$DAY/$DAY"

# Check if the binary exists
if [ -f "$BINARY" ]; then
	$BINARY
else
	echo "Binary for $DAY does not exist. Did you compile?"
fi
