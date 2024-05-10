#!/bin/bash

# Define the start and end range
start=0
end=10

# Define the number of tests
tests=50

# Check if the "img" and "data" folder exist, if not, create them
if [ ! -d "img" ]; then
    mkdir img
fi

if [ ! -d "data" ]; then
    mkdir data
fi


# Loop through the range of N values
for ((N=start; N<=end; N++)); do
    # Check if the file exists
    if [ ! -f "data/random_test.$(printf "%03d" $N).out" ]; then
        # If the file does not exist, create it and append the header
        echo "HW;effectiveHW;keySHW;outSHW;finSHW;timing;solutions;contained;x;k" > "data/random_test.$(printf "%03d" $N).out"
    fi
    # If the file exists, run the Rust code and append the output
    ../target/release/leak_solver $tests $N >> "data/random_test.$(printf "%03d" $N).out"    
done

