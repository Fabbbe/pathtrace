#!/bin/sh

inotifywait -q -m -e close_write out.ppm |
while read -r filename event; do 
  img2sixel ./out.ppm
done
