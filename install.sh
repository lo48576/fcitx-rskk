#!/bin/sh

set -eu

# For debugging and testing purpose.

cd "$(readlink -f "$(dirname "$0")")"

FCITX_DIR="${HOME}/.config/fcitx"
ADDON_DIR="${FCITX_DIR}/addon"
IM_DIR="${FCITX_DIR}/inputmethod"
FCITX_LIB_DIR="${FCITX_DIR}/lib"

set -x

cargo build
mkdir -p "${ADDON_DIR}" "${IM_DIR}" "${FCITX_LIB_DIR}"
cp -f assets/fcitx-rskk.conf "${ADDON_DIR}/"
cp -f assets/rskk.conf "${IM_DIR}/"
cp target/debug/libfcitx_rskk.so "${FCITX_LIB_DIR}/fcitx-rskk.so"
