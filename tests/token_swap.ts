import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Swap } from "../target/types/swap";
import { Connection, PublicKey, clusterApiUrl, } from "@solana/web3.js";
import {
  createMint,
  createAccount,
  mintTo,
  getAssociatedTokenAddress,
  transfer
} from "@solana/spl-token";

anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.Swap as Program<Swap>;

describe("swap", () => {
  const connection = new Connection(clusterApiUrl('devnet'), "confirmed");

  const mint_t1_kp = anchor.web3.Keypair.fromSecretKey(
    Uint8Array.from([
      117, 103, 199, 25, 207, 169, 56, 0, 60, 241, 228, 245, 8, 230, 54, 61, 253, 0, 9,
      206, 193, 26, 150, 254, 111, 197, 96, 11, 41, 198, 168, 225, 68, 204, 117, 205, 27,
      39, 226, 133, 72, 212, 183, 228, 2, 217, 129, 104, 92, 78, 27, 157, 53, 54, 7, 202, 187,
      66, 67, 230, 109, 100, 103, 127
    ])
  );

  const mint_t2_kp = anchor.web3.Keypair.fromSecretKey(
    Uint8Array.from([118, 47, 65, 192, 12, 177, 109, 61, 242, 160, 69, 132, 89, 68, 6,
      165, 57, 150, 171, 3, 24, 202, 226, 106, 197, 161, 232, 15, 135, 121, 111, 74, 173,
      210, 30, 22, 103, 240, 225, 139, 154, 228, 37, 155, 244, 38, 67, 73, 138, 205, 81,
      96, 68, 128, 24, 73, 67, 238, 47, 208, 88, 98, 251, 159])
  );

  const payer = anchor.web3.Keypair.fromSecretKey(
    Uint8Array.from([213, 70, 40, 87, 126, 0, 93, 73, 139, 108, 227, 225, 117,
      242, 254, 199, 60, 179, 134, 142, 10, 224, 126, 14, 119, 172, 20, 129, 65, 12,
      29, 54, 106, 243, 69, 118, 11, 26, 25, 215, 126, 74, 51, 116, 176, 178, 125, 20,
      182, 169, 217, 45, 111, 40, 167, 180, 165, 154, 119, 1, 135, 80, 133, 62])
  );

  const seeds_one = get_seeds("vcdfgtre");
  const seeds_two = get_seeds("asdewqas");
  const seeds_three = get_seeds("kiuhyrfg");
  const seeds_four = get_seeds("pool_owner");
  const [pda1, bump1] = getPdaFromSeeds(seeds_one);
  const [pda2, bump2] = getPdaFromSeeds(seeds_two);
  const [pda3, bump3] = getPdaFromSeeds(seeds_three);
  const [pda4, bump4] = getPdaFromSeeds(seeds_four);

  console.log(
    `Pda 1 - ${pda1} & bump1 - ${bump1}\nPda 2 - ${pda2} & bump2 - ${bump2}\nPda 3 - ${pda3} & bump3 - ${bump3}\nPda 4 - ${pda4} & bump4 - ${bump4}`
  );

  it("mint the tokens", async () => {
    const mint_pk1 = await createMint(
      connection,
      payer,
      payer.publicKey,
      payer.publicKey,
      0,
      mint_t1_kp
    );
    console.log(`Mint - ${mint_pk1}`);
    //
    const mint_pk2 = await createMint(
      connection,
      payer,
      payer.publicKey,
      payer.publicKey,
      0,
      mint_t2_kp
    );
    console.log(`Mint - ${mint_pk2}`);
  });

  it("create token accounts", async () => {
    const token_user_t1_pk = await createAccount(
      connection,
      payer,
      mint_t1_kp.publicKey,
      payer.publicKey
    );
    console.log(
      `Newly created ata - ${token_user_t1_pk} for mint - ${mint_t1_kp.publicKey}`
    );
    //
    const token_user_t2_pk = await createAccount(
      connection,
      payer,
      mint_t2_kp.publicKey,
      payer.publicKey
    );
    console.log(
      `Newly created ata - ${token_user_t2_pk} for mint - ${mint_t1_kp.publicKey}`
    );
  });

  it("mint tokens", async () => {
    const amount = 10000;
    //
    const ata_user_t1_pk = await getAssociatedTokenAddress(
      mint_t1_kp.publicKey,
      payer.publicKey
    );
    const tx1 = await mintTo(
      connection,
      payer,
      mint_t1_kp.publicKey,
      ata_user_t1_pk,
      payer,
      1000
    );
    console.log(
      `MintTo - ${amount} tokens of mint - ${mint_t1_kp.publicKey} were minted to - ${ata_user_t1_pk}\ntx hash - ${tx1}`
    );
    //
    const ata_user_t2_pk = await getAssociatedTokenAddress(
      mint_t2_kp.publicKey,
      payer.publicKey
    );
    const tx2 = await mintTo(
      connection,
      payer,
      mint_t2_kp.publicKey,
      ata_user_t2_pk,
      payer,
      1000
    );
    console.log(
      `MintTo - ${amount} tokens of mint - ${mint_t2_kp.publicKey} were minted to - ${ata_user_t2_pk}\ntx hash - ${tx2}`
    );
  });

  it("create pool", async () => {
    try {
      const tx = await program.methods
        .createPool(
          seeds_one,
          seeds_two,
          seeds_three,
          seeds_four,
          new anchor.BN(10)
        )
        .accounts({
          pool: pda1,
          payer: payer.publicKey,
          token1Mint: mint_t1_kp.publicKey,
          token1Pool: pda2,
          token2Mint: mint_t2_kp.publicKey,
          token2Pool: pda3,
          poolOwner: pda4
        })
        .signers([payer])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (e) {
      console.log(e);
    }
  });

  it("transfer tokens to newly created pda token accounts", async () => {
    const amount = 500;
    const ata_user_t1_pk = await getAssociatedTokenAddress(
      mint_t1_kp.publicKey,
      payer.publicKey
    );
    const tx1 = await transfer(
      connection,
      payer,
      ata_user_t1_pk,
      pda2,
      payer,
      amount
    );
    console.log(
      `Amount - ${amount} were transferred from - ${ata_user_t1_pk} to pda - ${pda2}\ntx hash - ${tx1}`
    );
    //
    const ata_user_t2_pk = await getAssociatedTokenAddress(
      mint_t2_kp.publicKey,
      payer.publicKey
    );
    const tx2 = await transfer(
      connection,
      payer,
      ata_user_t2_pk,
      pda3,
      payer,
      amount
    );
    console.log(
      `Amount - ${amount} were transferred from - ${ata_user_t2_pk} to pda - ${pda3}\ntx hash - ${tx2}`
    );
  });

  it("swap t1 for t2", async () => {
    try {
      const swap = {
        token2ForToken1: { amount: new anchor.BN(50) }
      };
      const ata_user_t1_pk = await getAssociatedTokenAddress(
        mint_t1_kp.publicKey,
        payer.publicKey
      );
      const ata_user_t2_pk = await getAssociatedTokenAddress(
        mint_t2_kp.publicKey,
        payer.publicKey
      );
      const tx = await program.methods
        .swapToken(bump4, swap)
        .accounts({
          user: payer.publicKey,
          pool: pda1,
          userToken1: ata_user_t1_pk,
          userToken2: ata_user_t2_pk,
          token1Pool: pda2,
          token2Pool: pda3,
          poolOwner: pda4
        })
        .signers([payer])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (e) {
      console.log(e);
    }
  });
});

function getPda(seed_str: any) {
  return getPdaFromSeeds(get_seeds(seed_str));
}

function getPdaFromSeeds(seeds: any) {
  return PublicKey.findProgramAddressSync(
    [Uint8Array.from(seeds)],
    program.programId
  );
}

function get_seeds(seed_str: any) {
  return [...seed_str].map((char) => char.codePointAt());
}