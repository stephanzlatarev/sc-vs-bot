FROM stephanzlatarev/starcraft

RUN apt-get update && apt-get install -y socat curl && rm -rf /var/lib/apt/lists/*
RUN curl -fsSL https://deb.nodesource.com/setup_lts.x | bash - && apt-get install -y nodejs && rm -rf /var/lib/apt/lists/*

COPY bots/ /bots/
COPY maps/ /StarCraftII/Maps/

COPY host.sh /host.sh
RUN chmod +x /host.sh

WORKDIR /node

COPY node/ ./
RUN npm install

ENTRYPOINT ["/host.sh"]
