CONTAINER_NAME="saga-universe-pdf-import"

build:
	docker-compose build

start:
	docker-compose down && docker-compose up -d

init:
	docker exec -it ${CONTAINER_NAME} bash