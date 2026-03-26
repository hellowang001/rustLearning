# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Solana/Anchor learning project. The `learn/` directory contains a single Anchor workspace used for experimenting with Solana concepts (rent, accounts, PDAs, etc.).

## Common Commands

All commands run from within the `learn/` directory.

### Build & Test

```bash
# Start local validator (required before running tests with --skip-local-validator)
solana-test-validator

# Run all tests (against running local validator)
anchor test --skip-local-validator

# Build only
anchor build

# Run a single test by name
yarn run ts-mocha -p ./tsconfig.json -t 1000000 "tests/**/*.ts" --grep "测试名称"
```

### Lint (TypeScript)

```bash
yarn lint        # check
yarn lint:fix    # auto-fix
```

## Architecture

```
learn/                        # Anchor workspace root
├── Anchor.toml               # Cluster: localnet, wallet: ~/.config/solana/id.json
├── programs/learn/src/lib.rs # Solana program (Rust, Anchor framework)
└── tests/learn.ts            # TypeScript integration tests (@coral-xyz/anchor)
```

- **Program ID**: `8sm8zjwUwhvBLxhv1RHd64fQvdAasxoE3j57NrqSxJdr`
- **Rust toolchain**: `1.89.0` (pinned via `rust-toolchain.toml`)
- **Anchor version**: `^0.32.1`
- **Package manager**: `yarn`

## Workflow Pattern

Each learning topic is implemented as one or more Anchor instructions in `lib.rs` with corresponding tests in `learn.ts`. Old experiments are commented out (not deleted) in both files for reference.
