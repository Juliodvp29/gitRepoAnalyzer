# ─── Compilación ─────────────────────────────────────────
FROM rust:1.86-slim AS builder

# Instalar dependencias necesarias para compilar
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copiar solo los archivos de dependencias primero
COPY Cargo.toml Cargo.lock ./

# Compilar dependencias vacías para cachear
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copiar el código fuente real y compilar
COPY src ./src
RUN touch src/main.rs && cargo build --release

# ─── Runtime ─────────────────────────────────────────────
FROM debian:bookworm-slim

# Instalar git (necesario para clonar repos) y certificados SSL
RUN apt-get update && apt-get install -y \
    git \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copiar solo el binario compilado de la etapa anterior
COPY --from=builder /app/target/release/ghan-backend .

# Exponer el puerto
EXPOSE 3000

# Comando de arranque
CMD ["./ghan-backend"]