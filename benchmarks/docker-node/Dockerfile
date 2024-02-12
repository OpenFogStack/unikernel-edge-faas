FROM alpine

WORKDIR /root
COPY app-node/* ./

RUN apk add nodejs

ENTRYPOINT ["node", "/root/main.js"]
