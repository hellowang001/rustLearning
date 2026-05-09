import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { KeypairVsPda } from "../target/types/keypair_vs_pda";
import * as splToken from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";
import { Transaction, sendAndConfirmTransaction } from "@solana/web3.js";
import { assert } from 'chai';
import * as async_hooks from "node:async_hooks";
import {
    createCreateMetadataAccountV3Instruction,
    PROGRAM_ID as TOKEN_METADATA_PROGRAM_ID,
} from "@metaplex-foundation/mpl-token-metadata";

import {
    createAssociatedTokenAccount,
    getAccount,
    getMint,
    TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import BN from 'bn.js';
import {
    Raydium,
    DEVNET_PROGRAM_ID,
    TxVersion,
    CurveCalculator,
    publicKey,
} from '@raydium-io/raydium-sdk-v2';
import bs58 from "bs58";
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
    // const signerKp = provider.wallet.payer;
    const superMe = provider.wallet;

    const connection = provider.connection;

    // 为 admin config 账户生成密钥对（将作为签名者传递以授权 adminConfig 账户创建）
    const bankPiv = "5YueKNVCvQqdmid9NgE6owdLUiB56oLHptiEhiVvomWQe9TEJnuNqWgZZ1Nxke1fnz2886FsTKnHjrqY2ZnwdZDe"
    const secretKeyBank = new Uint8Array(bs58.decode(bankPiv))
    let bankKeypir = web3.Keypair.fromSecretKey(secretKeyBank)
    /*
    it("Creates a new mint and associated token account using CPI",async ()=>{

        // seeds
        const [mint]=PublicKey.findProgramAddressSync(
            [Buffer.from("my_mint"),signerKp.publicKey.toBuffer()],
            program.programId
        );
        // 获取ATA 账户方法，只需要传入seeds 和地址，这个方法会自动加上 ASSOCIATED_TOKEN_PROGRAM_ID 去派生出来
        const ata = splToken.getAssociatedTokenAddressSync(mint,signerKp.publicKey,false)
        // 只需要实例 createAndMintToken的account 就可以了
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
    */

    /*
    it("Transfers token using CPI",async ()=>{
        // 派生Mint 的PDA
        const [mint] = PublicKey.findProgramAddressSync(
            [Buffer.from("my_mint"),signerKp.publicKey.toBuffer()],
            program.programId
        );
        // 获取ATA
        const fromAta = splToken.getAssociatedTokenAddressSync(mint,signerKp.publicKey,false);
        const toAta = splToken.getAssociatedTokenAddressSync(mint,toKp.publicKey);

        // 创建
        // 创建 to_ata，因为它尚未存在
        try {
        await splToken.createAssociatedTokenAccount(
            provider.connection,
            signerKp,
            mint,
            toKp.publicKey
        );
        } catch (error) {
        throw new Error(error)
        }

        const transferAmount = new anchor.BN(10_000_000_000); // 10颗
        const tx = await program.methods.transferTokens(transferAmount).accounts({
            from:signerKp.publicKey,
            fromAta:fromAta,
            toAta:toAta,
            tokenProgram:splToken.TOKEN_PROGRAM_ID,
        }).rpc();
        console.log("Transfer Transaction signature:", tx);
        // Verify the transfer
        // 验证转移
        const toBalance = await provider.connection.getTokenAccountBalance(toAta);
        assert.equal(
            toBalance.value.amount,
            transferAmount.toString(),
            "Recipient balance should match transfer amount"
        );
    })
    */

    /*
    it("Get Token Balances", async () => {
        const [mint] = PublicKey.findProgramAddressSync(
            [Buffer.from("my_mint"), signerKp.publicKey.toBuffer()],
            program.programId
        );
        const signerKpAta = splToken.getAssociatedTokenAddressSync(mint, signerKp.publicKey);

        const tx = await program.methods.getBalance().accounts({ tokenAccount: signerKpAta }).rpc();
        console.log("tx:", tx);
        // msg!() 日志会直接打印在终端，也可以通过 getTransaction 获取
        const txInfo = await provider.connection.getTransaction(tx, { commitment: "confirmed" });
        console.log("Program logs:", txInfo?.meta?.logMessages);
    })
    */

    /*
    it("TypeScript SPL Token Tests", async () => {

        const mintDecimals = 6;
        const mintAuthority = provider.wallet.publicKey;
        const freezeAuthority = provider.wallet.publicKey;
        // const toKp = new web3.Keypair();

        //  先创建Mint 


        // 返回的是一个publickey ,返回的就是这个代币的Mint,以后关于这个代币的操作都要用到这个MintPublicKey
        const mintPublicKey = await splToken.createMint(
            provider.connection,
            signerKp,
            mintAuthority,
            freezeAuthority,
            mintDecimals

        );
        console.log("Created Mint:", mintPublicKey.toString());
        // 创建或获取 signerKp 的此代币的ATA（已存在不报错）
        const signerKpAtaAccount = await splToken.getOrCreateAssociatedTokenAccount(
            provider.connection,
            signerKp,
            mintPublicKey,
            signerKp.publicKey
        );
        const signerKpAtaAccountAddress = signerKpAtaAccount.address;
        const mintAmount = BigInt(1000 * (10 ** mintDecimals));
        // 给代币填充名称信息
        // 1 先推导 MetadataAccount 的PDA地址
        const [metadataPDA] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("metadata"),
                TOKEN_METADATA_PROGRAM_ID.toBuffer(),
                mintPublicKey.toBuffer(),       // 你的 Mint 地址
            ],
            TOKEN_METADATA_PROGRAM_ID
        );
        //构造创建Metadata 的指令
        const createMetadataIx = createCreateMetadataAccountV3Instruction(
            {
                metadata: metadataPDA,
                mint: mintPublicKey,
                mintAuthority: signerKp.publicKey,
                payer: signerKp.publicKey,
                updateAuthority: signerKp.publicKey,
            },
            {
                createMetadataAccountArgsV3: {
                    data: {
                        name: "My Token",   // 代币名称
                        symbol: "MTK",        // 代币符号
                        uri: "https://ipfs.io/ipfs/bafkreidwhm2o7berdtgy4623w7gl7ylv2t4v7p62rusqkj6gdj32hyfpei",           // 指向 JSON 元数据的 URL（可为空）
                        sellerFeeBasisPoints: 0,
                        creators: null,
                        collection: null,
                        uses: null,
                    },
                    isMutable: true,   // true = 之后还能修改 metadata
                    collectionDetails: null,
                },
            }
        );
        // 把修改元信息的交易发出去
        const tx = new Transaction().add(createMetadataIx);
        await sendAndConfirmTransaction(provider.connection, tx, [signerKp]);

        console.log("Metadata Account:", metadataPDA.toString());

        // 然后调用MintTo 铸造


        await splToken.mintTo(
            provider.connection,
            signerKp,
            mintPublicKey,
            signerKpAtaAccountAddress,
            signerKp,     // ← 传 Signer（Keypair），而不是 PublicKey
            mintAmount
        )
        const mintInfo = await splToken.getMint(provider.connection, mintPublicKey);
        assert.equal(mintInfo.decimals, mintDecimals, "Mint 的小数位数应该匹配");
        assert.equal(mintInfo.mintAuthority?.toString(), mintAuthority.toString(), "Mint 的授权者应该匹配");
        assert.equal(mintInfo.freezeAuthority?.toString(), freezeAuthority.toString(), "冻结授权者应该匹配");
        // 验证余额
        const accounBalances = await provider.connection.getTokenAccountBalance(signerKpAtaAccountAddress);
        console.log("mint后的源余额:", accounBalances.value.amount);

        assert.equal(accounBalances.value.amount, mintAmount.toString(), "余额应该和 mint 的数量匹配");
        // 转账
        
        // 读取转移前的余额

        // 创建接收者ATA，由本地钱包去创建
        const destinationAta = await splToken.getOrCreateAssociatedTokenAccount(
            provider.connection,
            signerKp,
            mintPublicKey,
            toKp.publicKey
        );
        const sourceBalanceBefore = await provider.connection.getTokenAccountBalance(signerKpAtaAccount.address);
        const destinationBalanceBefore = await provider.connection.getTokenAccountBalance(destinationAta.address);
        console.log("转移前的源余额:", sourceBalanceBefore.value.amount);
        console.log("转移前的目标余额:", destinationBalanceBefore.value.amount);
        const transferAmount = BigInt(500 * (10 ** mintDecimals));
        // 转80个
        await splToken.transfer(
            provider.connection,
            signerKp,// 此笔交易付手续费的人
            signerKpAtaAccountAddress,   // ← source 是 ATA 地址，不是钱包地址
            destinationAta.address, // 目标 Token Account（ATA 地址）
            signerKp, // source ATA 的 owner（Signer）
            transferAmount
        )
        // 读取转移后的余额
        const sourceBalanceAfter = await provider.connection.getTokenAccountBalance(signerKpAtaAccount.address);
        const destinationBalanceAfter = await provider.connection.getTokenAccountBalance(destinationAta.address);
        // 验证转账后的金额
        console.log("转移后的signerkp余额:", sourceBalanceAfter.value.amount);
        console.log("转移后的toKp余额:", destinationBalanceAfter.value.amount);
        assert.equal(sourceBalanceAfter.value.amount, (mintAmount - transferAmount).toString(), "源应该还剩 500 个 token");
        assert.equal(destinationBalanceAfter.value.amount, transferAmount.toString(), "目标应该收到 500 个 token");
    })
    */


    /*
    it("test init token sell", async () => {

        // pda mint 账户，通过程序id 和特定的mint 种子生产mint 地址
        [mint] = web3.PublicKey.findProgramAddressSync(
            [Buffer.from("token_mint")],
            program.programId
        );

        // 国库pda地址
        [treasuryPda] = web3.PublicKey.findProgramAddressSync(
            [Buffer.from("treasury")],
            program.programId
        );

        // 初始化方法调用
        const tx = await program.methods.initialize().accounts({
            admin: adminKp.publicKey,
            adminConfig: adminConfigKp.publicKey,
            mint: mint,
            treasury: treasuryPda,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: anchor.web3.SystemProgram.programId,
        }).signers([adminKp, adminConfigKp]).rpc();

        console.log("initialize tx:", tx);

        const mintInfo = await getMint(connection, mint);
        assert.equal(mintInfo.mintAuthority.toBase58(), mint.toBase58());
        assert.equal(Number(mintInfo.supply), 0);
        assert.equal(mintInfo.decimals, 9);


    })
    */

    /*
    it("buy tokens", async () => {
        const solToSend = new anchor.BN(1e9); // 1 SOL
        const expectedTokenAmount = Number(solToSend) * TOKENS_PER_SOL; // 1*100 个 token

        const initialTreasuryBalance = await connection.getBalance(treasuryPda);

        // 为买家创建他的ata
        buyerAta = await createAssociatedTokenAccount(
            provider.connection,
            buyer,
            mint,
            buyer.publicKey,
            undefined,
            TOKEN_PROGRAM_ID
        );

        const buyerAtaInfo = await getAccount(
            connection,
            buyerAta,
            undefined,
            TOKEN_PROGRAM_ID
        )
        const initialBuyerAtaBalance = Number(buyerAtaInfo.amount);

        // 调用我们程序的Mint 函数已购买token
        const tx = await program.methods.mint(solToSend).accounts({
            buyer: buyer.publicKey,
            mint: mint,
            buyerAta: buyerAta,
            treasury: treasuryPda,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: anchor.web3.SystemProgram.programId,
        }).rpc();
        console.log("mint tx:", tx);
        console.log("Sent", solToSend, "SOL, expecting", expectedTokenAmount, "tokens");

        const newTreasuryBalance = await connection.getBalance(treasuryPda);
        assert.equal(
            newTreasuryBalance - initialTreasuryBalance,
            Number(solToSend),
            "SOL 未正确转移到 treasury"
        );

        const updatedBuyerAtaInfo = await getAccount(connection, buyerAta, undefined, TOKEN_PROGRAM_ID);
        const newBuyerAtaBalance = Number(updatedBuyerAtaInfo.amount);

        assert.equal(
            newBuyerAtaBalance - initialBuyerAtaBalance,
            expectedTokenAmount,
            "Token 未正确 mint"
        );

    })
    */

    /*
    it("withdraw founds from treasury", async () => {
        // 国库pda地址
        [treasuryPda] = web3.PublicKey.findProgramAddressSync(
            [Buffer.from("treasury")],
            program.programId
        );

        const initialAdminBalance = await connection.getBalance(adminKp.publicKey);
        const initialTreasuryBalance = await connection.getBalance(treasuryPda);

        console.log("Initial treasury balance:", initialTreasuryBalance, "SOL");
        console.log("Initial admin balance:", initialAdminBalance, "SOL");
        assert.isAbove(
            initialTreasuryBalance,
            0,
            "Treasury should have funds from previous tests"
        );

        const amountToWithdraw = new anchor.BN(Math.floor(initialTreasuryBalance / 2)); // 提取一半的 treasury 余额
        // const amountToWithdraw = new anchor.BN(Math.floor(500000000)); // 提取一半的 treasury 余额

        try {
            const tx = await program.methods
                .withdrawFunds(amountToWithdraw)
                .accounts({
                    admin: adminKp.publicKey,
                    adminConfig: new PublicKey("Bve7yrmgma4jzzvvnth8Xzs3rWax7ftzBTAMhjATiDfF"),
                    treasury: treasuryPda,
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .rpc();
            console.log("withdrawFunds tx:", tx);

            const newAdminBalance = await connection.getBalance(adminKp.publicKey);
            const newTreasuryBalance = await connection.getBalance(treasuryPda);

            console.log("New treasury balance:", newTreasuryBalance, "SOL");
            console.log("New admin balance:", newAdminBalance, "SOL");

            // 断言 treasury 余额减少了我们提取的金额，即初始 treasury 余额的一半
            assert.approximately(
                initialTreasuryBalance - newTreasuryBalance,
                Number(amountToWithdraw),
                10000,
                "Treasury 余额没有减少到大致正确的金额"
            );

            // 断言管理员余额在提款后增加
            assert.isTrue(
                newAdminBalance > initialAdminBalance,
                "提款后，管理员余额没有增加"
            );
        } catch (error) {
            console.error("Error in withdraw test:", error);
            throw error;
        }
    })
    */
    // 查找用户的银行PDA
    const [userAccountPDA] = web3.PublicKey.findProgramAddressSync([Buffer.from("bank_account"),
    superMe.publicKey.toBuffer()], program.programId)
    /*
    it("Test Bank", async () => {
        // 初始化银行


        const initTx = program.methods.initialize().accounts({
            bank: bankKeypir.publicKey,
            payer: superMe.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
        }).signers([bankKeypir]).rpc()
        console.log("初始化交易签名", initTx);
        // 获取银行数据
        const bankData = await program.account.bank.fetch(bankKeypir.publicKey);

        // 查看bank 的Data 账户，是否存储 totalDeposits 数据为0 正确初始化
        assert.equal(bankData.totalDeposits.toString(), "0");
        console.log("初始化测试通过");
    });
    */

    /*
    it("Test Create Account", async () => {
        // 为签名者创建用户账户
        const createTx = await program.methods.createUserAccount().accounts({
            bank: bankKeypir.publicKey,
            userAccount: userAccountPDA,
            user: superMe.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
        }).rpc()
        console.log("创建用户交易签名", createTx);
        // 获取用户账户数据 PDA账户
        const userAccountData = await program.account.userAccount.fetch(userAccountPDA);
        // 验证用户账户是否已正确设置
        assert.equal(userAccountData.owner.toString(), superMe.publicKey.toString());
        assert.equal(userAccountData.balance.toString(), "0");
    })
    */
    /*
    it("Test Deposit", async () => {
        //查询余额
        const initUserBalance = await provider.connection.getBalance(superMe.publicKey);
        const initBankBalance = await provider.connection.getBalance(bankKeypir.publicKey);
        console.log(`初始用户 SOL 余额：${initUserBalance / 1e9} SOL`);
        console.log(`初始银行 SOL 余额：${initBankBalance / 1e9} SOL`);

        const bankAccountDataBefore = await program.account.bank.fetch(bankKeypir.publicKey);
        


        // 调用充值方法
        const depositAmount = new anchor.BN(5_000_000); // 5 SOL 以 lamports 为单位
        const balanceBefore = await program.methods.getBalance().accounts({
            bank: bankKeypir.publicKey,
            userAccount: userAccountPDA,
            user: superMe.publicKey
        }).view()
        const depositTx = await program.methods.deposit(depositAmount).accounts({
            bank: bankKeypir.publicKey,
            userAccount: userAccountPDA,
            user: superMe.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
        }).rpc();
        console.log("存款交易签名", depositTx);
        // 通过用户的PDA地址去获取用户PDA说存储的data里面的的银行账户余额
        const userAccountData = await program.account.userAccount.fetch(userAccountPDA);
        // assert.equal(userAccountData.balance.toString(), depositAmount.toString(), "useraccount Balances ERR");

        // 验证银行账户的PDA里面存储的data里的用户总存款是否匹配
        const bankAccountDataAfter = await program.account.bank.fetch(bankKeypir.publicKey);
        console.log("   ✓ 断言 1: 充值银行后余额 === 充值前银行余额 + 充值金额");
        const expectedBankBalance = bankAccountDataBefore.totalDeposits.add(depositAmount);
        assert.equal(
            bankAccountDataAfter.totalDeposits.toString(),
            expectedBankBalance.toString(),
            "充值后银行余额应该等于充值前余额加上充值金额"
        );
        // 上面是记账，下面是最终的SOL余额
        const finaUserSolBalances = await provider.connection.getBalance(superMe.publicKey);
        const finaBankSolBalances = await provider.connection.getBalance(bankKeypir.publicKey);
        console.log(`最终用户 SOL 余额：${finaUserSolBalances / 1e9} SOL`);
        console.log(`最终银行 SOL 余额：${finaBankSolBalances / 1e9} SOL`);

        // 检查实际SOl 大于 初始化金额
        assert.isTrue(finaBankSolBalances > initBankBalance);

        // 用户余额应减少存款金额 + 一些交易费用
        assert.isTrue(finaUserSolBalances < initUserBalance - Number(depositAmount));
        assert.isTrue(finaUserSolBalances > initUserBalance - Number(depositAmount) - 10000);
        // 获取账户余额
        const balanceAfter = await program.methods.getBalance().accounts({
            bank: bankKeypir.publicKey,
            userAccount: userAccountPDA,
            user: superMe.publicKey
        }).view()
            // 断言 2: 余额增加量应该等于充值金额
        console.log("   ✓ 断言 2: 余额增加量 === 充值金额");
        const balanceIncrease = balanceAfter.sub(balanceBefore);
        assert.equal(
            balanceIncrease.toString(),
            depositAmount.toString(),
            "余额增加量应该等于充值金额"
        );
        // 验证余额是否比充值前多了充值金额
        console.log("   ✓ 断言 3: 充值后余额 === 充值前余额 + 充值金额");
        const expectedBalance = balanceBefore.add(depositAmount);
        assert.equal(
            balanceAfter.toString(),
            expectedBalance.toString(),
            "充值后余额应该等于充值前余额加上充值金额"
        );
        console.log(`用户余额：${Number(balanceBefore) / 1e9} SOL`);
    })
        */
    /*
    it("get balance",async()=>{
                // 获取账户余额
        const balance = await program.methods.getBalance().accounts({
            bank: bankKeypir.publicKey,
            userAccount: userAccountPDA,
            user: superMe.publicKey
        }).view()


        console.log(`用户余额：${Number(balance) / 1e9} SOL`);
        console.log("balance 的类型是%s",typeof balance);  // 输出: "object"

        const depositAmount = new anchor.BN(5_000_000); // 5 SOL 以 lamports 为单位
        console.log("depositAmount 的类型是%s",typeof depositAmount);  // 输出: "object"

        // ✅ 加法
        const sum = balance.add(depositAmount);
        console.log(`${balance} + ${depositAmount} = ${sum}`);  // 5000000 + 3000000 = 8000000

    })
        */

    /*
    it("get interest",async()=>{
                // 获取账户余额
        const balance = await program.methods.calculateInterest().accounts({
            bank: bankKeypir.publicKey,
            userAccount: userAccountPDA,
            user: superMe.publicKey
        }).view()


        console.log(`用户余额：${Number(balance) / 1e9} SOL`);
    })
        */


    /*
    it("Test Withdraw",async () =>{
            // 获取初始余额
        const userAccountData = await program.account.userAccount.fetch(userAccountPDA);
        const initialBalance = userAccountData.balance;

        // 获取初始 SOL 余额
        const initialUserBalance = await provider.connection.getBalance(superMe.publicKey);
        const initialBankBalance = await provider.connection.getBalance(bankKeypir.publicKey);
        const withdrawAmount = new anchor.BN(2_000_000); // 
        const bankDataBefor= await program.account.bank.fetch(bankKeypir.publicKey);

        // 调用提取方法
        const withdrawTx = await program.methods.withdraw(withdrawAmount).accounts({
            bank: bankKeypir.publicKey,
            userAccount: userAccountPDA,
            user: superMe.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
        }).rpc()
        console.log("提款交易签名", withdrawTx);
        // 获取新余额
        const updatedUserAccountData = await program.account.userAccount.fetch(userAccountPDA);
        const newBalance = updatedUserAccountData.balance;

        // 验证余额是否正确
        const expectedBalance = initialBalance.sub(withdrawAmount);
        assert.equal(newBalance.toString(), expectedBalance.toString(),"newBalance ERR2");

        // 验证银行总存款
        const bankDataAfter = await program.account.bank.fetch(bankKeypir.publicKey);
        assert.equal(bankDataAfter.totalDeposits.toString(),bankDataBefor.totalDeposits.sub(withdrawAmount).toString(),"bankData err");

        // 获取最终 SOL 余额
        const finalUserBalance = await provider.connection.getBalance(superMe.publicKey);
        const finalBankBalance = await provider.connection.getBalance(bankKeypir.publicKey);

        console.log(`最终用户 SOL 余额：${finalUserBalance / 1e9} SOL`);
        console.log(`最终银行 SOL 余额：${finalBankBalance / 1e9} SOL`);

        // 检查实际 SOL 转移
        // 用户余额应增加提款金额 (减去 tx 费用)
        // 由于用户支付 tx 费用，因此最终余额可能略低于预期
        assert.isTrue(finalUserBalance < initialUserBalance + Number(withdrawAmount),"ERR 2");
        // assert.isTrue(finalUserBalance > initialUserBalance - 10000); // 允许合理的 tx 费用

        // 银行余额应减少提款金额
        assert.isTrue(finalBankBalance <= initialBankBalance,"ERR 3");
    })
        */


    // solana-test-validator
    // anchor test --skip-local-validator
    // 单独运行某个测试：
    // anchor test --skip-local-validator -- --grep "TypeScript SPL Token Tests"
    // ANCHOR_PROVIDER_URL=https://api.devnet.solana.com ANCHOR_WALLET=~/.config/solana/id.json yarn run ts-mocha -p ./tsconfig.json -t 1000000 "tests/pda.ts"
    // ANCHOR_PROVIDER_URL=https://solana-devnet.g.alchemy.com/v2/wqZxT7UnY6AgrzV42CtGgGQ7ZGM-UrTq ANCHOR_WALLET=~/.config/solana/id.json yarn run ts-mocha -p ./tsconfig.json -t 1000000 "tests/pda.ts"
    // ANCHOR_PROVIDER_URL=https://devnet.helius-rpc.com/?api-key=9e16b6ee-45f4-4544-80f3-92370659cb99 ANCHOR_WALLET=~/.config/solana/id.json yarn run ts-mocha -p ./tsconfig.json -t 1000000 "tests/pda.ts"

    /*
    // 此函数向一个地址空投SOL
    async function airdropSol(publickey, amount) {
        let airdropTx = await anchor.getProvider().connection.requestAirdrop(publickey, amount);
        await confirmTransaction(airdropTx);
    }
    */

    /*
    async function confirmTransaction(tx) {
        const latestBlockHash = await anchor.getProvider().connection.getLatestBlockhash();
        await anchor.getProvider().connection.confirmTransaction(
            {
                blockhash: latestBlockHash.blockhash,
                lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
                signature: tx,
            });
    }
    */

    it("swap via our own program (CPI → Raydium CPMM) 2 On My contract ", async () => {
        const poolId = "8hc6S94q1L784DY6h99wSUdc8DuPniy4UZWKXd7tdBD4";
        console.log("cpmmSwapProgram=",DEVNET_PROGRAM_ID.CREATE_CPMM_POOL_PROGRAM.toString());

        const cpmmSwapProgram = new PublicKey(DEVNET_PROGRAM_ID.CREATE_CPMM_POOL_PROGRAM.toString()); // cpmm 程序地址，要用的
        // 1. 初始化 Raydium SDK，获取池子所需的所有账户地址
        const raydium = await Raydium.load({
            owner: superMe.payer,
            connection,
            cluster: "devnet",
            disableFeatureCheck: true,
            disableLoadToken: true,
            blockhashCommitment: "finalized",
        });
        // 使用raydium sdk 获得 池子的信息
        const { poolInfo, poolKeys } = await raydium.cpmm.getPoolInfoFromRpc(poolId);
        // 2 确定input/output: 我们是用mintA 换 mintB 
        const inputMint = new PublicKey(poolInfo.mintA.address)
        const outputMint = new PublicKey(poolInfo.mintB.address)
        // 拿到了代币的地址后，去获取用户的代币 ATA地址 等下account 要用
        const inputTokenAccount = splToken.getAssociatedTokenAddressSync(inputMint, superMe.publicKey);
        const outputTokenAccount = splToken.getAssociatedTokenAddressSync(outputMint, superMe.publicKey);

        // 3. 投入 10 个 mintA（注意精度）
        const amountIn = new BN(10 * 10 ** poolInfo.mintA.decimals);
        const minAmountOut = new BN(0); // 测试环境不做滑点保护
        console.log("开始换币")
        const swapTxId =await program.methods.proxySwapBaseInput(amountIn, minAmountOut).accounts({
                cpSwapProgram:cpmmSwapProgram,
                payer: superMe.publicKey,
                authority: new PublicKey(poolKeys.authority),
                ammConfig: new PublicKey(poolKeys.config.id),
                poolState:  new PublicKey(poolId),
                inputTokenAccount:inputTokenAccount,
                outputTokenAccount:outputTokenAccount,
                inputVault:new PublicKey(poolKeys.vault.A),
                outputVault:new PublicKey(poolKeys.vault.B),
                inputTokenProgram:TOKEN_PROGRAM_ID,
                outputTokenProgram:TOKEN_PROGRAM_ID,
                inputTokenMint:inputMint,
                outputTokenMint:outputMint,
                observationState:new PublicKey(poolKeys.observationId),

            }).rpc();
            console.log("Swap 成功:", `https://solscan.io/tx/${swapTxId.toString()}?cluster=devnet`);
    })
})