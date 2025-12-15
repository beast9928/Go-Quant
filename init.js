// @ts-nocheck
import * as anchor from "@coral-xyz/anchor";
import idl from "../target/idl/collateral_vault.json";
import { PublicKey, SystemProgram } from "@solana/web3.js";

const PROGRAM_ID = new PublicKey("47ZVKmD5b2c5XRfrAtnenNirfgq8NYrnLy8bFpJqxm45");

(async () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = new anchor.Program(idl, PROGRAM_ID, provider);
  const [vault] = PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), provider.wallet.publicKey.toBuffer()],
    PROGRAM_ID
  );

  await program.methods.initializeVault().accounts({
    user: provider.wallet.publicKey,
    vault,
    vaultTokenAccount: new PublicKey("USDT_VAULT_TOKEN_ACCOUNT"),
    systemProgram: SystemProgram.programId,
  }).rpc();

  console.log("Vault initialized");
})();
