import express from "express";
import vaultRoutes from "./routes/vault";

const app = express();
app.use(express.json());
app.use("/vault", vaultRoutes);
app.listen(3000);
