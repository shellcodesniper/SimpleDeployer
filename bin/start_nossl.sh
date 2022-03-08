docker swarm init
docker network create --opt encrypted --driver overlay --attachable overlay


sudo rm -rf /etc/kuuwange/
sudo mkdir -p /etc/kuuwange/nginx/
sudo mkdir -p /etc/kuuwange/certs/
sudo chown -R ${USER:=$(/usr/bin/id -run)}:$USER /etc/kuuwange/
mkdir -p $(pwd)/logs/

cp -rf ./nginx_template/nossl/* /etc/kuuwange/nginx/
cp ./config.cfg /etc/kuuwange/config.cfg
cp ./regenerate.sh /etc/kuuwange/nginx/
touch /etc/kuuwange/.env
cp .env /etc/kuuwange/.env

docker pull shellcodesniper/simpledeploy:stable

docker run -d --rm --name deployer \
  --network=overlay \
  -v /etc/kuuwange/:/etc/kuuwange \
  -v $(pwd)/logs/:/app/logs/ \
  -v /var/run/docker.sock:/var/run/docker.sock \
    shellcodesniper/simpledeploy:stable \
      /etc/kuuwange/config.cfg
