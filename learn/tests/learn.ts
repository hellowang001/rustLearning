import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Learn } from "../target/types/learn";

describe("learn", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.learn as Program<Learn>;
  /*
  it("初始化映射 !", async () => {
    // Add your test here.
    const  key = new anchor.BN(42);
    const seeds =[key.toArrayLike(Buffer,"le",8)]
    let valueAccount = anchor.web3.PublicKey.findProgramAddressSync(
        seeds,
        program.programId
    )
    const tx = await program.methods.initialize(key).accounts({val:valueAccount}).rpc();
    console.log("Your transaction signature", tx);
  });

   */

  it("初始化映射并且设置 !", async () => {
    // Add your test here.
    const  key = new anchor.BN(42);
    const value =new anchor.BN(1337);
    const seeds =[key.toArrayLike(Buffer,"le",8)]
    let valueAccount = anchor.web3.PublicKey.findProgramAddressSync(
        seeds,
        program.programId
    )[0];
    // 初始化initialize 账户，把 key 42 放在了valueAccount的 seeds 中
    await program.methods.initialize(key).accounts({val:valueAccount}).rpc();
    // 设置账户
    // 修改了这个账户的 seeds
    await program.methods.set(value).accounts({val:valueAccount}).rpc();

    // 读取账户
    let result = await program.account.val.fetch(valueAccount);

    console.log(`值 ${result.value} 被存储在 ${valueAccount.toBase58()}`);
  });
});
// solana-test-validator
// anchor test --skip-local-validator