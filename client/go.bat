
docker rm -f sc-vs-bot-client

docker build -t sc-vs-bot-client .

docker run -d --name sc-vs-bot-client -p 10001:10001 sc-vs-bot-client

docker logs -f sc-vs-bot-client
