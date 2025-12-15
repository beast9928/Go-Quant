// @ts-nocheck
import * as anchor from "@coral-xyz/anchor";
import idl from "../target/idl/collateral_vault.json";
import { PublicKey, Connection } from "@solana/web3.js";

const PROGRAM_ID = new PublicKey("47ZVKmD5b2c5XRfrAtnenNirfgq8NYrnLy8bFpJqxm45");

(async () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = new anchor.Program(idl, PROGRAM_ID, provider);
  const [vault] = PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), provider.wallet.publicKey.toBuffer()],
    PROGRAM_ID
  );

  const state = await program.account.collateralVault.fetch(vault);
  console.log(state);
})();
