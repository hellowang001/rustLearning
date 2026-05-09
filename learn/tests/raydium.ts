import * as anchor from "@coral-xyz/anchor";
import {
    createAssociatedTokenAccount,
    createMint,
    mintTo,
    getAccount,
    getAssociatedTokenAddressSync,
    TOKEN_2022_PROGRAM_ID,
    TOKEN_PROGRAM_ID,
    createInitializeMintInstruction,
    createInitializeMetadataPointerInstruction,
    getMintLen,
    ExtensionType,
    TYPE_SIZE,
    LENGTH_SIZE,
} from "@solana/spl-token";
import { createInitializeInstruction, pack } from "@solana/spl-token-metadata";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import {
    Raydium,
    DEVNET_PROGRAM_ID,
    TxVersion,
    CurveCalculator,
} from '@raydium-io/raydium-sdk-v2';
import BN from 'bn.js';

describe("keypair_vs_pda", () => {
    anchor.setProvider(anchor.AnchorProvider.env());
    const provider = anchor.getProvider() as anchor.AnchorProvider;
    const wallet = (provider.wallet as anchor.Wallet).payer;
    const connection = provider.connection;
    // const raydium = await initializeTransferHookInstructionData
    /*
    it("create Token2022 with metadata and mint 10000", async () => {
        const mintKeypair = anchor.web3.Keypair.generate();

        // 1. 定义元数据
        const metadata = {
            mint: mintKeypair.publicKey,
            name: "My Test Token",
            symbol: "MTT",
            uri: "https://ipfs.io/ipfs/bafkreibcglldkfdekdkxgumlveoe6qv3pbiceypkwtli33clbzul7leo4m",
            additionalMetadata: [],
        };

        // 2. 计算所需空间并创建带元数据的 Mint
        const mintLen = getMintLen([ExtensionType.MetadataPointer]);
        const metadataLen = TYPE_SIZE + LENGTH_SIZE + pack(metadata).length;
        const lamports = await connection.getMinimumBalanceForRentExemption(mintLen + metadataLen);

        const createMintTx = new anchor.web3.Transaction().add(
            SystemProgram.createAccount({
                fromPubkey: wallet.publicKey,
                newAccountPubkey: mintKeypair.publicKey,
                space: mintLen,
                lamports,
                programId: TOKEN_2022_PROGRAM_ID,
            }),
            createInitializeMetadataPointerInstruction(
                mintKeypair.publicKey,
                wallet.publicKey,
                mintKeypair.publicKey,
                TOKEN_2022_PROGRAM_ID,
            ),
            createInitializeMintInstruction(
                mintKeypair.publicKey,
                9,
                wallet.publicKey,
                null,
                TOKEN_2022_PROGRAM_ID,
            ),
            createInitializeInstruction({
                programId: TOKEN_2022_PROGRAM_ID,
                metadata: mintKeypair.publicKey,
                updateAuthority: wallet.publicKey,
                mint: mintKeypair.publicKey,
                mintAuthority: wallet.publicKey,
                name: metadata.name,
                symbol: metadata.symbol,
                uri: metadata.uri,
            }),
        );
        await anchor.web3.sendAndConfirmTransaction(connection, createMintTx, [wallet, mintKeypair]);
        console.log("Mint 地址:", mintKeypair.publicKey.toBase58());
        console.log("名称:", metadata.name, "| 符号:", metadata.symbol);

        // 3. 为自己创建 ATA
        const ata = await createAssociatedTokenAccount(
            connection,
            wallet,
            mintKeypair.publicKey,
            wallet.publicKey,
            undefined,
            TOKEN_2022_PROGRAM_ID,
        );
        console.log("ATA 地址:", ata.toBase58());

        // 4. 铸造 10000 枚代币
        const tx = await mintTo(
            connection,
            wallet,
            mintKeypair.publicKey,
            ata,
            wallet,
            10000 * 10 ** 9,
            [],
            undefined,
            TOKEN_2022_PROGRAM_ID,
        );
        console.log("https://solscan.io/tx/%s?cluster=devnet:", tx.toString());


        // 5. 验证余额
        const accountInfo = await getAccount(connection, ata, undefined, TOKEN_2022_PROGRAM_ID);
        console.log("铸造完成，当前余额:", Number(accountInfo.amount) / 10 ** 9, "枚");
    });
    */
    /*
    
    it("create Raydium CPMM pool", async () => {
        // 1. 初始化 Raydium SDK
        const raydium = await Raydium.load({
            owner: wallet,
            connection,
            cluster: "devnet",
            disableFeatureCheck: true,
            disableLoadToken: true,
            blockhashCommitment: "finalized",
        });

        // 2. 创建 mintA（普通 SPL Token）
        const mintA = await createMint(connection, wallet, wallet.publicKey, null, 9);
        const ataA = await createAssociatedTokenAccount(connection, wallet, mintA, wallet.publicKey);
        await mintTo(connection, wallet, mintA, ataA, wallet, 10000 * 10 ** 9);
        console.log("mintA:", mintA.toBase58());

        // 3. 创建 mintB（普通 SPL Token）
        const mintB = await createMint(connection, wallet, wallet.publicKey, null, 6);
        const ataB = await createAssociatedTokenAccount(connection, wallet, mintB, wallet.publicKey);
        await mintTo(connection, wallet, mintB, ataB, wallet, 10000 * 10 ** 6);
        console.log("mintB:", mintB.toBase58());

        // 4. 获取 Raydium devnet 手续费配置
        const feeConfigs = await raydium.api.getCpmmConfigs();
        console.log("使用费率配置:", feeConfigs[0].id);

        // 5. Raydium CPMM 要求 mintA 地址 < mintB 地址（字节序），否则建池失败
        const isAltB = mintA.toBuffer().compare(mintB.toBuffer()) < 0;
        const [sortedMintA, sortedMintB] = isAltB ? [mintA, mintB] : [mintB, mintA];
        const [sortedDecA, sortedDecB] = isAltB ? [9, 6] : [6, 9];
        const [sortedAmtA, sortedAmtB] = isAltB
            ? [new BN(1000 * 10 ** 9), new BN(1000 * 10 ** 6)]
            : [new BN(1000 * 10 ** 6), new BN(1000 * 10 ** 9)];

        // 6. 创建池子
        const { execute, extInfo } = await raydium.cpmm.createPool({
            programId: DEVNET_PROGRAM_ID.CREATE_CPMM_POOL_PROGRAM,
            poolFeeAccount: DEVNET_PROGRAM_ID.CREATE_CPMM_POOL_FEE_ACC,
            mintA: {
                address: sortedMintA.toBase58(),
                decimals: sortedDecA,
                programId: TOKEN_PROGRAM_ID.toBase58(),
            },
            mintB: {
                address: sortedMintB.toBase58(),
                decimals: sortedDecB,
                programId: TOKEN_PROGRAM_ID.toBase58(),
            },
            mintAAmount: sortedAmtA,
            mintBAmount: sortedAmtB,
            startTime: new BN(0),                  // 立即开始
            feeConfig: feeConfigs[0],
            associatedOnly: false,
            ownerInfo: { useSOLBalance: true },
            txVersion: TxVersion.V0,
        });

        const { txId } = await execute({ sendAndConfirm: true });
        console.log("池子创建成功:", `https://solscan.io/tx/${txId}?cluster=devnet`);
        console.log("池子地址:", extInfo.address.poolId.toBase58());
    })
        */
    it("swap tokens in Raydium CPMM pool", async () => {
        // 把上一个测试输出的池子地址填在这里
        const poolId = "8hc6S94q1L784DY6h99wSUdc8DuPniy4UZWKXd7tdBD4";

        // 1. 初始化 Raydium SDK
        const raydium = await Raydium.load({
            owner: wallet,
            connection,
            cluster: "devnet",
            disableFeatureCheck: true,
            disableLoadToken: true,
            blockhashCommitment: "finalized",
        });

        // 2. 从链上获取池子信息（rpcData 包含储备量和费率）
        const { poolInfo, poolKeys, rpcData } = await raydium.cpmm.getPoolInfoFromRpc(poolId);
        console.log("池子 mintA:", poolInfo.mintA.address, "mintB:", poolInfo.mintB.address);

        // 3. 计算 swap 结果（用 100 个 mintA 换 mintB）
        const inputAmount = new BN(100 * 10 ** poolInfo.mintA.decimals);
        const swapResult = CurveCalculator.swapBaseInput(
            inputAmount,
            rpcData.baseReserve,                    // mintA 当前储备量
            rpcData.quoteReserve,                   // mintB 当前储备量
            rpcData.configInfo!.tradeFeeRate,        // 交易手续费率
            rpcData.configInfo!.creatorFeeRate,      // 创建者费率（普通池为0）
            rpcData.configInfo!.protocolFeeRate,     // 协议费率
            rpcData.configInfo!.fundFeeRate,         // 基金费率
            false,                                   // isCreatorFeeOnInput（普通池为false）
        );
        console.log("预计换出:", swapResult.outputAmount.toString(), "个 mintB（最小单位）");

        // 4. 执行 swap
        const { execute } = await raydium.cpmm.swap({
            poolInfo,
            poolKeys,
            inputAmount,
            swapResult,
            slippage: 0.01,  // 1% 滑点容忍
            baseIn: true,    // true 表示用 mintA 换 mintB
            txVersion: TxVersion.V0,
        });

        const { txId: swapTxId } = await execute({ sendAndConfirm: true });
        console.log("Swap 成功:", `https://solscan.io/tx/${swapTxId}?cluster=devnet`);
    });

    /*
    it("Swap via our own program (CPI → Raydium CPMM) ", async () => {
        const poolId = "8hc6S94q1L784DY6h99wSUdc8DuPniy4UZWKXd7tdBD4";

        // 1. 初始化 Raydium SDK，获取池子所需的所有账户地址
        const raydium = await Raydium.load({
            owner: wallet,
            connection,
            cluster: "devnet",
            disableFeatureCheck: true,
            disableLoadToken: true,
            blockhashCommitment: "finalized",
        });
        const { poolInfo, poolKeys } = await raydium.cpmm.getPoolInfoFromRpc(poolId);

        // 2. 确定 input/output：用 mintA 换 mintB
        const inputMint  = new PublicKey(poolInfo.mintA.address);
        const outputMint = new PublicKey(poolInfo.mintB.address);

        const inputTokenAccount  = getAssociatedTokenAddressSync(inputMint,  wallet.publicKey);
        const outputTokenAccount = getAssociatedTokenAddressSync(outputMint, wallet.publicKey);

        // 3. 投入 10 个 mintA（注意精度）
        const amountIn        = new BN(10 * 10 ** poolInfo.mintA.decimals);
        const minAmountOut    = new BN(0); // 测试环境不做滑点保护

        // 4. 加载我们自己的 Anchor 程序
        const program = anchor.workspace.KeypairVsPda as anchor.Program;

        const cpSwapProgram = new PublicKey(DEVNET_PROGRAM_ID.CREATE_CPMM_POOL_PROGRAM.toString());

        const swapTxId=await program.methods
            .proxySwapBaseInput(amountIn, minAmountOut)
            .accounts({
                payer:               wallet.publicKey,
                authority:           new PublicKey(poolKeys.authority),
                ammConfig:           new PublicKey(poolKeys.config.id),
                poolState:           new PublicKey(poolId),
                inputTokenAccount:   inputTokenAccount,
                outputTokenAccount:  outputTokenAccount,
                inputVault:          new PublicKey(poolKeys.vault.A),
                outputVault:         new PublicKey(poolKeys.vault.B),
                inputTokenProgram:   TOKEN_PROGRAM_ID,
                outputTokenProgram:  TOKEN_PROGRAM_ID,
                inputTokenMint:      inputMint,
                outputTokenMint:     outputMint,
                observationState:    new PublicKey(poolKeys.observationId),
                cpSwapProgram:       cpSwapProgram,
            })
            .rpc();
        console.log("Swap 成功:", `https://solscan.io/tx/${swapTxId.toString()}?cluster=devnet`);

        console.log("通过我们自己的合约 CPI swap 成功！");
    });
    */



});
// ANCHOR_PROVIDER_URL=https://devnet.helius-rpc.com/?api-key=9e16b6ee-45f4-4544-80f3-92370659cb99 ANCHOR_WALLET=~/.config/solana/id.json yarn run ts-mocha -p ./tsconfig.json -t 1000000 "tests/raydium.ts"
