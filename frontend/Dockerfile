FROM node:lts

RUN apt update && \
    apt install -y openjdk-8-jdk && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/rucktar

ADD package.json yarn.lock* /usr/local/rucktar/

RUN yarn install

CMD npx shadow-cljs watch browser
