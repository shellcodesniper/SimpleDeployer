#!/bin/zsh

docker run -it -p 443:443 -p 80:80 --rm --name nginx \
-v $(pwd)/nginx/certs/:/etc/nginx/certs/ -v $(pwd)/nginx/default.conf:/etc/nginx/conf.d/default.conf -v $(pwd)/nginx/nginx.conf:/etc/nginx/nginx.conf \
nginx:alpine