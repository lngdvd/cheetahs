/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/peregrinus.json`.
 */
export type Peregrinus = {
  address: "G5REtjRJu24GyTM8AVpT2qH5gAXFjVNF7Uhbmpe4MEpG";
  metadata: {
    name: "peregrinus";
    version: "0.1.0";
    spec: "0.1.0";
    description: "Created with Anchor";
  };
  instructions: [
    {
      name: "closeStaking";
      discriminator: [61, 183, 134, 159, 84, 41, 222, 193];
      accounts: [
        {
          name: "owner";
          writable: true;
          signer: true;
        },
        {
          name: "userStake";
          writable: true;
        }
      ];
      args: [];
    },
    {
      name: "initialize";
      discriminator: [175, 175, 109, 31, 13, 152, 155, 237];
      accounts: [];
      args: [];
    },
    {
      name: "mintSbt";
      discriminator: [135, 204, 128, 71, 117, 168, 158, 28];
      accounts: [
        {
          name: "payer";
          writable: true;
          signer: true;
        },
        {
          name: "mint";
          writable: true;
        },
        {
          name: "tokenProgram2022";
          address: "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb";
        },
        {
          name: "systemProgram";
          address: "11111111111111111111111111111111";
        }
      ];
      args: [];
    },
    {
      name: "redeemStakerewards";
      discriminator: [226, 28, 75, 46, 125, 36, 130, 15];
      accounts: [
        {
          name: "owner";
          writable: true;
          signer: true;
        },
        {
          name: "userStakeInfo";
          writable: true;
          pda: {
            seeds: [
              {
                kind: "const";
                value: [117, 115, 101, 114, 95, 115, 116, 97, 107, 101];
              },
              {
                kind: "account";
                path: "owner";
              },
              {
                kind: "account";
                path: "mint";
              }
            ];
          };
        },
        {
          name: "mint";
          writable: true;
        },
        {
          name: "userNftTokenAccount";
          writable: true;
        },
        {
          name: "pdaNftTokenAccount";
          writable: true;
        },
        {
          name: "associatedTokenProgram";
          address: "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";
        },
        {
          name: "tokenProgram";
          address: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
        },
        {
          name: "systemProgram";
          address: "11111111111111111111111111111111";
        }
      ];
      args: [];
    },
    {
      name: "stakeNft";
      discriminator: [38, 27, 66, 46, 69, 65, 151, 219];
      accounts: [
        {
          name: "owner";
          writable: true;
          signer: true;
        },
        {
          name: "userStakeInfo";
          writable: true;
          pda: {
            seeds: [
              {
                kind: "const";
                value: [117, 115, 101, 114, 95, 115, 116, 97, 107, 101];
              },
              {
                kind: "account";
                path: "owner";
              },
              {
                kind: "account";
                path: "mint";
              }
            ];
          };
        },
        {
          name: "mint";
          writable: true;
        },
        {
          name: "userNftTokenAccount";
          writable: true;
        },
        {
          name: "pdaNftTokenAccount";
          writable: true;
        },
        {
          name: "associatedTokenProgram";
          address: "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";
        },
        {
          name: "tokenProgram";
          address: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
        },
        {
          name: "systemProgram";
          address: "11111111111111111111111111111111";
        }
      ];
      args: [];
    },
    {
      name: "unstakeNft";
      discriminator: [17, 182, 24, 211, 101, 138, 50, 163];
      accounts: [
        {
          name: "owner";
          writable: true;
          signer: true;
        },
        {
          name: "userStakeInfo";
          writable: true;
          pda: {
            seeds: [
              {
                kind: "const";
                value: [117, 115, 101, 114, 95, 115, 116, 97, 107, 101];
              },
              {
                kind: "account";
                path: "owner";
              },
              {
                kind: "account";
                path: "mint";
              }
            ];
          };
        },
        {
          name: "mint";
          writable: true;
        },
        {
          name: "userNftTokenAccount";
          writable: true;
        },
        {
          name: "pdaNftTokenAccount";
          writable: true;
        },
        {
          name: "associatedTokenProgram";
          address: "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";
        },
        {
          name: "tokenProgram";
          address: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
        },
        {
          name: "systemProgram";
          address: "11111111111111111111111111111111";
        }
      ];
      args: [];
    }
  ];
  accounts: [
    {
      name: "userStake";
      discriminator: [102, 53, 163, 107, 9, 138, 87, 153];
    }
  ];
  errors: [
    {
      code: 6000;
      name: "stillStaking";
      msg: "Please unstake NFT first";
    },
    {
      code: 6001;
      name: "isInitialized";
      msg: "Configurations is initialized";
    },
    {
      code: 6002;
      name: "isNotAuthority";
      msg: "Is not authority";
    },
    {
      code: 6003;
      name: "mintingLimitExceeded";
      msg: "Minting Limit Exceeded";
    },
    {
      code: 6004;
      name: "zeroRecipient";
      msg: "0x0 recipient not allowed";
    },
    {
      code: 6005;
      name: "invalidTokenId";
      msg: "Invalid tokenId";
    },
    {
      code: 6006;
      name: "invalidParams";
      msg: "Invalid params";
    },
    {
      code: 6007;
      name: "invalidTokenAccount";
      msg: "Invalid params";
    }
  ];
  types: [
    {
      name: "userStake";
      type: {
        kind: "struct";
        fields: [
          {
            name: "owner";
            type: "pubkey";
          },
          {
            name: "mint";
            type: "pubkey";
          },
          {
            name: "startTime";
            type: "u64";
          },
          {
            name: "lastRedeem";
            type: "u64";
          },
          {
            name: "isStaking";
            type: "bool";
          },
          {
            name: "bump";
            type: "u8";
          }
        ];
      };
    }
  ];
  constants: [
    {
      name: "cnftConfigSeed";
      type: "bytes";
      value: "[99, 78, 70, 84, 45, 99, 111, 110, 102, 105, 103]";
    },
    {
      name: "userStakeSeed";
      type: "bytes";
      value: "[117, 115, 101, 114, 95, 115, 116, 97, 107, 101]";
    }
  ];
};
