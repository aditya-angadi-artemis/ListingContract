export type Quidproquo = {
  "version": "0.0.0",
  "name": "quidproquo",
  "instructions": [
    {
      "name": "make",
      "accounts": [
        {
          "name": "offer",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "offerMaker",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "offerMakersMakerTokens",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "escrowedMakerTokens",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "makerMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenrent",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "escrowedMakerTokensBump",
          "type": "u8"
        },
        {
          "name": "offerBump",
          "type": "u8"
        },
        {
          "name": "offerTakerAmount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "accept",
      "accounts": [
        {
          "name": "offer",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "escrowedMakerTokens",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "makerMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "offerMaker",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "offerTaker",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "offerTakersMakerTokens",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenrent",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "offerBump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "cancel",
      "accounts": [
        {
          "name": "offer",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "offerMaker",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "offerMakersMakerTokens",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "makerMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "escrowedMakerTokens",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenrent",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "offerBump",
          "type": "u8"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "offer",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "maker",
            "type": "publicKey"
          },
          {
            "name": "takerAmount",
            "type": "u64"
          },
          {
            "name": "escrowedMakerTokensBump",
            "type": "u8"
          }
        ]
      }
    }
  ]
};

export const IDL: Quidproquo = {
  "version": "0.0.0",
  "name": "quidproquo",
  "instructions": [
    {
      "name": "make",
      "accounts": [
        {
          "name": "offer",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "offerMaker",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "offerMakersMakerTokens",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "escrowedMakerTokens",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "makerMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenrent",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "escrowedMakerTokensBump",
          "type": "u8"
        },
        {
          "name": "offerBump",
          "type": "u8"
        },
        {
          "name": "offerTakerAmount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "accept",
      "accounts": [
        {
          "name": "offer",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "escrowedMakerTokens",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "makerMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "offerMaker",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "offerTaker",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "offerTakersMakerTokens",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenrent",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "offerBump",
          "type": "u8"
        }
      ]
    },
    {
      "name": "cancel",
      "accounts": [
        {
          "name": "offer",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "offerMaker",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "offerMakersMakerTokens",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "makerMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "escrowedMakerTokens",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenrent",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "offerBump",
          "type": "u8"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "offer",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "maker",
            "type": "publicKey"
          },
          {
            "name": "takerAmount",
            "type": "u64"
          },
          {
            "name": "escrowedMakerTokensBump",
            "type": "u8"
          }
        ]
      }
    }
  ]
};
