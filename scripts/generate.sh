#!/usr/bin/env bash

# This script will:
# 1. Generate a new project from this template in /tmp
# 2. Verifies the new project compiles
# 3. Creates a new GitHub repository and pushes the generated project to it

set -e

function main() {
  if [[ $# -ne 3 ]]; then
    echo "usage: new-axum <github-user> <project-name> <description>"
    return 1
  fi
  # Used in Cargo.toml for homepage and repository properties
  GITHUB_USER="$1"
  NAME="$2"
  DESC="$3"
  pushd /tmp
  cargo generate digitalsanctum/axum-template --name "$NAME" -d github_user="${GITHUB_USER}" -d description="$DESC"
  pushd "$NAME"
  cargo build
  git add .
  git commit -am "Initial commit"
  gh repo create "$NAME" --private --source=. --remote=upstream --description "$DESC" --push
  popd
  popd
}

main "$@"
