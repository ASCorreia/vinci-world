import * as anchor from "@project-serum/anchor";
import * as web3 from "@solana/web3.js";
import {Program} from "@project-serum/anchor";
import {Gifportal} from "../target/types/gifportal";
import {TOKEN_PROGRAM_ID, MINT_SIZE, createAssociatedTokenAccountInstruction, getAssociatedTokenAddress, createInitializeMintInstruction} from "@solana/spl-token"; //0.3.5
import assert from 'assert';
import { publicKey } from "@project-serum/anchor/dist/cjs/utils";


describe("Token-Contract", () => {
    const key = anchor.AnchorProvider.env();
    anchor.setProvider(key);

    const program = anchor.workspace.Gifportal as Program<Gifportal>;

    const mintKey: anchor.web3.Keypair = anchor.web3.Keypair.generate();

    let associatedTokenAccount: anchor.web3.PublicKey = key.wallet.publicKey;

    it("Mint and Burn a token", async() => {
        //const key = key.wallet.publicKey;
        const lamports: number = await program.provider.connection.getMinimumBalanceForRentExemption(MINT_SIZE);

        //Get the ATA for a token on a public key (but might not exist yet)
        associatedTokenAccount = await getAssociatedTokenAddress(mintKey.publicKey, key.wallet.publicKey);

        //Fires a list of instructions
        const mint_tx = new anchor.web3.Transaction().add(
            //Use anchor to creante an account from the key we created
            anchor.web3.SystemProgram.createAccount({
                fromPubkey: key.wallet.publicKey,
                newAccountPubkey: mintKey.publicKey,
                space: MINT_SIZE,
                programId: TOKEN_PROGRAM_ID,
                lamports
            }),
            //creates, through a transaction, our mint account that is controlled by our anchor wallet (key)
            createInitializeMintInstruction(mintKey.publicKey, 0, key.wallet.publicKey, key.wallet.publicKey),
            
            //Creates the ATA account that is associated with our mint on our anchor wallet (key)
            createAssociatedTokenAccountInstruction(key.wallet.publicKey, associatedTokenAccount, key.wallet.publicKey, mintKey.publicKey)
        );

        //Sends and create the transaction
        const res = await key.sendAndConfirm(mint_tx, [mintKey]);

        console.log(await program.provider.connection.getParsedAccountInfo(mintKey.publicKey));

        console.log("Account: ", res);
        console.log("Mint Key: ", mintKey.publicKey.toString());
        console.log("User: ", key.wallet.publicKey.toString());
        /* 
        //Creates public keys struct from buffers and / or Strings
        console.log("User: ", key.wallet.publicKey.toBuffer());
        console.log("Enconding: ", new anchor.web3.PublicKey(key.wallet.publicKey.toString()));
        console.log("Enconding: ", new anchor.web3.PublicKey(key.wallet.publicKey.toBuffer()));
        */


        //Starts the Mint Operation
        console.log("Starting Mint Operation (Minting 10 tokens)");
        //Executes our smart contract to mint our token into our specified ATA
        const tx = await program.methods.mintToken(new anchor.BN(10)).accounts({
            mint: mintKey.publicKey,
            tokenProgram: TOKEN_PROGRAM_ID,
            tokenAccount: associatedTokenAccount,
            payer: key.wallet.publicKey,
        }).rpc();
        //Transaction logs
        console.log("Your transaction signature ", tx);
        const minted = (await program.provider.connection.getParsedAccountInfo(associatedTokenAccount)).value.data.parsed.info.tokenAmount.amount;
        console.log("Ammount of token minted: ", minted);
        assert.equal(minted, new anchor.BN(10));
        console.log("Mint Operation Finished!");

        //Starts the Burn Operation
        console.log("Starting Burn Operation (Burning 7 tokens)");
        //Executes our smart contract to burn our token into our specified ATA
        const tx2 = await program.methods.burnToken(new anchor.BN(7)).accounts({
            mint: mintKey.publicKey,
            tokenProgram: TOKEN_PROGRAM_ID,
            tokenAccount: associatedTokenAccount,
            payer: key.wallet.publicKey,
        }).rpc();
        //Transaction logs
        console.log("Your transaction signature ", tx2);
        const minted2 = (await program.provider.connection.getParsedAccountInfo(associatedTokenAccount)).value.data.parsed.info.tokenAmount.amount;
        console.log("Total number of tokens after Burn: ", minted2);
        assert.equal(minted2, new anchor.BN(3));
        console.log("Burn Operation Finished!");
    });

    it ("Quest simulation", async() => {
        const account = anchor.web3.Keypair.generate();

        const [vinciWorldPDA, _] = await web3.PublicKey.findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode("Placeholder_13"),
          key.wallet.publicKey.toBuffer(),
        ],
        program.programId
        );

        console.log("Vinci World PDA address ", vinciWorldPDA);
        
        const tx = await program.methods.startStuffOff().accounts({
              baseAccount: vinciWorldPDA, //account.publicKey,
              user: key.wallet.publicKey,
              systemProgram: web3.SystemProgram.programId,
            }).rpc(); //.signers[account] before rpc()

        console.log("Vinci World PDA account created with Transaction", tx);

        /*
        const tx = await program.rpc.startStuffOff({
            accounts: {
              baseAccount: account.publicKey,
              user: key.wallet.publicKey,
              systemProgram: web3.SystemProgram.programId,
            },
            signers: [account]
          });*/

        let fetchAccount = await program.account.baseAccount.fetch(vinciWorldPDA); //account.publicKey
        
        console.log("Total Ammount Of Tokens", fetchAccount.totalGifs.toString());
        console.log("Owner of the account: ", fetchAccount.owner.toString());
        console.log("Address of the provider: ", key.wallet.publicKey.toString());
        assert.equal(fetchAccount.totalGifs.toString(), "0");

        const addValue = await program.methods.addAmmount(new anchor.BN(15)).accounts({
            baseAccount: vinciWorldPDA, //account.publicKey,
        }).rpc();
        //can we pass more than one ammount and accounts?

        let fetchAccount2 = await program.account.baseAccount.fetch(vinciWorldPDA); //account.publicKey
        console.log("Match won - 15 Tokens awarded");
        console.log("Total Ammount Of Tokens", fetchAccount2.totalGifs.toString());
        assert.equal(fetchAccount2.totalGifs.toString(), "15");

        const tournamentPay = await program.methods.payTournament(new anchor.BN(30)).accounts({
        }).remainingAccounts([{pubkey: vinciWorldPDA, isSigner: false, isWritable: true}]).rpc();

        let fetchAccount3 = await program.account.baseAccount.fetch(vinciWorldPDA); //account.publicKey
        console.log("Tournament won - 30 Tokens awarded");
        console.log("Total Ammount Of Tokens", fetchAccount3.totalGifs.toString());
        assert.equal(fetchAccount3.totalGifs.toString(), "45");
    });
});

