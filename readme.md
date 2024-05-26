# USDC SENDER

## Main Instructions

- initialize

- deposit_usdc

- withdraw_usdc

- withdraw_admin

## Compile and Deploy

- Make sure you have solana installed
- Make sure you have rust installed
- Install phantom wallet on the browser or any from the list
- Make sure u have a solana wallet with SOL token (devnet)

- Config to devnet

```
solana config set --url devnet
```

- Create wallet

```
solana-keygen new -o <path to keypair file>
```

- Configure new wallet as main keypair

```
solana config set --keypair <path to new keypair>
```

- Add test token to the wallet

```
solana airdrop 2
```

- Build the project

```
anchor build
```

- Sync the program id

```
anchor keys sync
```

- Build again the project

```
anchor build
```

- After the build step completes, deploy the program to devnet

```
anhor deploy
```

## Test

- Install node modules

```
npm install or yarn
```

- initialize

```
npm run ts-node init
```

- withdraw_admin

```
npm run ts-node withdraw_admin -a 0.01 -u <user_address>
```

- deposit_usdc

```
npm run ts-node deposit -a 1000
```

You can see changes in token balance

```
spl token accounts
```

- withdraw_usdc

```
npm run ts-node withdraw -a 500
```
