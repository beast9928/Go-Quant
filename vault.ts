import { Router } from "express";
const router = Router();

router.post("/initialize", (_, res) => res.send("Vault init"));
router.post("/deposit", (_, res) => res.send("Deposit"));
router.post("/withdraw", (_, res) => res.send("Withdraw"));
router.get("/balance/:user", (_, res) => res.send("Balance"));
router.get("/transactions/:user", (_, res) => res.send("Tx history"));
router.get("/tvl", (_, res) => res.send("TVL"));

export default router;
