#!/usr/bin/env bash
set -euo pipefail

BLOG_DIR="$1"

if [ -z "$BLOG_DIR" ]; then
    echo "Must provide directory of blog"
    exit 1
fi

cargo watch -x "run -- -C $BLOG_DIR"
