# イメージ名とコンテナ名
IMAGE_NAME := backend-rust-backend
CONTAINER_NAME := backend-rust-backend-1

# Docker コマンド
.PHONY: build up down logs exec stop restart clean tree
build:
	docker-compose build

up:
	docker-compose up -d

down:
	docker-compose down

logs:
	docker-compose logs -f

exec:
	docker-compose exec $(CONTAINER_NAME) bash

stop:
	docker-compose stop

restart:
	docker-compose restart

clean:
	docker-compose down --rmi all --volumes # イメージとボリュームを削除

tree:
	tree -I 'target|.*' # target ディレクトリと . で始まるファイルを無視してツリー表示