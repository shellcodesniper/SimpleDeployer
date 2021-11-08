docker swarm init
docker network create --opt encrypted --driver overlay --attachable overlay

rm -rf /tmp/kuuwange/
mkdir -p /tmp/kuuwange/nginx/
mkdir -p /tmp/kuuwange/certs/

cp -rf ./bin/certs/ /tmp/kuuwange/certs/
cp -rf ./bin/nginx_template/ssl/ /tmp/kuuwange/nginx/
cp ./secret.cfg /tmp/kuuwange/config.cfg
cp ./bin/regenerate.sh /tmp/kuuwange/nginx/

docker build . -t shellcodesniper/simpledeploy:latest

docker run -it --rm --name deployer --network=overlay -v /tmp/kuuwange/:/tmp/kuuwange -v /var/run/docker.sock:/var/run/docker.sock shellcodesniper/simpledeploy:latest /tmp/kuuwange/config.cfg
