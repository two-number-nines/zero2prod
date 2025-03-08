#!/usr/bin/env bash

set -x
set -eo pipefail

DB_USER=${POSTGRES_USER:=postgres}

if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: psql is not installed"
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed"
  exit 1
fi


DB_PASSWORD=${POSTGRES_PASSWORD:=password}

DB_NAME=${POSTGRES_DB:=newsletter}

DB_PORT=${POSTGRES_PORT:=5432}

if [[ -z "${SKIP_DOCKER}" ]]; then
docker run \
  -e POSTGRES_USER="${DB_USER}" \
  -e POSTGRES_PASSWORD="${DB_PASSWORD}" \
  -e POSTGRES_DB="${DB_NAME}" \
  -p "${DB_PORT}:${DB_PORT}" \
  -d postgres \
  postgres -N 1000
fi

 DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}

export PGPASSWORD=${DB_PASSWORD}
# TODO: this thing sucks, add more specific error
# error example: handling psql: error: connection to server at "localhost" (::1), port 5432 failed: FATAL:  database "newsletter" does not exist
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "${DB_NAME}" -c '\q'; do
   >&2 echo "Postgres still unavailable - sleeping"
   sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT}"

export DATABASE_URL
sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go"