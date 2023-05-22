#!/bin/bash

for file in pieces/*.svg
do
     convert -antialias -background none -resize 1024x1024 -density 1270x1270  "${file}" "${file%.svg}.png"
done
