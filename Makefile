DEV_COMPOSE = docker compose -p mybrigitte-dev --env-file .env.dev -f docker-compose.yml -f docker-compose.dev.yml
PROD_COMPOSE = docker compose -p mybrigitte-prod --env-file .env -f docker-compose.yml

.PHONY: dev dev-down dev-seed dev-reset-db prod prod-down config

.PHONY: hooks audit check check-quick

hooks:
	bash scripts/install-hooks.sh

audit:
	python3 scripts/check.py audit

check:
	python3 scripts/check.py full

check-quick:
	python3 scripts/check.py quick

dev:
	@test -f .env.dev || cp .env.dev.example .env.dev
	$(DEV_COMPOSE) up --build --watch

dev-down:
	$(DEV_COMPOSE) down

prod:
	@test -f .env || { echo 'Create .env from .env.example and configure production secrets first.'; exit 1; }
	$(PROD_COMPOSE) up -d --build --wait

prod-down:
	$(PROD_COMPOSE) down

config:
	$(DEV_COMPOSE) config --quiet
	$(PROD_COMPOSE) config --quiet

# Reload fixtures without deleting existing records.
dev-seed:
	@test -f .env.dev || cp .env.dev.example .env.dev
	$(DEV_COMPOSE) build migrate seed
	$(DEV_COMPOSE) run --rm migrate
	$(DEV_COMPOSE) run --rm seed

# Disposable development database only; preserves Rust/frontend cache volumes.
dev-reset-db:
	@test -f .env.dev || cp .env.dev.example .env.dev
	$(DEV_COMPOSE) down
	@if docker volume inspect mybrigitte-dev_postgres_data >/dev/null 2>&1; then docker volume rm mybrigitte-dev_postgres_data; fi
	$(DEV_COMPOSE) up -d --build --wait
