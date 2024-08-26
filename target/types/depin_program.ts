/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/depin_program.json`.
 */
export type DepinProgram = {
  "address": "3W7pnY6U3Aa7ERYf7KTwMmfNmyFRNTNivR4Ya6nKScXh",
  "metadata": {
    "name": "depinProgram",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "burn",
      "discriminator": [
        116,
        110,
        29,
        56,
        107,
        219,
        42,
        93
      ],
      "accounts": [
        {
          "name": "globalState",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108
                ]
              }
            ]
          }
        },
        {
          "name": "mintAccount",
          "writable": true
        },
        {
          "name": "escrowAccountA",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  115,
                  99,
                  114,
                  111,
                  119
                ]
              },
              {
                "kind": "const",
                "value": [
                  84,
                  111,
                  107,
                  101,
                  110,
                  65
                ]
              }
            ]
          }
        },
        {
          "name": "escrowAccountB",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  115,
                  99,
                  114,
                  111,
                  119
                ]
              },
              {
                "kind": "const",
                "value": [
                  84,
                  111,
                  107,
                  101,
                  110,
                  66
                ]
              }
            ]
          }
        },
        {
          "name": "escrowAccountC",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  115,
                  99,
                  114,
                  111,
                  119
                ]
              },
              {
                "kind": "const",
                "value": [
                  84,
                  111,
                  107,
                  101,
                  110,
                  67
                ]
              }
            ]
          }
        },
        {
          "name": "tokenAAta",
          "writable": true
        },
        {
          "name": "tokenBAta",
          "writable": true
        },
        {
          "name": "tokenCAta",
          "writable": true
        },
        {
          "name": "fromAccount",
          "writable": true
        },
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "initialize",
      "discriminator": [
        175,
        175,
        109,
        31,
        13,
        152,
        155,
        237
      ],
      "accounts": [
        {
          "name": "globalState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108
                ]
              }
            ]
          }
        },
        {
          "name": "escrowAccountA",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  115,
                  99,
                  114,
                  111,
                  119
                ]
              },
              {
                "kind": "const",
                "value": [
                  84,
                  111,
                  107,
                  101,
                  110,
                  65
                ]
              }
            ]
          }
        },
        {
          "name": "escrowAccountB",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  115,
                  99,
                  114,
                  111,
                  119
                ]
              },
              {
                "kind": "const",
                "value": [
                  84,
                  111,
                  107,
                  101,
                  110,
                  66
                ]
              }
            ]
          }
        },
        {
          "name": "escrowAccountC",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  115,
                  99,
                  114,
                  111,
                  119
                ]
              },
              {
                "kind": "const",
                "value": [
                  84,
                  111,
                  107,
                  101,
                  110,
                  67
                ]
              }
            ]
          }
        },
        {
          "name": "tokenA"
        },
        {
          "name": "tokenB"
        },
        {
          "name": "tokenC"
        },
        {
          "name": "mintAccount"
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": []
    },
    {
      "name": "mint",
      "discriminator": [
        51,
        57,
        225,
        47,
        182,
        146,
        137,
        166
      ],
      "accounts": [
        {
          "name": "globalState",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108
                ]
              }
            ]
          }
        },
        {
          "name": "mintAccount",
          "writable": true
        },
        {
          "name": "escrowAccountA",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  115,
                  99,
                  114,
                  111,
                  119
                ]
              },
              {
                "kind": "const",
                "value": [
                  84,
                  111,
                  107,
                  101,
                  110,
                  65
                ]
              }
            ]
          }
        },
        {
          "name": "escrowAccountB",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  115,
                  99,
                  114,
                  111,
                  119
                ]
              },
              {
                "kind": "const",
                "value": [
                  84,
                  111,
                  107,
                  101,
                  110,
                  66
                ]
              }
            ]
          }
        },
        {
          "name": "escrowAccountC",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  101,
                  115,
                  99,
                  114,
                  111,
                  119
                ]
              },
              {
                "kind": "const",
                "value": [
                  84,
                  111,
                  107,
                  101,
                  110,
                  67
                ]
              }
            ]
          }
        },
        {
          "name": "tokenAAta",
          "writable": true
        },
        {
          "name": "tokenBAta",
          "writable": true
        },
        {
          "name": "tokenCAta",
          "writable": true
        },
        {
          "name": "toAccount",
          "writable": true
        },
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "mintAuthority",
          "writable": true,
          "signer": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        }
      ],
      "args": [
        {
          "name": "tokenA",
          "type": "u64"
        },
        {
          "name": "tokenB",
          "type": "u64"
        },
        {
          "name": "tokenC",
          "type": "u64"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "globalState",
      "discriminator": [
        163,
        46,
        74,
        168,
        216,
        123,
        133,
        98
      ]
    }
  ],
  "events": [
    {
      "name": "burnEvent",
      "discriminator": [
        33,
        89,
        47,
        117,
        82,
        124,
        238,
        250
      ]
    },
    {
      "name": "createTokenEvent",
      "discriminator": [
        4,
        4,
        86,
        151,
        191,
        94,
        245,
        193
      ]
    },
    {
      "name": "initializeEvent",
      "discriminator": [
        206,
        175,
        169,
        208,
        241,
        210,
        35,
        221
      ]
    },
    {
      "name": "mintEvent",
      "discriminator": [
        197,
        144,
        146,
        149,
        66,
        164,
        95,
        16
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "amountCantBeZero",
      "msg": "Error: Amount can't be zero!"
    },
    {
      "code": 6001,
      "name": "unauthorized",
      "msg": "Error: Unauthorized User!"
    },
    {
      "code": 6002,
      "name": "unknownTokenA",
      "msg": "Error: Unknown Token A Passed!"
    },
    {
      "code": 6003,
      "name": "unknownTokenB",
      "msg": "Error: Unknown Token B Passed!"
    },
    {
      "code": 6004,
      "name": "unknownTokenC",
      "msg": "Error: Unknown Token C Passed!"
    }
  ],
  "types": [
    {
      "name": "burnEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "tokenA",
            "type": "u64"
          },
          {
            "name": "tokenB",
            "type": "u64"
          },
          {
            "name": "tokenC",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "createTokenEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "name",
            "docs": [
              "Token Name"
            ],
            "type": "string"
          }
        ]
      }
    },
    {
      "name": "globalState",
      "docs": [
        "The struct for storing global configuration"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "tokenA",
            "type": "pubkey"
          },
          {
            "name": "tokenB",
            "type": "pubkey"
          },
          {
            "name": "tokenC",
            "type": "pubkey"
          },
          {
            "name": "mintAccount",
            "type": "pubkey"
          },
          {
            "name": "totalStakedAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "initializeEvent",
      "type": {
        "kind": "struct",
        "fields": []
      }
    },
    {
      "name": "mintEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "amount",
            "type": "u64"
          }
        ]
      }
    }
  ],
  "constants": [
    {
      "name": "escrowTag",
      "type": "bytes",
      "value": "[101, 115, 99, 114, 111, 119]"
    },
    {
      "name": "globalTag",
      "type": "bytes",
      "value": "[103, 108, 111, 98, 97, 108]"
    },
    {
      "name": "tokenAWeightage",
      "type": "u64",
      "value": "40"
    },
    {
      "name": "tokenBWeightage",
      "type": "u64",
      "value": "30"
    },
    {
      "name": "tokenCWeightage",
      "type": "u64",
      "value": "30"
    }
  ]
};
