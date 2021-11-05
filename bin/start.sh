mkdir -p /tmp/kuuwange/nginx/
cp ./sampleConfig.cfg /tmp/kuuwange/
docker run -it --rm -v /tmp/kuuwange/:/shared_dir --name deployer shellcodesniper/simpledeploy /shared_dir/sampleConfig.cfg