# NaijaRemitLock - Smart Contract

Purpose-locked remittances for Nigeria built on Stellar Soroban.

## What it does

When Nigerians abroad send money home, sometimes it gets spent on the wrong things. This smart contract locks the money for a specific purpose (school fees, rent, business, hospital) and only releases it when conditions are met.

## Features

- Create purpose-locked lockboxes
- Time-based release (specific date)
- Emergency cancel with delay
- Supports XLM and USDC

## Contract Functions

| Function | Description |
|----------|-------------|
| `create_lockbox` | Create a new lockbox with purpose and release time |
| `release` | Release funds to beneficiary |
| `emergency_cancel` | Cancel with 7-day delay |

## Deployment

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/naija_remit_lock.wasm \
  --source YOUR_SECRET_KEY \
  --network testnet
