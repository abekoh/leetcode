#!/bin/bash -e
if [ $# != 2 ]; then
  echo "please set language and problem_name"
  exit 1
fi

LANG=$1
NAME=$2

case $LANG in
  cpp)
    cp -R ./__TEMPLATES/cpp/REPLACE_ME ./
    mv REPLACE_ME $NAME
    find $NAME -type f | xargs sed -i '' -e "s/REPLACE_ME/$NAME/g"
    echo "add_subdirectory($NAME)" >> ./CMakeLists.txt
    ;;

  rust)
    cargo init $NAME
    ;;

  *)
    echo "unsupported language: $LANG"
    exit 1
    ;;
esac

