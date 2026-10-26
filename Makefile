DEV_COMPOSE = docker compose -p mybrigitte-dev --env-file .env.dev -f docker-compose.yml -f docker-compose.dev.yml
PROD_COMPOSE = docker compose -p mybrigitte-prod --env-file .env -f docker-compose.yml

# Recipes stay valid in both sh and cmd.exe. Native Windows make (from PowerShell
# or cmd) always uses cmd.exe, whatever sh.exe happens to be in PATH.
ifeq ($(OS)$(findstring msys,$(MAKE_HOST))$(findstring cygwin,$(MAKE_HOST)),Windows_NT)
SHELL := cmd.exe
NULL := NUL
COPY := copy /Y
GIT_EXEC := $(shell git --exec-path 2>NUL)
# Git Bash, never the WSL launcher that `bash` may resolve to.
BASH := $(if $(GIT_EXEC),"$(GIT_EXEC)/../../../bin/bash.exe",bash)
else
NULL := /dev/null
COPY := cp
BASH := bash
endif

# The progress UI needs Python 3.8+; without it (or with PLAIN=1) Compose prints its own output.
probe = $(shell $(1) -c "import sys; sys.exit(sys.version_info < (3, 8))" >$(NULL) 2>&1 && echo $(1))
ifeq ($(NULL),NUL)
PYTHON := $(or $(call probe,py),$(call probe,python),$(call probe,python3))
else
PYTHON := $(call probe,python3)
endif
# $(call stack,<mode and flags>,<compose>,<fallback up arguments>)
stack = $(if $(PYTHON),$(PYTHON) scripts/stack.py $(1) $(2),$(2) $(3))

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
	@echo Building migrate and seed
	@$(DEV_COMPOSE) build --quiet migrate seed
	@echo Applying migrations
	@$(DEV_COMPOSE) run --rm migrate
	@echo Loading fixtures
	@$(DEV_COMPOSE) run --rm seed

dev-reset-db: .env.dev ## DELETE the dev database, then recreate and seed it
	@$(DEV_COMPOSE) down
	@docker volume rm mybrigitte-dev_postgres_data >$(NULL) 2>&1 && echo Removed the dev database volume || echo No dev database volume to remove
	@$(call stack,dev --detach,$(DEV_COMPOSE),up -d --build --wait)

logs: ## Follow dev logs (all services, or s=<service>)
	@$(DEV_COMPOSE) logs --follow --tail 100 $(if $(filter command line,$(origin s)),$(s))

shell: ## Open a shell in a running dev container (s=backend by default)
	@$(DEV_COMPOSE) exec $(s) sh

ps: ## Show dev and prod containers
	@echo Development:
	@$(DEV_COMPOSE) ps --all
	@echo Production:
	@$(PROD_COMPOSE) ps --all

##@ Production

prod: ## Build and start production in the background
	@$(if $(wildcard .env),,$(error Create .env from .env.example and configure production secrets first))
	@$(call stack,prod,$(PROD_COMPOSE),up -d --build --wait)

prod-down: ## Stop production; data is kept
	@$(PROD_COMPOSE) down

prod-logs: ## Follow production logs (all services, or s=<service>)
	@$(PROD_COMPOSE) logs --follow --tail 100 $(if $(filter command line,$(origin s)),$(s))

##@ Quality

hooks: ## Install the Git hooks
	@$(BASH) scripts/install-hooks.sh

check-quick: ## Fast checks in the Docker tooling image
	@$(BASH) scripts/check-docker.sh quick

check: ## Complete local gate
	@$(BASH) scripts/check-docker.sh full

audit: ## RustSec and Bun audits
	@$(BASH) scripts/check-docker.sh audit

config: ## Validate both Compose configurations
	@$(DEV_COMPOSE) config --quiet && $(PROD_COMPOSE) config --quiet && echo Compose configurations are valid

##@ Maintenance

prune: ## Drop Docker build cache unused for 7 days (volumes are kept)
	@docker builder prune --force --filter until=168h

.env.dev:
	@$(COPY) .env.dev.example .env.dev >$(NULL)
	@echo Created .env.dev from .env.dev.example

help: ## Show this help
ifeq ($(NULL),NUL)
	@$(if $(PYTHON),$(PYTHON) scripts/stack.py help Makefile,findstr /C:"##" Makefile)
else
	@awk 'BEGIN { FS = ":.*## " } \
		/^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } \
		/^[a-z-]+:.*## / { printf "  \033[36m%-14s\033[0m %s\n", $$1, $$2 }' $(MAKEFILE_LIST)
	@printf '\nmake <target> PLAIN=1 shows the raw Compose output instead of progress bars.\n'
endif
