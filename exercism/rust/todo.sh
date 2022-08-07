#!/usr/bin/env bash

for proj in * ; do
    if [ -d "$proj" ]; then
        pushd "$proj" > /dev/null
        cargo test > /dev/null 2>&1
        if [ $? -eq 0 ]; then
            echo "✅ $proj"
        else
            echo "❌ $proj"
        fi
        popd > /dev/null
    fi
done