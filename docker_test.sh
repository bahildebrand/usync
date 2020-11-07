#!/bin/bash
declare -r CUR_DIR='pwd'

docker run antmicro/renode \
    -v ${CUR_DIR}:/ \
    /bin/bash -c "./entry.sh"