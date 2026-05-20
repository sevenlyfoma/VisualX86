#docker build .

FROM rust AS build1

# set a directory for the app
WORKDIR /usr/src/app

# copy all the files to the container
COPY . .

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh


RUN wasm-pack build model --target web

RUN cp -r ./model/pkg ./web/static/pkg

FROM node:16.20.2 AS build2

COPY --from=0 /usr/src/app /usr/src/app

WORKDIR /usr/src/app/web

RUN npm install

EXPOSE 8080

CMD ["node", "index.js"]


