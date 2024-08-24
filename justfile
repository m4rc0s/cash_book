dev-pod-name := "caderninho-dev"
dev-db-name := "caderninhodb"
dev-db-user:= "root"
dev-db-password := "root"

run-dev: setup-pod-dev run-pg-dev

setup-pod-dev:
	@podman pod create \
		--name {{dev-pod-name}} \
		-p 8080:8080 \
		-p 8081:80 \
		-p 8082:9000 \
		-p 8083:9001 \
		-p 5432:5432

	mkdir -p pg-data

run-pg-dev:
	@podman run -d \
		--name {{dev-db-name}} \
		--pod {{dev-pod-name}} \
		--restart unless-stopped \
		-e POSTGRES_PASSWORD={{dev-db-password}} \
		-e POSTGRES_USER={{dev-db-user}} \
		-v ${PWD}/pg-data:/var/lib/postgresql/data \
		docker.io/postgres:16
