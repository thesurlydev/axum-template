#!/usr/bin/env bash

# This script will:
# 1. Generate a new project from this template in /tmp
# 2. Verifies the new project compiles

set -e

function main() {
  if [[ $# -ne 3 ]]; then
    echo "usage: $0 <github-user> <project-name> <description>"
    return 1
  fi
  GITHUB_USER="$1"
  NAME="$2"
  DESC="$3"
  cargo generate --path ../ --name "$NAME" -d github_user="${GITHUB_USER}" -d description="$DESC" --destination ../../
  cd ../../"$NAME"
  cargo build
}

main "$@"
