.PHONY: all
all: fmt ensure-deps test debug

### Create the database folder
.PHONY: generate-database
generate-database:
	@echo "=> Creating database"
	mkdir database

### Sync dependencies
.PHONY: ensure-deps
ensure-deps:
	@echo "=> Syncing dependencies"
	cargo check

### Testing
.PHONY: test
test:
	@echo "=> Running tests"
	# go test ./... -covermode=atomic -coverpkg=./... -count=1 -race

### Formatting, linting, and deps
.PHONY: fmt
fmt:
	@echo "=> Executing rustfmt"
	rustfmt ./server/src/*.rs;
	rustfmt ./server/src/application/*.rs;
	rustfmt ./server/src/auth/*.rs;
	#rustfmt ./server/src/db/*.rs;
	rustfmt ./server/src/model/repository/*.rs;
	rustfmt ./server/src/model/*.rs;

### Build with debug flags
.PHONY: debug
debug:
	@echo "=> Build debug mode"
	cargo build

### Build release
.PHONY: compile
compile:
	@echo "=> Build release"
	cargo build --release
### Run
.PHONY: run
run:
	@echo "==> Running local command..."
	cargo run

### Create docker image
.PHONY: create-image
create-image:
	@echo "==> Creating docker image..."
	docker build . -t asapp-challenge

### Run docker image
.PHONY: run-docker
run-docker:
	@echo "==> Runnning asapp docker container..."
	 docker run --name=asapp -p 8080:8080 asapp-challenge

## Run docker image
.PHONY: remove-docker
remove-docker:
	@echo "==> Removing asapp docker container..."
	docker rm asapp

## Stop docker image
.PHONY: stop-docker
stop-docker:
	@echo "==> Stopping asapp docker container..."
	docker stop asapp

## Start docker image
.PHONY: start-docker
start-docker:
	@echo "==> Starting asapp docker container..."
	docker start asapp

