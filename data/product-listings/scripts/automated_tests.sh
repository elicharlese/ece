#!/bin/bash

# automated_tests.sh - Your reliable assistant for automated testing

set -e

# Check if pytest is installed
if ! command -v pytest &> /dev/null; then
    echo "pytest not found, installing..."
    pip3 install pytest
else
    echo "pytest is already installed."
fi

echo "Running your tests... 🧪"

# Run pytest and store the result
pytest_result=$(pytest)

# Print the result of the pytest run
echo "${pytest_result}"

# Extract the summary of the tests
summary=$(echo "${pytest_result}" | tail -n 1)

# Check if tests failed
if [[ "${summary}" == *"failed"* ]]; then
    echo "Some tests have failed 😞, please check the output above."
else
    echo "All tests passed successfully. Great job! 🎉"
fi