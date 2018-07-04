#!/bin/bash

for i in $(docker ps -f "status=exited" -q)
  do
    if [ -n $i ]; then
      echo "$i"
      docker rm "$i"
    else
        echo "No containers with exit status"
    fi
 done
