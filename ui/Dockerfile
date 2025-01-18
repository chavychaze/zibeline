# Build Stage
FROM node:20-alpine as builder

WORKDIR /app

# Install yew dependencies
RUN apk add --no-cache git curl build-base pkg-config
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN cargo install trunk wasm-bindgen-cli

COPY ./ui /app

RUN trunk build --release

# Production Stage
FROM nginx:alpine

COPY --from=builder /app/dist /usr/share/nginx/html

COPY ./ui/nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]