#!/bin/bash

for CRATE in *
do
  if [ -d "${CRATE}" ]; then
    cd $CRATE;
    echo ${CRATE};
    cargo upgrade
    cd ..;
  fi
done
