-include .env

all: test

test: build
	cargo test

debug: build
	RUST_BACKTRACE=full cargo test
	
t: test
	RUST_BACKTRACE=1 cargo test  -- --nocapture

build:
	stellar contract build
	@ls -l ../../target/wasm32v1-none/release/*.wasm

fmt:
	cargo fmt --all

clean:
	cargo clean 

# Required .env variables:
#   TESTNET         = testnet
#   SOURCE_TESTNET  = <stellar keypair name or secret key>
#   ADMIN_TESTNET   = <admin G... address>
#   TOKEN_TESTNET   = <token G... address>
#   CONTRACT_ID     = <deployed contract C... address>
#
# Deployed testnet contract: CDROGZQOR4WJVCNKSMRJSLGYQP2PRU6JSGRHNHRIU56ZB2LGXMIANFIP
# Admin:                     GB3B2MHRV5KZXSGJ6RG2IJMI4T4J4S5WIQAM2RIUQ57QZNP4B745EDM5
# Token (native XLM):        CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC

upload: build
	@echo "Uploading contract..."
	@stellar contract install \
	  --network ${TESTNET} \
	  --source ${SOURCE_TESTNET} \
	  --wasm ../../target/wasm32v1-none/release/school_management.wasm > upload.log 2>&1 || (cat upload.log && exit 1)
	@grep -oE "[a-f0-9]{64}" upload.log | tail -n 1 > wasm_hash.txt
	@echo "Wasm hash saved to wasm_hash.txt:"
	@cat wasm_hash.txt

deploy: upload
	@echo "Deploying contract..."
	stellar contract deploy \
	  --wasm-hash $$(cat wasm_hash.txt) \
	  --network ${TESTNET} \
	  --source ${SOURCE_TESTNET} \
	  -- \
	  --admin ${ADMIN_TESTNET} \
	  --token ${TOKEN_TESTNET}

interact:
	stellar contract invoke \
		--id ${CONTRACT_ID} \
		--source-account ${SOURCE_TESTNET} \
		--network ${TESTNET} \
		-- \
		get_student \
		--student_id 1