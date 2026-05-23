# School Management — Soroban Smart Contract

A Soroban smart contract on Stellar for managing student registrations, class assignments, and fee payments.

## Project Structure

```text
.
├── contracts
│   └── school-management
│       ├── src
│       │   ├── lib.rs
│       │   ├── school_management.rs   # contract logic
│       │   ├── storage.rs             # data types & storage keys
│       │   ├── error.rs               # contract errors
│       │   ├── events.rs              # contract events
│       │   └── test.rs                # unit tests
│       ├── Cargo.toml
│       └── Makefile
├── Cargo.toml
└── README.md
```

## Contract Functions

| Function | Auth | Description |
|---|---|---|
| `__constructor(admin, token)` | Admin | Initialize contract with admin address and payment token |
| `register_student(wallet, name, class)` | Student | Register a new student; returns assigned student ID |
| `get_student(student_id)` | None | Fetch student details by ID |
| `make_payment(student_id, amount)` | Student | Transfer tokens from student wallet to admin |
| `update_student_class(student_id, new_class)` | Admin | Move a student to a different class |
| `get_student_payment_history(student_id)` | None | Return all payments made by a student |
| `remove_student(student_id)` | Admin | Permanently remove a student and their payment records |

**Classes:** `Grade` · `HighSchool` · `College`

## Deployed Contract (Testnet)

| | |
|---|---|
| **Network** | Stellar Testnet |
| **Contract ID** | `CDROGZQOR4WJVCNKSMRJSLGYQP2PRU6JSGRHNHRIU56ZB2LGXMIANFIP` |
| **Admin** | `GB3B2MHRV5KZXSGJ6RG2IJMI4T4J4S5WIQAM2RIUQ57QZNP4B745EDM5` |
| **Payment Token** | `CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC` (native XLM) |
| **Explorer** | https://lab.stellar.org/r/testnet/contract/CDROGZQOR4WJVCNKSMRJSLGYQP2PRU6JSGRHNHRIU56ZB2LGXMIANFIP |

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) with the `wasm32v1-none` target
- [Stellar CLI](https://developers.stellar.org/docs/tools/developer-tools/cli/stellar-cli)

```bash
rustup target add wasm32v1-none
```

### Clone

```bash
git clone https://github.com/johneliud4/school-management.git
cd school-management
```

### Build

```bash
cd contracts/school-management
stellar contract build
```

### Test

```bash
cd contracts/school-management
cargo test
```

### Deploy to Testnet

Create a `.env` file in `contracts/school-management/`:

```env
TESTNET=testnet
SOURCE_TESTNET=<your-stellar-keypair-name>
ADMIN_TESTNET=<your-G-address>
TOKEN_TESTNET=<token-contract-address>
CONTRACT_ID=<deployed-contract-address>
```

Then run:

```bash
make deploy
```

### Interact with the Deployed Contract

Register a student:

```bash
stellar contract invoke \
  --id CDROGZQOR4WJVCNKSMRJSLGYQP2PRU6JSGRHNHRIU56ZB2LGXMIANFIP \
  --network testnet \
  --source <your-keypair> \
  -- \
  register_student \
  --student_wallet <G-address> \
  --name "Alice" \
  --class_name '"College"'
```

Get a student:

```bash
stellar contract invoke \
  --id CDROGZQOR4WJVCNKSMRJSLGYQP2PRU6JSGRHNHRIU56ZB2LGXMIANFIP \
  --network testnet \
  --source <your-keypair> \
  -- \
  get_student \
  --student_id 1
```
