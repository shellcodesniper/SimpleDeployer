docker swarm init
docker network create --opt encrypted --driver overlay --attachable overlay

sudo rm -rf /etc/kuuwange/
sudo mkdir -p /etc/kuuwange/nginx/
sudo mkdir -p /etc/kuuwange/certs/
sudo chown -R ${USER:=$(/usr/bin/id -run)}:$USER /etc/kuuwange/
mkdir -p $(pwd)/logs/

cp -rf ./certs/* /etc/kuuwange/certs/
cp -rf ./nginx_template/ssl/* /etc/kuuwange/nginx/
cp ./config.cfg /etc/kuuwange/config.cfg
cp ./regenerate.sh /etc/kuuwange/nginx/

docker pull shellcodesniper/simpledeploy:arm
docker run -d --name deployer \
  --network=overlay \
  -v /etc/kuuwange/:/etc/kuuwange \
  -v /var/run/docker.sock:/var/run/docker.sock \
  -v $(pwd)/logs/:/app/logs \
    shellcodesniper/simpledeploy:arm \
      /etc/kuuwange/config.cfg