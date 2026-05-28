EXTRACT_DIR = fixtures
LEAN_SPEC_DEVNET ?= devnet4
LEAN_SPEC_DEVNET4_LEANSPEC_COMMIT = db21cc24cbeeb78a91c42a713089fe0c845eb648
LEAN_SPEC_DEVNET4_LOCAL_FIXTURES ?= ../../../lean-spec-tests
LEAN_SPEC_DEVNET4_TESTS_REF ?= a3bf5d72e74e3bdf7634e5ee3a8154aabf0a8565
LEAN_SPEC_DEVNET4_TESTS_ARCHIVE_URL = https://github.com/ReamLabs/lean-spec-tests/archive/$(LEAN_SPEC_DEVNET4_TESTS_REF).tar.gz
LEAN_SPEC_RELEASE_TAG ?= latest
LEAN_SPEC_FIXTURES_ASSET = fixtures-prod-scheme.tar.gz
FIXTURES_URL = https://github.com/leanEthereum/leanSpec/releases/download/$(LEAN_SPEC_RELEASE_TAG)/$(LEAN_SPEC_FIXTURES_ASSET)
FIXTURES_SHA256_URL = $(FIXTURES_URL).sha256
KEYS_DIR = fixtures/keys/prod_scheme
KEYS_URL = https://github.com/leanEthereum/leansig-test-keys/releases/download/latest/prod_scheme.tar.gz

FIXTURES := state_transition ssz fork_choice justifiability slot_clock sync verify_signatures
SELECTED := $(filter $(FIXTURES),$(MAKECMDGOALS))

.PHONY: all clean test $(EXTRACT_DIR) $(FIXTURES)

all: test

$(EXTRACT_DIR):
	@if [ -d $(EXTRACT_DIR)/consensus ]; then \
		echo "$(EXTRACT_DIR) already exists. Skipping download."; \
	elif [ "$(LEAN_SPEC_DEVNET)" = "devnet4" ]; then \
		echo "Installing devnet4 fixtures generated from leanSpec $(LEAN_SPEC_DEVNET4_LEANSPEC_COMMIT)..."; \
		rm -rf $(EXTRACT_DIR); \
		mkdir -p $(EXTRACT_DIR); \
		if [ -d "$(LEAN_SPEC_DEVNET4_LOCAL_FIXTURES)/consensus" ]; then \
			cp -R "$(LEAN_SPEC_DEVNET4_LOCAL_FIXTURES)/consensus" "$(EXTRACT_DIR)/consensus"; \
		else \
			tmp_dir=$$(mktemp -d); \
			trap 'rm -rf "$$tmp_dir"' EXIT; \
			curl -4 -fsSL "$(LEAN_SPEC_DEVNET4_TESTS_ARCHIVE_URL)" -o "$$tmp_dir/lean-spec-tests.tar.gz" || { echo "devnet4 fixture download failed."; exit 1; }; \
			tar --warning=no-unknown-keyword -xzf "$$tmp_dir/lean-spec-tests.tar.gz" --strip-components=1 -C "$$tmp_dir" || { echo "devnet4 fixture extraction failed."; exit 1; }; \
			cp -R "$$tmp_dir/consensus" "$(EXTRACT_DIR)/consensus" || { echo "devnet4 fixture archive did not contain consensus/."; exit 1; }; \
		fi; \
		echo "devnet4 fixtures installed successfully."; \
	else \
		echo "Downloading devnet5 leanSpec fixtures release $(LEAN_SPEC_RELEASE_TAG)..."; \
		tmp_dir=$$(mktemp -d); \
		trap 'rm -rf "$$tmp_dir"' EXIT; \
		rm -rf $(EXTRACT_DIR); \
		mkdir -p $(EXTRACT_DIR); \
		curl -4 -fsSL $(FIXTURES_URL) -o "$$tmp_dir/$(LEAN_SPEC_FIXTURES_ASSET)" || { echo "Fixture download failed."; exit 1; }; \
		curl -4 -fsSL $(FIXTURES_SHA256_URL) -o "$$tmp_dir/$(LEAN_SPEC_FIXTURES_ASSET).sha256" || { echo "Fixture checksum download failed."; exit 1; }; \
		(cd "$$tmp_dir" && sha256sum -c "$(LEAN_SPEC_FIXTURES_ASSET).sha256") || { echo "Fixture checksum verification failed."; exit 1; }; \
		tar --warning=no-unknown-keyword -xzf "$$tmp_dir/$(LEAN_SPEC_FIXTURES_ASSET)" --strip-components=1 -C $(EXTRACT_DIR) || { echo "Fixture extraction failed."; exit 1; }; \
		echo "Fixtures downloaded successfully."; \
	fi

$(KEYS_DIR):
	@if [ -d $(KEYS_DIR) ]; then \
		echo "$(KEYS_DIR) already exists. Skipping download."; \
	elif [ "$(LEAN_SPEC_DEVNET)" = "devnet4" ]; then \
		echo "Installing devnet4 prod keys generated from leanSpec $(LEAN_SPEC_DEVNET4_LEANSPEC_COMMIT)..."; \
		mkdir -p fixtures/keys; \
		if [ -d "$(LEAN_SPEC_DEVNET4_LOCAL_FIXTURES)/keys/prod_scheme" ]; then \
			cp -R "$(LEAN_SPEC_DEVNET4_LOCAL_FIXTURES)/keys/prod_scheme" "$(KEYS_DIR)"; \
		else \
			tmp_dir=$$(mktemp -d); \
			trap 'rm -rf "$$tmp_dir"' EXIT; \
			curl -4 -fsSL "$(LEAN_SPEC_DEVNET4_TESTS_ARCHIVE_URL)" -o "$$tmp_dir/lean-spec-tests.tar.gz" || { echo "devnet4 key download failed."; exit 1; }; \
			tar --warning=no-unknown-keyword -xzf "$$tmp_dir/lean-spec-tests.tar.gz" --strip-components=1 -C "$$tmp_dir" || { echo "devnet4 key extraction failed."; exit 1; }; \
			cp -R "$$tmp_dir/keys/prod_scheme" "$(KEYS_DIR)" || { echo "devnet4 fixture archive did not contain keys/prod_scheme/."; exit 1; }; \
		fi; \
		echo "devnet4 keys installed successfully."; \
	else \
		echo "Downloading devnet5 test keys..."; \
		mkdir -p fixtures/keys; \
		curl -sL $(KEYS_URL) | tar xz -C fixtures/keys || { echo "Key download failed."; exit 1; }; \
		echo "Test keys downloaded successfully."; \
	fi

test: $(EXTRACT_DIR) $(KEYS_DIR)
	@echo "Running lean-spec tests..."
ifeq ($(SELECTED),)
	@if [ "$(LEAN_SPEC_DEVNET)" = "devnet5" ]; then \
		cargo test --release --no-default-features --features lean-spec-tests,devnet5; \
	else \
		cargo test --release --features lean-spec-tests; \
	fi
else
	@if [ "$(LEAN_SPEC_DEVNET)" = "devnet5" ]; then \
		cargo test --release --no-default-features --features lean-spec-tests,devnet5 --test tests -- \
			--exact $(addprefix test_all_,$(addsuffix _fixtures,$(SELECTED))) --nocapture; \
	else \
		cargo test --release --features lean-spec-tests --test tests -- \
		--exact $(addprefix test_all_,$(addsuffix _fixtures,$(SELECTED))) --nocapture; \
	fi
endif
	@echo "Tests complete."

$(FIXTURES):
	@:

clean:
	@echo "Cleaning up downloaded fixtures and keys..."
	@rm -rf $(EXTRACT_DIR)
	@echo "Clean up complete."
