// tests/my_solana_dapp.ts

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MySolanaDapp } from "../target/types/my_solana_dapp";
import { assert } from "chai";

describe("my_solana_dapp", () => {
  // 配置客户端以使用本地集群。
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // 从工作区获取已部署的程序实例
  const program = anchor.workspace.MySolanaDapp as Program<MySolanaDapp>;

  // 为我们的计数器账户创建一个新的密钥对。
  // 我们需要这个密钥对的公钥来找到链上的账户。
  const counterAccount = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    // 调用程序的 `initialize` 指令
    const tx = await program.methods
      .initialize()
      .accounts({
        // 传入指令需要的账户
        counter: counterAccount.publicKey,
        user: provider.wallet.publicKey,
      })
      .signers([counterAccount]) // 因为我们在创建(init)一个新账户，所以需要它的签名
      .rpc();

    console.log("Your transaction signature", tx);

    // 从链上获取账户的最新状态
    const accountData = await program.account.counter.fetch(counterAccount.publicKey);

    // 断言（Assert）来验证 count 值是否为 0
    assert.ok(accountData.count.toNumber() === 0, "Counter should be initialized to 0");
  });

  it("Increments the counter!", async () => {
    // 调用 `increment` 指令
    await program.methods
      .increment()
      .accounts({
        counter: counterAccount.publicKey,
      })
      .rpc();

    const accountData = await program.account.counter.fetch(counterAccount.publicKey);

    // 验证 count 值是否为 1
    assert.ok(accountData.count.toNumber() === 1, "Counter should be 1 after incrementing");
  });

  it("Decrements the counter!", async () => {
    // 调用 `decrement` 指令
    await program.methods
      .decrement()
      .accounts({
        counter: counterAccount.publicKey,
      })
      .rpc();

    const accountData = await program.account.counter.fetch(counterAccount.publicKey);

    // 验证 count 值是否回到 0
    assert.ok(accountData.count.toNumber() === 0, "Counter should be 0 after decrementing");
  });
});
