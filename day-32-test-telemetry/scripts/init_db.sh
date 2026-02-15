
# #!/usr/bin/env bash
# set -x
# set -eo pipefail

# # let’s check that sqlx-cli is installed
# if ! [ -x "$(command -v sqlx)" ]; then
# echo >&2 "Error: sqlx is not installed."
# echo >&2 "Use:"
# echo >&2 " cargo install --version='~0.8' sqlx-cli \
# --no-default-features --features rustls,postgres"
# echo >&2 "to install it."
# exit 1
# fi

# # Check if a custom parameter has been set, otherwise use default values
# DB_PORT="${POSTGRES_PORT:=5432}"
# SUPERUSER="${SUPERUSER:=postgres}"
# SUPERUSER_PWD="${SUPERUSER_PWD:=password}"
# APP_USER="${APP_USER:=app}"
# APP_USER_PWD="${APP_USER_PWD:=secret}"
# APP_DB_NAME="${APP_DB_NAME:=newsletter}"

# # Launch postgres using Docker
# CONTAINER_NAME="postgres"
# docker run \
# --env POSTGRES_USER=${SUPERUSER} \
# --env POSTGRES_PASSWORD=${SUPERUSER_PWD} \
# --health-cmd="pg_isready -U ${SUPERUSER} || exit 1" \
# --health-interval=1s \
# --health-timeout=5s \
# --health-retries=5 \
# --publish "${DB_PORT}":5432 \
# --detach \
# --name "${CONTAINER_NAME}" \
# postgres -N 1000
# # ^ Increased maximum number of connections for testing purposes

# # Wait for Postgres to be ready to accept connections
# until [ \
# "$(docker inspect -f "{{.State.Health.Status}}" ${CONTAINER_NAME})" == \
# "healthy" \
# ]; do
# >&2 echo "Postgres is still unavailable - sleeping"
# sleep 1
# done
# >&2 echo "Postgres is up and running on port ${DB_PORT}!"

# # Create the application user
# CREATE_QUERY="CREATE USER ${APP_USER} WITH PASSWORD '${APP_USER_PWD}';"
# docker exec -it "${CONTAINER_NAME}" psql -U "${SUPERUSER}" -c "${CREATE_QUERY}"
# # Grant create db privileges to the app user
# GRANT_QUERY="ALTER USER ${APP_USER} CREATEDB;"
# docker exec -it "${CONTAINER_NAME}" psql -U "${SUPERUSER}" -c "${GRANT_QUERY}"

# # To store our subscribers details we need to create our first table.
# DATABASE_URL=postgres://${APP_USER}:${APP_USER_PWD}@localhost:${DB_PORT}/${APP_DB_NAME}
# export DATABASE_URL
# sqlx database create



# ⛳️ Step 3 - View DB in a GUI client

# Now that we've run our first migration lets add `sqlx migrate run to sh` to our script file

# Lets also modify the script to make it easy to run migrations against an existing Postgres instance without having to tear it down manually and re-create it every time we start teh program

#!/usr/bin/env bash
set -x
set -eo pipefail

# let’s check that sqlx-cli is installed
if ! [ -x "$(command -v sqlx)" ]; then
echo >&2 "Error: sqlx is not installed."
echo >&2 "Use:"
echo >&2 " cargo install --version='~0.8' sqlx-cli \
--no-default-features --features rustls,postgres"
echo >&2 "to install it."
exit 1
fi

# Check if a custom parameter has been set, otherwise use default values
DB_PORT="${POSTGRES_PORT:=5432}"
SUPERUSER="${SUPERUSER:=postgres}"
SUPERUSER_PWD="${SUPERUSER_PWD:=password}"
APP_USER="${APP_USER:=app}"
APP_USER_PWD="${APP_USER_PWD:=secret}"
APP_DB_NAME="${APP_DB_NAME:=newsletter}"

# Allow to skip Docker if a dockerized Postgres database is already running
if [[ -z "${SKIP_DOCKER}" ]]
then
# Launch postgres using Docker
CONTAINER_NAME="postgres"
docker run \
--env POSTGRES_USER=${SUPERUSER} \
--env POSTGRES_PASSWORD=${SUPERUSER_PWD} \
--health-cmd="pg_isready -U ${SUPERUSER} || exit 1" \
--health-interval=1s \
--health-timeout=5s \
--health-retries=5 \
--publish "${DB_PORT}":5432 \
--detach \
--name "${CONTAINER_NAME}" \
postgres -N 1000
# ^ Increased maximum number of connections for testing purposes
until [ \
"healthy" \
]; do
sleep 1
done

# Wait for Postgres to be ready to accept connections
until [ \
"$(docker inspect -f "{{.State.Health.Status}}" ${CONTAINER_NAME})" == \
"healthy" \
]; do
>&2 echo "Postgres is still unavailable - sleeping"
sleep 1
done


# Create the application user
CREATE_QUERY="CREATE USER ${APP_USER} WITH PASSWORD '${APP_USER_PWD}';"
docker exec -it "${CONTAINER_NAME}" psql -U "${SUPERUSER}" -c "${CREATE_QUERY}"
# Grant create db privileges to the app user
GRANT_QUERY="ALTER USER ${APP_USER} CREATEDB;"
docker exec -it "${CONTAINER_NAME}" psql -U "${SUPERUSER}" -c "${GRANT_QUERY}"
fi

>&2 echo "Postgres is up and running on port ${DB_PORT}!"

# To store our subscribers details we need to create our first table.
DATABASE_URL=postgres://${APP_USER}:${APP_USER_PWD}@localhost:${DB_PORT}/${APP_DB_NAME}
export DATABASE_URL
sqlx database create
sqlx migrate run
>&2 echo "Postgres has been migrated, ready to go!"


# Finally If you check your database using your favourite graphic interface for Postgres you will now see a sub-scriptions table alongside a brand new _sqlx_migrations table: this is where sqlx keeps track of what migrations have been run against your database - it should contain a single row now for our create_subscriptions_table migration.

# I went for https://www.beekeeperstudio.io/get:
# Connection Type: Postgres
# Authentication Method: Username / Password
# User: app
# Password: secret
# Default Database: newsletter
