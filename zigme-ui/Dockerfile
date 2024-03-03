# docker build -t test-ui:latest . && docker run --env-file=../.env -p 3000:3000 test-ui:latest
FROM node:21.6-alpine3.19 AS builder

WORKDIR /app

COPY package*.json .

RUN npm ci

COPY . . 

RUN npm run build

RUN npm prune --production

FROM node:21.6-alpine3.19 AS runner

WORKDIR /app

COPY --from=builder /app/build build/

COPY --from=builder /app/node_modules node_modules/

COPY package.json .

ENV NODE_ENV=production

CMD ["node", "build"]
