FROM node AS build

WORKDIR /build
COPY . .
RUN yarn install && yarn run tsc

FROM node:alpine

WORKDIR /app
COPY package.json yarn.lock .env ./
RUN yarn install --production --frozen-lockfile
COPY --from=build /build/dist ./

CMD ["node", "src/main.js"]