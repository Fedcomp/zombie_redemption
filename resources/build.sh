#!/usr/bin/bash

find -name '*.svg' | while read -r line ; do
    NEW_NAME=$(echo $line | rev | cut -f 2- -d '.' | rev)
    echo "Converting $line"
    inkscape $line --export-filename "$NEW_NAME.png"
    echo
done
