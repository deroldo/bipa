#!/bin/bash

TARGET_DIR="target"
TARGET_SIZE=$(du -sh "$TARGET_DIR" 2>/dev/null | cut -f1)

if [ -z "$TARGET_SIZE" ]; then
  echo "No 'target' directory found."
  exit 0
fi

echo "The 'target' directory is using $TARGET_SIZE. Do you want to clean it? (y/n)"
read -r confirm

if [[ "$confirm" == "y" || "$confirm" == "Y" ]]; then
  cargo clean
  echo "'target' directory has been cleaned."
else
  echo "Operation canceled."
fi
