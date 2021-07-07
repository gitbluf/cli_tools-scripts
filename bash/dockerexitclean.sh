#!/bin/bash

for i in $(docker ps -f "status=exited" -q)
  do
    if [ -n $i ]; then
      echo "$i" >> /path/to/lastdockercleanup.txt
      echo "$i" $(date) >> /path/to/dockerCleanHistory.txt
      docker rm "$i"
      echo "Subject: Docker cleanup" && cat /path/to/lastdockercleanup.txt | sendmail  "example@email.com"
      rm -rf /path/to/lastdockercleanup.txt
    else
        echo "No containers with exit status" $(date) >> /path/to/dockerCleanHistory.txt
    fi
 done
