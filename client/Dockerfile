FROM stephanzlatarev/starcraft

RUN apt-get update && apt-get install -y socat curl && rm -rf /var/lib/apt/lists/*
RUN curl -fsSL https://deb.nodesource.com/setup_lts.x | bash - && apt-get install -y nodejs && rm -rf /var/lib/apt/lists/*

COPY maps/LeyLinesAIE_v3.SC2Map /StarCraftII/Maps/LeyLinesAIE_v3.SC2Map

COPY client.sh /client.sh
RUN chmod +x /client.sh

WORKDIR /node

COPY node/ ./
RUN npm install

ENTRYPOINT ["/client.sh"]
