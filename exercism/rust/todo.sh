#!/usr/bin/env bash

# Old Way
# for proj in * ; do
#     if [ -d "$proj" ]; then
#         pushd "$proj" > /dev/null
#         cargo test > /dev/null 2>&1
#         if [ $? -eq 0 ]; then
#             echo "✅ $proj"
#         else
#             echo "❌ $proj"
#         fi
#         popd > /dev/null
#     fi
# done

# This is The Way
pushd ./tester > /dev/null
cargo run -q
popd > /dev/null