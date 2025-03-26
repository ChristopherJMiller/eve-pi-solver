#!/bin/bash

# Check if SWI-Prolog is installed
if ! command -v swipl &> /dev/null; then
    echo "Error: SWI-Prolog is not installed. Please install it first."
    echo "Visit https://www.swi-prolog.org/download/stable for installation instructions."
    exit 1
fi

# Default product to solve for
PRODUCT="recursive_computing_module"

# If a product was specified as an argument, use that instead
if [ $# -eq 1 ]; then
    PRODUCT=$1
fi

echo "Solving for $PRODUCT..."

# Run the solver with the specified product
# Pass the product name as a command-line argument to the Prolog script
swipl -q -l run_solver.pl -t main -- "$PRODUCT"

# Check if output.json was created
if [ -f "output.json" ]; then
    echo "Production plan written to output.json"
    
    # Optional: Display a brief summary if jq is installed
    if command -v jq &> /dev/null; then
        echo "Plan summary:"
        jq '.plan | length' output.json | xargs echo "Total steps:"
        jq '.plan[] | .character + " on " + .planet + " (" + .type + ") producing " + .output' output.json
    fi
else
    echo "Error: Failed to generate production plan"
    exit 1
fi 