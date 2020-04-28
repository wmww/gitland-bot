#!/bin/bash
set -euo pipefail

SERVER_REPO="$PWD"/../gitland/
CLIENT_REPO="$PWD"/../gitland-client/

iteration() {
  echo "Pulling server data..."
  git -C "$SERVER_REPO" pull
  echo
  echo Running bot...
  ./target/release/wmww-gitland-bot --server-repo "$SERVER_REPO" act --client-repo "$CLIENT_REPO"
  echo
  echo "Checking for an action..."
  MOVE="$(cat $CLIENT_REPO/act)"
  if ! git -C "$CLIENT_REPO" diff --exit-code act; then
    git -C "$CLIENT_REPO" add act
    git -C "$CLIENT_REPO" commit -m "Move $MOVE"
    git -C "$CLIENT_REPO" push -f
  else
    echo "No change, still going $MOVE"
  fi
}

cargo build --release

while true; do
  echo
  echo ================================================
  echo
  iteration
  echo
  echo "sleeping..."
  sleep 45s
done
