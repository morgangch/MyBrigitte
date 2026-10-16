DEV_COMPOSE = docker compose -p mybrigitte-dev --env-file .env.dev -f docker-compose.yml -f docker-compose.dev.yml
PROD_COMPOSE = docker compose -p mybrigitte-prod --env-file .env -f docker-compose.yml

# The progress UI needs Python 3.8+; without it (or with PLAIN=1) Compose prints its own output.
PYTHON := $(shell python3 -c 'import sys; sys.exit(sys.version_info < (3, 8))' >/dev/null 2>&1 && echo python3)
# $(call stack,<mode and flags>,<compose>,<fallback up arguments>)
stack = $(if $(PYTHON),$(PYTHON) scripts/stack.py $(1) $(2),$(2) $(3))
say = printf '\033[1;36m▸\033[0m %s\n'

s ?= backend
export PLAIN

.DEFAULT_GOAL := help
.PHONY: help dev dev-up dev-down dev-seed dev-reset-db logs shell ps prod prod-down prod-logs \
	config hooks audit check check-quick prune

##@ Development

dev: .env.dev ## Build and start dev with hot reload, then follow logs (Ctrl+C stops)
	@$(call stack,dev,$(DEV_COMPOSE),up --build --watch)

dev-up: .env.dev ## Build and start dev in the background
	@$(call stack,dev --detach,$(DEV_COMPOSE),up -d --build --wait)

dev-down: ## Stop dev; data and build caches are kept
	@$(DEV_COMPOSE) down

dev-seed: .env.dev ## Apply migrations and replay fixtures without overwriting rows
	@$(say) 'Building migrate and seed'
	@$(DEV_COMPOSE) build --quiet migrate seed
	@$(say) 'Applying migrations'
	@$(DEV_COMPOSE) run --rm migrate
	@$(say) 'Loading fixtures'
	@$(DEV_COMPOSE) run --rm seed

dev-reset-db: .env.dev ## DELETE the dev database, then recreate and seed it
	@$(DEV_COMPOSE) down
	@if docker volume inspect mybrigitte-dev_postgres_data >/dev/null 2>&1; then \
		$(say) 'Removing mybrigitte-dev_postgres_data'; docker volume rm mybrigitte-dev_postgres_data >/dev/null; fi
	@$(call stack,dev --detach,$(DEV_COMPOSE),up -d --build --wait)

logs: ## Follow dev logs (all services, or s=<service>)
	@$(DEV_COMPOSE) logs --follow --tail 100 $(if $(filter command line,$(origin s)),$(s))

shell: ## Open a shell in a running dev container (s=backend by default)
	@$(DEV_COMPOSE) exec $(s) sh

ps: ## Show dev and prod containers
	@$(say) 'Development'
	@$(DEV_COMPOSE) ps --all
	@echo
	@$(say) 'Production'
	@$(PROD_COMPOSE) ps --all

##@ Production

prod: ## Build and start production in the background
	@test -f .env || { printf '\033[31m✘\033[0m Create .env from .env.example and configure production secrets first.\n'; exit 1; }
	@$(call stack,prod,$(PROD_COMPOSE),up -d --build --wait)

prod-down: ## Stop production; data is kept
	@$(PROD_COMPOSE) down

prod-logs: ## Follow production logs (all services, or s=<service>)
	@$(PROD_COMPOSE) logs --follow --tail 100 $(if $(filter command line,$(origin s)),$(s))

##@ Quality

hooks: ## Install the Git hooks
	@bash scripts/install-hooks.sh

check-quick: ## Fast checks in the Docker tooling image
	@bash scripts/check-docker.sh quick

check: ## Complete local gate
	@bash scripts/check-docker.sh full

audit: ## RustSec and Bun audits
	@bash scripts/check-docker.sh audit

config: ## Validate both Compose configurations
	@$(DEV_COMPOSE) config --quiet && $(PROD_COMPOSE) config --quiet && $(say) 'Compose configurations are valid'

##@ Maintenance

prune: ## Drop Docker build cache unused for 7 days (volumes are kept)
	@docker builder prune --force --filter until=168h

.env.dev:
	@cp .env.dev.example .env.dev
	@$(say) 'Created .env.dev from .env.dev.example'

help: ## Show this help
	@awk 'BEGIN { FS = ":.*## " } \
		/^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } \
		/^[a-z-]+:.*## / { printf "  \033[36m%-14s\033[0m %s\n", $$1, $$2 }' $(MAKEFILE_LIST)
	@printf '\nPLAIN=1 shows the raw Compose output instead of progress bars.\n'
