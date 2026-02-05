# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
#   MCP STDIO WRAPPER - Makefile
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
#
# Description: Build & automation helpers for the MCP STDIO Wrapper project
# Usage: run `make` or `make help` to view available targets
#
# help: MCP STDIO WRAPPER
#
# ──────────────────────────────────────────────────────────────────────────
SHELL := /bin/bash
.SHELLFLAGS := -eu -o pipefail -c

# =============================================================================
# DYNAMIC HELP
# =============================================================================
.PHONY: help
help:
	@grep "^# help\:" Makefile | grep -v grep | sed 's/\# help\: //' | sed 's/\# help\://'

# =============================================================================
# CARGO CHECKS
# =============================================================================
# help: check              - Run cargo check on the project
# help: check-test         - Run cargo check on tests
# help: pedantic           - Run clippy with pedantic lints on src/
# help: pedantic-test      - Run clippy with pedantic lints on tests/
# help: coverage           - Generate code coverage report with llvm-cov

.PHONY: cargo-check cargo-check-tests clippy-pedantic-src clippy-pedantic-tests

check chk c:
	@echo "Running cargo check..."
	@cargo check

check-test check-tests chkt ct:
	@echo "Running cargo check on tests..."
	@cargo check --tests

pedantic ped p:
	@echo "Running clippy with pedantic lints on src/..."
	@cargo clippy --lib --bins -- -W clippy::pedantic
	
pedantic-test pedantic-tests pedt pt:
	@echo "Running clippy with pedantic lints on tests/..."
	@cargo clippy --tests -- -W clippy::pedantic

coverage cov cv:
	@echo "Generating code coverage report..."
	@cargo install cargo-llvm-cov
	@rustup component add llvm-tools-preview
	@cargo llvm-cov --html --ignore-filename-regex "src/bin/.*"
	@echo "Coverage report generated at target/llvm-cov/html/index.html"
	@firefox target/llvm-cov/html/index.html