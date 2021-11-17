docker swarm init
docker network create --opt encrypted --driver overlay --attachable overlay

rm -rf /tmp/kuuwange/
mkdir -p /tmp/kuuwange/nginx/
mkdir -p /tmp/kuuwange/certs/

cp -rf ./certs/* /tmp/kuuwange/certs/
cp -rf ./nginx_template/ssl/* /tmp/kuuwange/nginx/
cp ./config.cfg /tmp/kuuwange/config.cfg
cp ./regenerate.sh /tmp/kuuwange/nginx/

docker pull shellcodesniper/simpledeploy:stable

docker run -d --rm --name deployer \
  --network=overlay \
  -v /tmp/kuuwange/:/tmp/kuuwange \
  -v $(pwd)/logs:/app/logs \
  -v /var/run/docker.sock:/var/run/docker.sock \
    shellcodesniper/simpledeploy:stable \
      /tmp/kuuwange/config.cfg
