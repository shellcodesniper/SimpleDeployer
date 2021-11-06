mkdir -p /tmp/kuuwange/certs/
cp ./config.cfg /tmp/kuuwange/
cp -rf ./certs/* /tmp/kuuwange/certs/

docker run -it --rm \
  -v /var/run/docker.sock:/var/run/docker.sock \
  -v /tmp/kuuwange/:/tmp/kuwange/\
  --name deployer \
  shellcodesniper/simpledeploy /tmp/kuuwange/config.cfg
