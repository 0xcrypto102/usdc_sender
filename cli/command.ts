#!/usr/bin/env ts-node
import { program } from 'commander';
import {
  PublicKey,
} from '@solana/web3.js';
import {
  initProject,
  getGlobalInfo,
  setClusterConfig,
  getUserPoolInfo,
  depositToken,
  withdrawToken,
  withdrawTokenAdmin,
} from "./scripts";
import { USDC_DECIMAL } from '../lib/types';

program.version('0.0.1');

programCommand('status')
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  .action(async (directory, cmd) => {
    const {
      env,
    } = cmd.opts();
    console.log('Solana config: ', env);
    await setClusterConfig(env);
    console.log(await getGlobalInfo());
  });

programCommand('user_status')
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  .option('-a, --address <string>', 'nft user pubkey')
  .action(async (directory, cmd) => {
    const {
      env,
      address,
    } = cmd.opts();
    console.log('Solana config: ', env);
    await setClusterConfig(env);
    if (address === undefined) {
      console.log("Error User Address input");
      return;
    }
    console.log(await getUserPoolInfo(new PublicKey(address)));
  });

programCommand('deposit')
  .option('-a, --amount <number>', 'deposit token amount')
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  .action(async (directory, cmd) => {
    const {
      env,
      amount,
    } = cmd.opts();

    console.log('Solana config: ', env);
    await setClusterConfig(env);

    if (amount === undefined || isNaN(parseFloat(amount))) {
      console.log("Error Token Amount input");
      return;
    }

    await depositToken(parseFloat(amount) * USDC_DECIMAL);
  });

programCommand('withdraw')
  .option('-a, --amount <number>', 'withdraw token amount')
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  .action(async (directory, cmd) => {
    const {
      env,
      amount,
    } = cmd.opts();

    console.log('Solana config: ', env);
    await setClusterConfig(env);

    if (amount === undefined || isNaN(parseFloat(amount))) {
      console.log("Error Token Amount input");
      return;
    }

    await withdrawToken(parseFloat(amount) * USDC_DECIMAL);
  });

programCommand('withdraw_admin')
  .option('-a, --amount <number>', 'withdraw token amount')
  .option('-u, --address <string>', 'user address')
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  .action(async (directory, cmd) => {
    const {
      env,
      address,
      amount,
    } = cmd.opts();

    console.log('Solana config: ', env);
    await setClusterConfig(env);

    if (amount === undefined || isNaN(parseFloat(amount))) {
      console.log("Error Token Amount input");
      return;
    }

    if (address === undefined) {
      console.log("Error User Address input");
      return;
    }

    await withdrawTokenAdmin(parseFloat(amount) * USDC_DECIMAL, new PublicKey(address));
  });

programCommand('init')
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  .action(async (directory, cmd) => {
    const {
      env,
    } = cmd.opts();
    console.log('Solana config: ', env);
    await setClusterConfig(env);

    await initProject();
  });

function programCommand(name: string) {
  return program
    .command(name)
    .option(
      '-e, --env <string>',
      'Solana cluster env name',
      'devnet', //mainnet-beta, testnet, devnet
    )
}

program.parse(process.argv);