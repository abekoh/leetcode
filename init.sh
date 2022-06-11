#!/bin/bash -e

if [ -z "$1" ]; then
  echo "please set problem name"
  exit 1
fi

NAME=$1

cp -R ./__TEMPLATES/cpp/REPLACE_ME ./
mv REPLACE_ME $NAME
find $NAME -type f | sed -e "s/REPLACE_ME/$NAME/g"
