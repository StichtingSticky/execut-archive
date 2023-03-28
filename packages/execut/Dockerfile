FROM rust:latest AS builder

WORKDIR /app
COPY . .
ENV SQLX_OFFLINE true
RUN --mount=type=cache,target=/app/target \
	--mount=type=cache,target=/usr/local/cargo/registry \
	--mount=type=cache,target=/usr/local/cargo/git \
	--mount=type=cache,target=/usr/local/rustup \
	set -eux; \
	rustup install stable; \
	cargo build --release; \
	objcopy --compress-debug-sections target/release/execut ./execut

FROM debian:bullseye-slim AS runner

RUN set -eux; \
	export DEBIAN_FRONTEND=noninteractive; \
	apt update; \
	apt install --yes --no-install-recommends bind9-dnsutils iputils-ping iproute2 curl ca-certificates htop; \
	apt clean autoclean; \
	apt autoremove --yes; \
	rm -rf /var/lib/{apt,dpkg,cache,log}/; \
	echo "Installed base utils!"

WORKDIR app

COPY --from=builder /app/execut ./execut
CMD ["./execut"]
