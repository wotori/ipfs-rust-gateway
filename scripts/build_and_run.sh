echo "____Building image..."
sudo docker build --tag ipfs-rust .
echo "____Stopping old container..."
sudo docker stop ipfs-rust
echo "____Removing old container..."
sudo docker rm ipfs-rust
echo "____Starting service as a new container..."
sudo docker run -d --name ipfs-rust -p 1717:8000 -v ~/ipfs/:/files ipfs-rust