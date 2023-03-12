const anchor = require("@project-serum/anchor");
const {Program} = require("@project-serum/anchor");

const main = async() => {
  console.log("Starting Tests...");

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Gifportal;

  const baseAccount = anchor.web3.Keypair.generate();
  const tx = await program.rpc.startStuffOff({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    },
    signers: [baseAccount]
  });
  console.log("Transaction signature: ", tx);

  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log("Total Ammount Deposited", account.totalGifs.toString());
 
  await program.rpc.addGif("1",{
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });

  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log("Deposit Received", account.gifList[0].userAddress);
  console.log("Deposit Received", account.gifList[0].userAddress.toString());
  console.log("Total Ammount Deposited", account.totalGifs.toString());

  await program.rpc.removeGif("AHYic562KhgtAEkb1rSesqS87dFYRcfXb4WwWus3Zc9C",{
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });

  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log("Total Ammount Deposited", account.totalGifs.toString());
  console.log("Deposit Received", account.gifList);

  console.log("Mint a token");
  associatedTokenAccount = await getAssociatedTokenAddress(mintKey.publicKey, provider.publicKey);

}

const runMain = async() => {
  try {
    await main();
    process.exit(0);
  } catch(error) {
    console.error(error);
    process.exit(1);
  }
}

runMain();
