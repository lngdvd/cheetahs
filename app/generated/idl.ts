import { Idl } from "@coral-xyz/anchor";

const IDL: Idl = {
  address: "G5REtjRJu24GyTM8AVpT2qH5gAXFjVNF7Uhbmpe4MEpG",
  metadata: {
    name: "peregrinus",
    version: "0.1.0",
    spec: "0.1.0",
    description: "Created with Anchor",
  },
  instructions: [
    {
      name: "close_staking",
      discriminator: [61, 183, 134, 159, 84, 41, 222, 193],
      accounts: [
        {
          name: "owner",
          writable: true,
          signer: true,
        },
        {
          name: "user_stake",
          writable: true,
        },
      ],
      args: [],
    },
    {
      name: "initialize",
      discriminator: [175, 175, 109, 31, 13, 152, 155, 237],
      accounts: [],
      args: [],
    },
    {
      name: "mint_sbt",
      discriminator: [135, 204, 128, 71, 117, 168, 158, 28],
      accounts: [
        {
          name: "payer",
          writable: true,
          signer: true,
        },
        {
          name: "mint",
          writable: true,
        },
        {
          name: "token_program_2022",
          address: "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb",
        },
        {
          name: "system_program",
          address: "11111111111111111111111111111111",
        },
      ],
      args: [],
    },
    {
      name: "redeem_stakerewards",
      discriminator: [226, 28, 75, 46, 125, 36, 130, 15],
      accounts: [
        {
          name: "owner",
          writable: true,
          signer: true,
        },
        {
          name: "user_stake_info",
          writable: true,
          pda: {
            seeds: [
              {
                kind: "const",
                value: [117, 115, 101, 114, 95, 115, 116, 97, 107, 101],
              },
              {
                kind: "account",
                path: "owner",
              },
              {
                kind: "account",
                path: "mint",
              },
            ],
          },
        },
        {
          name: "mint",
          writable: true,
        },
        {
          name: "user_nft_token_account",
          writable: true,
        },
        {
          name: "pda_nft_token_account",
          writable: true,
        },
        {
          name: "associated_token_program",
          address: "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
        },
        {
          name: "token_program",
          address: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
        },
        {
          name: "system_program",
          address: "11111111111111111111111111111111",
        },
      ],
      args: [],
    },
    {
      name: "stake_nft",
      discriminator: [38, 27, 66, 46, 69, 65, 151, 219],
      accounts: [
        {
          name: "owner",
          writable: true,
          signer: true,
        },
        {
          name: "user_stake_info",
          writable: true,
          pda: {
            seeds: [
              {
                kind: "const",
                value: [117, 115, 101, 114, 95, 115, 116, 97, 107, 101],
              },
              {
                kind: "account",
                path: "owner",
              },
              {
                kind: "account",
                path: "mint",
              },
            ],
          },
        },
        {
          name: "mint",
          writable: true,
        },
        {
          name: "user_nft_token_account",
          writable: true,
        },
        {
          name: "pda_nft_token_account",
          writable: true,
        },
        {
          name: "associated_token_program",
          address: "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
        },
        {
          name: "token_program",
          address: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
        },
        {
          name: "system_program",
          address: "11111111111111111111111111111111",
        },
      ],
      args: [],
    },
    {
      name: "unstake_nft",
      discriminator: [17, 182, 24, 211, 101, 138, 50, 163],
      accounts: [
        {
          name: "owner",
          writable: true,
          signer: true,
        },
        {
          name: "user_stake_info",
          writable: true,
          pda: {
            seeds: [
              {
                kind: "const",
                value: [117, 115, 101, 114, 95, 115, 116, 97, 107, 101],
              },
              {
                kind: "account",
                path: "owner",
              },
              {
                kind: "account",
                path: "mint",
              },
            ],
          },
        },
        {
          name: "mint",
          writable: true,
        },
        {
          name: "user_nft_token_account",
          writable: true,
        },
        {
          name: "pda_nft_token_account",
          writable: true,
        },
        {
          name: "associated_token_program",
          address: "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
        },
        {
          name: "token_program",
          address: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
        },
        {
          name: "system_program",
          address: "11111111111111111111111111111111",
        },
      ],
      args: [],
    },
  ],
  accounts: [
    {
      name: "UserStake",
      discriminator: [102, 53, 163, 107, 9, 138, 87, 153],
    },
  ],
  errors: [
    {
      code: 6000,
      name: "StillStaking",
      msg: "Please unstake NFT first",
    },
    {
      code: 6001,
      name: "IsInitialized",
      msg: "Configurations is initialized",
    },
    {
      code: 6002,
      name: "IsNotAuthority",
      msg: "Is not authority",
    },
    {
      code: 6003,
      name: "MintingLimitExceeded",
      msg: "Minting Limit Exceeded",
    },
    {
      code: 6004,
      name: "ZeroRecipient",
      msg: "0x0 recipient not allowed",
    },
    {
      code: 6005,
      name: "InvalidTokenId",
      msg: "Invalid tokenId",
    },
    {
      code: 6006,
      name: "InvalidParams",
      msg: "Invalid params",
    },
    {
      code: 6007,
      name: "InvalidTokenAccount",
      msg: "Invalid params",
    },
  ],
  types: [
    {
      name: "UserStake",
      type: {
        kind: "struct",
        fields: [
          {
            name: "owner",
            type: "pubkey",
          },
          {
            name: "mint",
            type: "pubkey",
          },
          {
            name: "start_time",
            type: "u64",
          },
          {
            name: "last_redeem",
            type: "u64",
          },
          {
            name: "is_staking",
            type: "bool",
          },
          {
            name: "bump",
            type: "u8",
          },
        ],
      },
    },
  ],
  constants: [
    {
      name: "CNFT_CONFIG_SEED",
      type: "bytes",
      value: "[99, 78, 70, 84, 45, 99, 111, 110, 102, 105, 103]",
    },
    {
      name: "USER_STAKE_SEED",
      type: "bytes",
      value: "[117, 115, 101, 114, 95, 115, 116, 97, 107, 101]",
    },
  ],
};

export default IDL;
