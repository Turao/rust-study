STORAGE_DIR=${PWD}/database/migrations
STORAGE_USER=test
STORAGE_PASSWORD=test
STORAGE_DATABASE=test

storage-start:
	docker run -it --rm --name storage -p 5432:5432 -e POSTGRES_DB=${STORAGE_DATABASE} -e POSTGRES_USER=${STORAGE_USER} -e POSTGRES_PASSWORD=${STORAGE_PASSWORD} postgres

storage-migrate-up:
	docker run -v ${STORAGE_DIR}:/migrations --network host migrate/migrate -path=/migrations/ -database postgres://${STORAGE_USER}:${STORAGE_PASSWORD}@localhost:5432/${STORAGE_DATABASE}?sslmode=disable -verbose up 1