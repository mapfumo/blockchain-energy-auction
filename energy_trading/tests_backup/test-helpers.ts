import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { EnergyTrading } from "../target/types/energy_trading";
import {
  PublicKey,
  Keypair,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import {
  createMint,
  createAccount,
  mintTo,
  getAccount,
  TOKEN_PROGRAM_ID,
  MINT_SIZE,
  getMinimumBalanceForRentExemptMint,
  createInitializeMintInstruction,
  createInitializeAccountInstruction,
  createMintToInstruction,
  getAssociatedTokenAddress,
  createAssociatedTokenAccountInstruction,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

export interface TestAccounts {
  // Program and provider
  program: Program<EnergyTrading>;
  provider: anchor.AnchorProvider;

  // USDC mint and token accounts
  usdcMint: Keypair;
  aggregatorUsdcAccount: PublicKey;
  batteryOwnerUsdcAccount: PublicKey;

  // Test users
  aggregator: Keypair;
  batteryOwner: Keypair;
  aggregatorAuthority: Keypair;

  // PDAs
  auctionPda: PublicKey;
  aggregatorPda: PublicKey;
  batteryPda: PublicKey;

  // Auction data
  auctionId: number;
  energyAmount: number;
  finalPrice: number;
}

export async function setupTestAccounts(): Promise<TestAccounts> {
  const program = anchor.workspace.EnergyTrading as Program<EnergyTrading>;
  const provider = anchor.getProvider();

  // Generate test keypairs
  const aggregator = Keypair.generate();
  const batteryOwner = Keypair.generate();
  const aggregatorAuthority = Keypair.generate();
  const usdcMint = Keypair.generate();

  // Airdrop SOL to test accounts
  await provider.connection.requestAirdrop(
    aggregator.publicKey,
    2 * LAMPORTS_PER_SOL
  );
  await provider.connection.requestAirdrop(
    batteryOwner.publicKey,
    2 * LAMPORTS_PER_SOL
  );
  await provider.connection.requestAirdrop(
    aggregatorAuthority.publicKey,
    2 * LAMPORTS_PER_SOL
  );

  // Create USDC mint
  const mintRent = await getMinimumBalanceForRentExemptMint(
    provider.connection
  );
  const mintTx = new anchor.web3.Transaction().add(
    SystemProgram.createAccount({
      fromPubkey: provider.wallet.publicKey,
      newAccountPubkey: usdcMint.publicKey,
      space: MINT_SIZE,
      lamports: mintRent,
      programId: TOKEN_PROGRAM_ID,
    }),
    createInitializeMintInstruction(
      usdcMint.publicKey,
      6, // USDC has 6 decimals
      provider.wallet.publicKey,
      null // No mint authority (immutable)
    )
  );

  await provider.sendAndConfirm(mintTx, [usdcMint]);

  // Create token accounts
  const aggregatorUsdcAccount = await getAssociatedTokenAddress(
    usdcMint.publicKey,
    aggregatorAuthority.publicKey
  );

  const batteryOwnerUsdcAccount = await getAssociatedTokenAddress(
    usdcMint.publicKey,
    batteryOwner.publicKey
  );

  // Create associated token accounts
  const createAggregatorTokenAccountTx = new anchor.web3.Transaction().add(
    createAssociatedTokenAccountInstruction(
      provider.wallet.publicKey,
      aggregatorUsdcAccount,
      aggregatorAuthority.publicKey,
      usdcMint.publicKey
    )
  );

  const createBatteryOwnerTokenAccountTx = new anchor.web3.Transaction().add(
    createAssociatedTokenAccountInstruction(
      provider.wallet.publicKey,
      batteryOwnerUsdcAccount,
      batteryOwner.publicKey,
      usdcMint.publicKey
    )
  );

  await provider.sendAndConfirm(createAggregatorTokenAccountTx);
  await provider.sendAndConfirm(createBatteryOwnerTokenAccountTx);

  // Mint USDC to aggregator account
  const mintAmount = 1000 * 10 ** 6; // 1000 USDC
  const mintToTx = new anchor.web3.Transaction().add(
    createMintToInstruction(
      usdcMint.publicKey,
      aggregatorUsdcAccount,
      provider.wallet.publicKey,
      mintAmount
    )
  );

  await provider.sendAndConfirm(mintToTx);

  // Generate PDAs
  const [auctionPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("auction"), new anchor.BN(1).toArrayLike(Buffer, "le", 8)],
    program.programId
  );

  const [aggregatorPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("aggregator"), aggregatorAuthority.publicKey.toBuffer()],
    program.programId
  );

  const [batteryPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("battery"), batteryOwner.publicKey.toBuffer()],
    program.programId
  );

  // Test data
  const auctionId = 1;
  const energyAmount = 100; // 100 kWh
  const finalPrice = 650; // 6.5 cents per kWh

  return {
    program,
    provider,
    usdcMint,
    aggregatorUsdcAccount,
    batteryOwnerUsdcAccount,
    aggregator,
    batteryOwner,
    aggregatorAuthority,
    auctionPda,
    aggregatorPda,
    batteryPda,
    auctionId,
    energyAmount,
    finalPrice,
  };
}

export async function initializeTestState(
  accounts: TestAccounts
): Promise<void> {
  const {
    program,
    aggregatorPda,
    batteryPda,
    aggregatorAuthority,
    batteryOwner,
  } = accounts;

  // Initialize aggregator account
  try {
    await program.methods
      .initialize()
      .accounts({
        aggregator: aggregatorPda,
        owner: aggregatorAuthority.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([aggregatorAuthority])
      .rpc();
  } catch (error) {
    // Account might already exist, that's okay
    console.log("Aggregator account may already exist");
  }

  // Initialize battery account
  try {
    await program.methods
      .initialize()
      .accounts({
        battery: batteryPda,
        owner: batteryOwner.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([batteryOwner])
      .rpc();
  } catch (error) {
    // Account might already exist, that's okay
    console.log("Battery account may already exist");
  }
}

export async function verifyUsdcBalance(
  connection: anchor.web3.Connection,
  tokenAccount: PublicKey,
  expectedAmount: number
): Promise<boolean> {
  try {
    const account = await getAccount(connection, tokenAccount);
    const balance = Number(account.amount);
    return balance >= expectedAmount;
  } catch (error) {
    return false;
  }
}

export async function verifyAccountExists(
  connection: anchor.web3.Connection,
  account: PublicKey
): Promise<boolean> {
  try {
    const accountInfo = await connection.getAccountInfo(account);
    return accountInfo !== null;
  } catch (error) {
    return false;
  }
}
