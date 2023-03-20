#!/bin/bash

while true
do
    #eval "docker stats --no-stream" > status.txt
    eval "$(cat statuspipe)" > status.txt
    sleep 10
    #echo "$(cat statuspipe)"
    #cat statuspipe | eval
done
