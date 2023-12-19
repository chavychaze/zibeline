### SAMPLE FILE ###

# Development Stage
FROM cmake:3.21.1 AS build-dev

WORKDIR /app

COPY . .

# Build for UNIX (development)
RUN mkdir build_dev \
    && cd build_dev \
    && cmake -DCMAKE_BUILD_TYPE=Debug .. \
    && make

FROM build-dev AS build-prod

# Build for UNIX (production)
RUN mkdir build_prod \
    && cd build_prod \
    && cmake -DCMAKE_BUILD_TYPE=Release .. \
    && make

# Build for Windows (production)
RUN apt-get update \
    && apt-get install -y \
        mingw-w64 \
    && mkdir build_windows \
    && cd build_windows \
    && cmake -DCMAKE_TOOLCHAIN_FILE=../cross-mingw.cmake -DCMAKE_BUILD_TYPE=Release .. \
    && make

FROM build-prod

EXPOSE 8080

USER user

# Command to run the application as a service in the background
CMD ["./build_prod/my_app", "&", "tail", "-f", "/dev/null"]