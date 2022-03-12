#!/bin/bash

DT=$(date +%Y-%m-%d-%H-%M)
DIR=.bak
FILE="$DIR/$DT.tgz"
FILE_EXCLUDE=exclude.tag
mkdir $DIR -p
touch .bak/$FILE_EXCLUDE
touch target/$FILE_EXCLUDE
touch app/node_modules/$FILE_EXCLUDE
touch samples/actixweb/websocket/target/$FILE_EXCLUDE
touch samples/lazy-log/node_modules/$FILE_EXCLUDE

tar -zcvf $FILE \
	--exclude-tag-all=$FILE_EXCLUDE \
	.