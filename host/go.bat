
docker rm -f sc-vs-bot-host

docker build -t sc-vs-bot-host .

docker run -d --name sc-vs-bot-host -p 8000:8000 -p 10044:10044 -p 10055:10055 sc-vs-bot-host

docker logs -f sc-vs-bot-host
