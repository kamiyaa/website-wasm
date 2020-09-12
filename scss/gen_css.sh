#!/bin/sh

date
echo '=========='

CSS_DIR=../static/css
SASS_DIR=.

mkdir -p "$CSS_DIR"

echo "cat ${SASS_DIR}/*.scss | sassc > ${CSS_DIR}/index.css"
cat "${SASS_DIR}"/*.scss | sassc > "${CSS_DIR}/index.css"
