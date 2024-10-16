#!/usr/bin/env bash

set -e

PORT=65432

if [[ ! -f ".ready" ]]; then
  # Install Rust
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  . "$HOME/.cargo/env"

  # Postgres
  if [[ ! -d postgres ]]; then
    git clone https://github.com/postgres/postgres --branch=REL_17_STABLE --depth=1
  fi
  cd postgres
  ./configure --prefix=$(pwd)/build --without-icu --enable-cassert --without-readline
  make -j all && make install
  cd ..
  if [[ ! -d db ]]; then
    postgres/build/bin/initdb -D db -c port=$PORT
  fi
  if [[ -f db/postmaster.pid ]]; then
    postgres/build/bin/pg_ctl -D db -l logfile stop
  fi
  postgres/build/bin/pg_ctl -D db -l logfile start
  postgres/build/bin/createdb -e -p $PORT test || true
  touch .ready
fi

PG_URL=postgres://localhost:$PORT/test cargo run --release