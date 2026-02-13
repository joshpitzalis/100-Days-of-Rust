<!--1/ run for permissions to execute the script-->
chmod +x scripts/init_db.sh

<!--2/ then launch Postgres with-->
./scripts/init_db.sh

<!--3. then remove it jus so you know how to -->
docker rm -f postgres

<!--4/ then launch Postgres again so we can use it-->
./scripts/init_db.sh
