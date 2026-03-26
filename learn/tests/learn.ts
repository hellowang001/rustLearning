import * as anchor from "@coral-xyz/anchor";
import {Program} from "@coral-xyz/anchor";
import {Learn} from "../target/types/learn";

describe("learn", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.learn as Program<Learn>;

    const defaultKeyPair = new anchor.web3.PublicKey("28QdAWmR5Tn5NsifTBvB8DxJDXN5eaQbFVwHactRxmff")
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

    /*

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

     */

    /*
    it('查看空账户花费金额',async () => {
      await program.methods.initialize().rpc();
    });

     */

    /*
    it('查看账户余额',async () => {
        const tx=await program.methods.readBalance().accounts({acct:defaultKeyPair}).rpc();
    });

     */


    async function getBalance(account) {
        const balance = await anchor.getProvider().connection.getBalance(account);
        console.log(`${account} has  ${balance / anchor.web3.LAMPORTS_PER_SOL} SOL `);
    }

    /*
    it("Transmit SOL", async () => {
        // generate a new wallet
        const recipient = anchor.web3.Keypair.generate();
        await getBalance(recipient.publicKey);

        // send to account 1 SOL via the program
        let amount = new anchor.BN(1 * anchor.web3.LAMPORTS_PER_SOL);
        await program.methods.sendSol(amount).accounts({recipient:recipient.publicKey}).rpc();

        await getBalance(recipient.publicKey)

    })

     */

    it("Test send Sol to more account",async ()=>{
        const recipient1 = anchor.web3.Keypair.generate();
        const recipient2 = anchor.web3.Keypair.generate();
        const recipient3 = anchor.web3.Keypair.generate();
        const recipient4 = anchor.web3.Keypair.generate();
        await getBalance(recipient1.publicKey);
        await getBalance(recipient2.publicKey);
        await getBalance(recipient3.publicKey);
        await getBalance(recipient4.publicKey);

        const accountMeta1 = {pubkey:recipient1.publicKey,isWritable:true,isSigner: false};
        const accountMeta2 = {pubkey:recipient2.publicKey,isWritable:true,isSigner: false};
        const accountMeta3 = {pubkey:recipient3.publicKey,isWritable:true,isSigner: false};
        const accountMeta4 = {pubkey:recipient4.publicKey,isWritable:true,isSigner: false};

        let amount = new anchor.BN(4*anchor.web3.LAMPORTS_PER_SOL);
        await program.methods.sendMore(amount).remainingAccounts([accountMeta1,accountMeta2,accountMeta3,accountMeta4]).rpc();
        await getBalance(recipient1.publicKey);
        await getBalance(recipient2.publicKey);
        await getBalance(recipient3.publicKey);
        await getBalance(recipient4.publicKey);

    })
});
// solana-test-validator
// anchor test --skip-local-validator