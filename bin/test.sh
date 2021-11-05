mkdir -p /tmp/kuuwange/nginx/
cp -rf ./bin/nginx_template/nossl/ /tmp/kuuwange/nginx/
cp secret.cfg /tmp/kuuwange/config.cfg

cargo r /tmp/kuuwange/config.cfg
