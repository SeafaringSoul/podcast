/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/podcast.json`.
 */
export type Podcast = {
  "address": "FZKxcgMWhMmXVoWM7iwN8GNzmE8WHZ1pt7SiJDCUsFtR",
  "metadata": {
    "name": "podcast",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "createChannel",
      "discriminator": [
        37,
        105,
        253,
        99,
        87,
        46,
        223,
        20
      ],
      "accounts": [
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "pdaPodcastChannel",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  100,
                  99,
                  97,
                  115,
                  116,
                  95,
                  99,
                  104,
                  97,
                  110,
                  110,
                  101,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              },
              {
                "kind": "arg",
                "path": "title"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "title",
          "type": "bytes"
        },
        {
          "name": "description",
          "type": "bytes"
        },
        {
          "name": "imageUrl",
          "type": "bytes"
        }
      ]
    },
    {
      "name": "createEp",
      "discriminator": [
        160,
        31,
        115,
        88,
        196,
        95,
        22,
        197
      ],
      "accounts": [
        {
          "name": "creator",
          "writable": true,
          "signer": true,
          "relations": [
            "pdaPodcastChannel"
          ]
        },
        {
          "name": "pdaPodcastEp",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  100,
                  99,
                  97,
                  115,
                  116,
                  95,
                  101,
                  112
                ]
              },
              {
                "kind": "account",
                "path": "creator"
              },
              {
                "kind": "arg",
                "path": "title"
              }
            ]
          }
        },
        {
          "name": "pdaPodcastChannel",
          "writable": true
        },
        {
          "name": "nftMint",
          "writable": true,
          "signer": true
        },
        {
          "name": "nftMetadata",
          "writable": true
        },
        {
          "name": "creatorNftAccount",
          "docs": [
            "Associated token account for creator"
          ],
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "creator"
              },
              {
                "kind": "const",
                "value": [
                  6,
                  221,
                  246,
                  225,
                  215,
                  101,
                  161,
                  147,
                  217,
                  203,
                  225,
                  70,
                  206,
                  235,
                  121,
                  172,
                  28,
                  180,
                  133,
                  237,
                  95,
                  91,
                  55,
                  145,
                  58,
                  140,
                  245,
                  133,
                  126,
                  255,
                  0,
                  169
                ]
              },
              {
                "kind": "account",
                "path": "nftMint"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "rent",
          "address": "SysvarRent111111111111111111111111111111111"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "tokenMetadataProgram"
        }
      ],
      "args": [
        {
          "name": "etitle",
          "type": "bytes"
        },
        {
          "name": "description",
          "type": "bytes"
        },
        {
          "name": "audioUrl",
          "type": "bytes"
        },
        {
          "name": "duration",
          "type": "u64"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "podcastChannel",
      "discriminator": [
        81,
        228,
        44,
        185,
        210,
        184,
        53,
        231
      ]
    },
    {
      "name": "podcastEp",
      "discriminator": [
        131,
        176,
        125,
        76,
        157,
        193,
        232,
        128
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "insufficientBalance",
      "msg": "Insufficient SOL balance to create channel"
    },
    {
      "code": 6001,
      "name": "episodeNotNft",
      "msg": "该单集不是 NFT"
    },
    {
      "code": 6002,
      "name": "invalidMetadataProgramId",
      "msg": "Invalid Metaplex Metadata program ID"
    },
    {
      "code": 6003,
      "name": "invalidEpisodeCreator",
      "msg": "Invalid episode error"
    },
    {
      "code": 6004,
      "name": "invalidMetadataPda",
      "msg": "Metadata PDA 与 Mint 不匹配"
    },
    {
      "code": 6005,
      "name": "invalidUtf8",
      "msg": "Title 不是有效的 UTF-8 字符串"
    },
    {
      "code": 6006,
      "name": "invalidChannelAuthority",
      "msg": ""
    }
  ],
  "types": [
    {
      "name": "nftHolder",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "address",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "podcastChannel",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "id",
            "type": "bytes"
          },
          {
            "name": "title",
            "type": "bytes"
          },
          {
            "name": "description",
            "type": "bytes"
          },
          {
            "name": "imageUrl",
            "type": "bytes"
          },
          {
            "name": "creator",
            "type": "pubkey"
          },
          {
            "name": "createAt",
            "type": "i64"
          },
          {
            "name": "updatedAt",
            "type": "i64"
          },
          {
            "name": "isNft",
            "type": "bool"
          },
          {
            "name": "nftMint",
            "type": {
              "option": "pubkey"
            }
          },
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "isPublic",
            "type": "bool"
          },
          {
            "name": "rssFeedUrl",
            "type": "string"
          },
          {
            "name": "episodes",
            "type": {
              "vec": "pubkey"
            }
          }
        ]
      }
    },
    {
      "name": "podcastEp",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "id",
            "type": "bytes"
          },
          {
            "name": "title",
            "type": "bytes"
          },
          {
            "name": "description",
            "type": "bytes"
          },
          {
            "name": "audioUrl",
            "type": "bytes"
          },
          {
            "name": "creator",
            "type": "pubkey"
          },
          {
            "name": "duration",
            "type": "u64"
          },
          {
            "name": "publishedAt",
            "type": "i64"
          },
          {
            "name": "updatedAt",
            "type": "i64"
          },
          {
            "name": "podcastChannelId",
            "type": "bytes"
          },
          {
            "name": "collection",
            "type": {
              "option": "pubkey"
            }
          },
          {
            "name": "isNft",
            "type": "bool"
          },
          {
            "name": "nftMint",
            "type": {
              "option": "pubkey"
            }
          },
          {
            "name": "nftMetadata",
            "type": {
              "option": "pubkey"
            }
          },
          {
            "name": "nftEdition",
            "type": {
              "option": "pubkey"
            }
          },
          {
            "name": "nftHolders",
            "type": {
              "vec": {
                "defined": {
                  "name": "nftHolder"
                }
              }
            }
          }
        ]
      }
    }
  ]
};
