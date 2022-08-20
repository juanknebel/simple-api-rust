.PHONY: all
all: fmt deps test debug

### Create the database folder and the databases file for sqlite
.PHONY: generate-database
generate-database:
	@echo "=> Creating database"
	mkdir database; touch database/testing_db.db; touch database/production_db.db

### Sync dependencies
.PHONY: deps
deps:
	@echo "=> Syncing dependencies"
	cargo check

### Testing
.PHONY: test
test:
	@echo "=> Running tests"
	cargo test

### Formatting, linting, and deps
.PHONY: fmt
fmt:
	@echo "=> Executing rustfmt"
	rustfmt ./server/src/*.rs;
	rustfmt ./server/src/application/*.rs;
	rustfmt ./server/src/auth/*.rs;
	rustfmt ./server/src/db/*.rs;
	rustfmt ./server/src/log/*.rs;
	rustfmt ./server/src/model/repository/*.rs;
	rustfmt ./server/src/model/*.rs;
	rustfmt ./server/src/openapi/*.rs;

### Build with debug flags
.PHONY: debug
debug:
	@echo "=> Build debug mode"
	cargo build

### Build release
.PHONY: release
release:
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
	@echo "==> todo!..."

### Run docker image
.PHONY: run-docker
run-docker:
	@echo "==> todo!..."

## Run docker image
.PHONY: remove-docker
remove-docker:
	@echo "==> todo!..."

## Stop docker image
.PHONY: stop-docker
stop-docker:
	@echo "==> todo!..."

## Start docker image
.PHONY: start-docker
start-docker:
	@echo "==> todo!..."
