import * as anchor from "@coral-xyz/anchor";
import {Program, web3} from "@coral-xyz/anchor";
import {KeypairVsPda} from "../target/types/keypair_vs_pda";
import * as splToken from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";
import { assert } from 'chai';
import * as async_hooks from "node:async_hooks";

describe("keypair_vs_pda", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.KeypairVsPda as Program<KeypairVsPda>;
    // const defaultKeyPair = new anchor.web3.PublicKey("28QdAWmR5Tn5NsifTBvB8DxJDXN5eaQbFVwHactRxmff")
    /*
    it("已经初始化-----PDA版本",async ()=>{
        const sees = []
        const [myPda,_bump]=anchor.web3.PublicKey.findProgramAddressSync(sees,program.programId);
        console.log("存储账户地址是",myPda.toBase58());
        const tx = await program.methods.initializePda().accounts({myPda:myPda}).rpc();
    });

    it("已初始化----keypair 版本",async ()=>{
        const newKeypair = anchor.web3.Keypair.generate();

        await airdropSol(newKeypair.publicKey,1e9);// 空投 1SOL

        console.log("key pair 账户地址是:",newKeypair.publicKey.toBase58());


        await program.methods.initializeKeypairAccount()
            .accounts({myKeypairAccount: newKeypair.publicKey})
            .signers([newKeypair]) // 签名者必须是keypair
            .rpc();
    })

     */

    /*
    it("控制台记录账户所有者",async ()=>{
        console.log(`程序地址是 ${program.programId}`)
        const newKeypair = anchor.web3.Keypair.generate();
        // var receiverWallet = anchor.web3.Keypair.generate();
        // 初始化之前获取账户所有者
        await airdropSol(newKeypair.publicKey,10);
        const accountInfoBefore = await anchor.getProvider().connection.getAccountInfo(newKeypair.publicKey);
        console.log(`初始 keypair 账户所有者是 ${accountInfoBefore.owner}`);

        await program.methods.initializeKeypairAccount()
            .accounts({myKeypairAccount:newKeypair.publicKey})
            .signers([newKeypair])
            .rpc();
        // 在初始化后获取账户所有者
        const accountInfoAfter = await anchor.getProvider().connection.getAccountInfo(newKeypair.publicKey);
        console.log(`初始化 keypair 账户所有者是${accountInfoAfter.owner}`);

    })
    async function getBalance(account) {
        const balance = await anchor.getProvider().connection.getBalance(account);
        console.log(`${account} has  ${balance / anchor.web3.LAMPORTS_PER_SOL} SOL `);
    }
     */

    /*
    it("Is initialized!",async ()=>{
        console.log("program address", program.programId.toBase58());
        const seeds = []
        const [pda, bump_] = anchor.web3.PublicKey.findProgramAddressSync(seeds, program.programId);
        console.log("owner of pda before initialize:",
            await anchor.getProvider().connection.getAccountInfo(pda));
        await program.methods.initializePda().accounts({pda:pda}).rpc();

        console.log("owner of pda after initialize",
            await anchor.getProvider().connection.getAccountInfo(pda).owner.toBase58());

    })

     */

    /*
    it("Is initialized!", async () => {
        const programId = await program.account.pda.programId;

        let seeds = [];
        let pdaAccount = anchor.web3.PublicKey.findProgramAddressSync(seeds, programId)[0];

        const tx = await program.methods.initialize().accounts({
            pda: pdaAccount
        }).rpc();

        // 转移 2 SOL
        const tx2 = await program.methods.donate(new anchor.BN(2_000_000_000)).accounts({
            pda: pdaAccount
        }).rpc();

        console.log("lamport balance of pdaAccount",
            await anchor.getProvider().connection.getBalance(pdaAccount));


        // 转回 1 SOL
        // 前者是被允许的地址
        await program.methods.withdraw(new anchor.BN(1_000_000_000)).accounts({
            pda: pdaAccount
        }).rpc();
        console.log("lamport balance of pdaAccount",
            await anchor.getProvider().connection.getBalance(pdaAccount));

     */

    /*
    it ("delete account",async ()=>{
        const seed = []
        let [thePda,_bump] =anchor.web3.PublicKey.findProgramAddressSync(seed,program.programId);

        await program.methods.initialize().accounts({pda:thePda}).rpc();
        let account1 = await program.account.pda.fetch(thePda);
        console.log(account1);
        // 删除
        await program.methods.delete().accounts({pda:thePda}).rpc();

        let account = await program.account.pda.fetch(thePda);
        console.log(account);
    })

     */

    const provider = anchor.AnchorProvider.env();
    const signerKp = provider.wallet.payer;
    const toKp = new web3.Keypair();
    it("Creates a new mint and associated token account using CPI",async ()=>{

        // seeds
        const [mint]=PublicKey.findProgramAddressSync(
            [Buffer.from("my_mint"),signerKp.publicKey.toBuffer()],
            program.programId
        );

        const ata = splToken.getAssociatedTokenAddressSync(mint,signerKp.publicKey,false)

        const tx = await program.methods.createAndMintToken().accounts({
            signer:signerKp.publicKey,
            newMint:mint,
            newAta:ata,
            tokenProgram:splToken.TOKEN_PROGRAM_ID,
            associatedTokenProgram:splToken.ASSOCIATED_TOKEN_PROGRAM_ID,
            systemProgram:anchor.web3.SystemProgram.programId,
        }).rpc();

        console.log("Transaction signature:", tx);
        console.log("Token (Mint Account) Address:", mint.toString());
        console.log("Associated Token Account:", ata.toString());

        // 验证 Token 详细信息
        const mintInfo = await splToken.getMint(provider.connection,mint);
        assert.equal(mintInfo.mintAuthority?.toString(),signerKp.publicKey.toString(),"mint authority should be the signer");
        assert.equal(mintInfo.freezeAuthority?.toString(),signerKp.publicKey.toString(),"Freeze no signer");
        assert.equal(mintInfo.supply.toString(),"100000000000","供应量不对");

        // 验证ATA
        const tokenAccount = await splToken.getAccount(provider.connection,ata);

        assert.equal(tokenAccount.mint.toString(),mint.toString(),"ATA 不对");
        assert.equal(tokenAccount.owner.toString(),signerKp.publicKey.toString(),"token owner 不对");
        assert.equal(tokenAccount.amount.toString(), "100000000000", "Token balance should be 100 tokens (with 9 decimals)");
        // Token 余额应为 100 个 Token（精度为 9）
        assert.equal(tokenAccount.delegate, null, "Token account should not have a delegate");
        // Token 账户不应有委托


    })

// solana-test-validator
// anchor test --skip-local-validator

// 此函数向一个地址空投SOL
    async function airdropSol(publickey, amount) {
        let airdropTx = await anchor.getProvider().connection.requestAirdrop(publickey, amount);
        await confirmTransaction(airdropTx);
    }

    async function confirmTransaction(tx) {
        const latestBlockHash = await anchor.getProvider().connection.getLatestBlockhash();
        await anchor.getProvider().connection.confirmTransaction(
            {
                blockhash: latestBlockHash.blockhash,
                lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
                signature: tx,
            });
    }
})