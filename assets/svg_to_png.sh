#!/bin/bash

for file in pieces/*.svg
do
     convert -antialias -background none -resize 256x256 -density 1270x1270  "${file}" "${file%.svg}.png"
done
