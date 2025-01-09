#!/usr/bin/env bash

# Set shell settings (see https://sipb.mit.edu/doc/safe-shell/).
set -eu -o pipefail

# Get arguments.
REVISION="${1:-master}"

# Determine variables.
SCRIPT_DIR="$(dirname "$(readlink -f "$0")")"
TMP_DIR="${SCRIPT_DIR}/__tmp"
WEB_DIR="${SCRIPT_DIR}/src/web"
REPO="https://github.com/openscopeproject/InteractiveHtmlBom.git"

# Update files.
rm -rf "$WEB_DIR" "$TMP_DIR"
git clone -b "$REVISION" "$REPO" "$TMP_DIR"
cp -rf "${TMP_DIR}/InteractiveHtmlBom/web" "$WEB_DIR"

# Write version file.
pushd "$TMP_DIR/InteractiveHtmlBom"
VERSION=$(python3 -c "import version; print(version.version);")
echo "$VERSION" > "${WEB_DIR}/version.txt"
popd

# Clean up.
rm -rf "$TMP_DIR"
rm -rf "${WEB_DIR}/user-file-examples"
echo "Updated to version $VERSION."
