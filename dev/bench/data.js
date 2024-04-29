window.BENCHMARK_DATA = {
  "lastUpdate": 1714417737901,
  "repoUrl": "https://github.com/QED-it/halo2",
  "entries": {
    "halo2 Benchmark": [
      {
        "commit": {
          "author": {
            "email": "daira@jacaranda.org",
            "name": "Daira Hopwood",
            "username": "daira"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5678a506cbec60d593a21ff618d09bed48abf7f2",
          "message": "Merge pull request #755 from zcash/lookup-errors\n\n`plonk::Error::TableError`: Better lookup errors",
          "timestamp": "2023-04-04T23:27:28Z",
          "tree_id": "e656b9132260d133ef8fc73654d012ae00242ca4",
          "url": "https://github.com/QED-it/halo2/commit/5678a506cbec60d593a21ff618d09bed48abf7f2"
        },
        "date": 1681919562348,
        "tool": "cargo",
        "benches": [
          {
            "name": "WIDTH = 3, RATE = 2-prover",
            "value": 102530307,
            "range": "± 12017176",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 3, RATE = 2-verifier",
            "value": 6184287,
            "range": "± 674564",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 9, RATE = 8-prover",
            "value": 200434661,
            "range": "± 10902406",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 9, RATE = 8-verifier",
            "value": 6529711,
            "range": "± 831382",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 12, RATE = 11-prover",
            "value": 235458498,
            "range": "± 17504075",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 12, RATE = 11-verifier",
            "value": 6752270,
            "range": "± 831358",
            "unit": "ns/iter"
          },
          {
            "name": "Poseidon/2-to-1",
            "value": 44305,
            "range": "± 3650",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash-to-point/510",
            "value": 141553,
            "range": "± 11998",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash/510",
            "value": 144677,
            "range": "± 8769",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/commit/510",
            "value": 242447,
            "range": "± 14591",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/short-commit/510",
            "value": 300173,
            "range": "± 19946",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash-to-point/520",
            "value": 167973,
            "range": "± 13230",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash/520",
            "value": 169574,
            "range": "± 11857",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/commit/520",
            "value": 293756,
            "range": "± 21613",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/short-commit/520",
            "value": 261103,
            "range": "± 19164",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash-to-point/1086",
            "value": 318027,
            "range": "± 23883",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash/1086",
            "value": 351659,
            "range": "± 32020",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/commit/1086",
            "value": 462346,
            "range": "± 36182",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/short-commit/1086",
            "value": 490311,
            "range": "± 31566",
            "unit": "ns/iter"
          },
          {
            "name": "double-and-add",
            "value": 3859689,
            "range": "± 182764",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/14",
            "value": 7431580,
            "range": "± 149267",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/15",
            "value": 10552940,
            "range": "± 478568",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/16",
            "value": 23235293,
            "range": "± 1282280",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/17",
            "value": 47749558,
            "range": "± 4073018",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/18",
            "value": 91394166,
            "range": "± 3133841",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/3",
            "value": 7793,
            "range": "± 908",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/4",
            "value": 9364,
            "range": "± 1270",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/5",
            "value": 16768,
            "range": "± 1529",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/6",
            "value": 23051,
            "range": "± 5217",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/7",
            "value": 28987,
            "range": "± 4120",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/8",
            "value": 50053,
            "range": "± 9674",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/9",
            "value": 117442,
            "range": "± 21538",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/10",
            "value": 238467,
            "range": "± 92417",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/11",
            "value": 538447,
            "range": "± 85674",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/12",
            "value": 931693,
            "range": "± 118203",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/13",
            "value": 1905070,
            "range": "± 183762",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/14",
            "value": 3971006,
            "range": "± 469492",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/15",
            "value": 8789798,
            "range": "± 1068055",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/16",
            "value": 19590127,
            "range": "± 2978434",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/17",
            "value": 40466620,
            "range": "± 3412143",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/18",
            "value": 79576837,
            "range": "± 6421537",
            "unit": "ns/iter"
          },
          {
            "name": "hash-to-curve/Pallas",
            "value": 27759,
            "range": "± 1754",
            "unit": "ns/iter"
          },
          {
            "name": "hash-to-curve/Vesta",
            "value": 27880,
            "range": "± 1954",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/8",
            "value": 156095440,
            "range": "± 5477419",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/9",
            "value": 338769292,
            "range": "± 18133065",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/10",
            "value": 716900254,
            "range": "± 24216585",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/11",
            "value": 1568790302,
            "range": "± 33566979",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/12",
            "value": 3431448564,
            "range": "± 134307444",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/13",
            "value": 7121831869,
            "range": "± 197283186",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/14",
            "value": 14899710599,
            "range": "± 181724814",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/15",
            "value": 31482069452,
            "range": "± 592477741",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/16",
            "value": 67066063343,
            "range": "± 1756446092",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/8",
            "value": 107419536,
            "range": "± 7408924",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/9",
            "value": 182159779,
            "range": "± 11958415",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/10",
            "value": 315467628,
            "range": "± 19063164",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/11",
            "value": 552087470,
            "range": "± 23662820",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/12",
            "value": 1004979113,
            "range": "± 38699367",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/13",
            "value": 1916210125,
            "range": "± 47534171",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/14",
            "value": 3597828888,
            "range": "± 77498257",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/15",
            "value": 7021187255,
            "range": "± 200114626",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/16",
            "value": 13411947861,
            "range": "± 155261381",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/8",
            "value": 5110319,
            "range": "± 511600",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/9",
            "value": 7813245,
            "range": "± 732996",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/10",
            "value": 12690551,
            "range": "± 1195784",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/11",
            "value": 21811081,
            "range": "± 2013502",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/12",
            "value": 36776106,
            "range": "± 2365100",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/13",
            "value": 66888127,
            "range": "± 6341413",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/14",
            "value": 128694532,
            "range": "± 9887158",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/15",
            "value": 232420534,
            "range": "± 16938555",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/16",
            "value": 411389425,
            "range": "± 21892983",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "daira@jacaranda.org",
            "name": "Daira Emma Hopwood",
            "username": "daira"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "7fd2ce259ec3d0b0e3ede3fa20e4cdcfc395efc9",
          "message": "Merge pull request #805 from zcash/check-in-lockfile\n\nAdd `Cargo.lock` to repository",
          "timestamp": "2023-11-29T21:52:01Z",
          "tree_id": "76a058cdb3e6c6aee9ec313880b5924b830526c7",
          "url": "https://github.com/QED-it/halo2/commit/7fd2ce259ec3d0b0e3ede3fa20e4cdcfc395efc9"
        },
        "date": 1701788769428,
        "tool": "cargo",
        "benches": [
          {
            "name": "WIDTH = 3, RATE = 2-prover",
            "value": 71554509,
            "range": "± 5683988",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 3, RATE = 2-verifier",
            "value": 4016453,
            "range": "± 70244",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 9, RATE = 8-prover",
            "value": 136480116,
            "range": "± 3735791",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 9, RATE = 8-verifier",
            "value": 4635638,
            "range": "± 151052",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 12, RATE = 11-prover",
            "value": 188086305,
            "range": "± 2940299",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 12, RATE = 11-verifier",
            "value": 5018719,
            "range": "± 127332",
            "unit": "ns/iter"
          },
          {
            "name": "Poseidon/2-to-1",
            "value": 31171,
            "range": "± 665",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash-to-point/510",
            "value": 133476,
            "range": "± 318",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash/510",
            "value": 146321,
            "range": "± 341",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/commit/510",
            "value": 239503,
            "range": "± 363",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/short-commit/510",
            "value": 239512,
            "range": "± 336",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash-to-point/520",
            "value": 136170,
            "range": "± 648",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash/520",
            "value": 149097,
            "range": "± 315",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/commit/520",
            "value": 242182,
            "range": "± 2150",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/short-commit/520",
            "value": 242079,
            "range": "± 549",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash-to-point/1086",
            "value": 285181,
            "range": "± 724",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash/1086",
            "value": 297928,
            "range": "± 3291",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/commit/1086",
            "value": 391002,
            "range": "± 1299",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/short-commit/1086",
            "value": 390985,
            "range": "± 1806",
            "unit": "ns/iter"
          },
          {
            "name": "double-and-add",
            "value": 2671513,
            "range": "± 15710",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/14",
            "value": 4706063,
            "range": "± 21011",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/15",
            "value": 8132260,
            "range": "± 19925",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/16",
            "value": 18097246,
            "range": "± 96464",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/17",
            "value": 32559519,
            "range": "± 202259",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/18",
            "value": 62628158,
            "range": "± 157020",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/3",
            "value": 7133,
            "range": "± 1594",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/4",
            "value": 8680,
            "range": "± 983",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/5",
            "value": 14305,
            "range": "± 480",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/6",
            "value": 18689,
            "range": "± 1627",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/7",
            "value": 26890,
            "range": "± 835",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/8",
            "value": 43737,
            "range": "± 986",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/9",
            "value": 91767,
            "range": "± 1687",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/10",
            "value": 162508,
            "range": "± 1374",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/11",
            "value": 295912,
            "range": "± 11602",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/12",
            "value": 574521,
            "range": "± 16381",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/13",
            "value": 1157769,
            "range": "± 35471",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/14",
            "value": 2387084,
            "range": "± 56075",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/15",
            "value": 5081461,
            "range": "± 163225",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/16",
            "value": 10741818,
            "range": "± 174427",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/17",
            "value": 22390870,
            "range": "± 432770",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/18",
            "value": 48936862,
            "range": "± 1290518",
            "unit": "ns/iter"
          },
          {
            "name": "hash-to-curve/Pallas",
            "value": 29174,
            "range": "± 121",
            "unit": "ns/iter"
          },
          {
            "name": "hash-to-curve/Vesta",
            "value": 29267,
            "range": "± 488",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/8",
            "value": 130550610,
            "range": "± 3485733",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/9",
            "value": 274983636,
            "range": "± 6156639",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/10",
            "value": 564688693,
            "range": "± 22064351",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/11",
            "value": 1223935008,
            "range": "± 24096852",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/12",
            "value": 2590834383,
            "range": "± 59472509",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/13",
            "value": 5501584405,
            "range": "± 120078028",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/14",
            "value": 11502639710,
            "range": "± 338896687",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/15",
            "value": 24628622830,
            "range": "± 624537912",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/16",
            "value": 51751333558,
            "range": "± 1079497053",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/8",
            "value": 85451257,
            "range": "± 1163310",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/9",
            "value": 139183195,
            "range": "± 1885062",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/10",
            "value": 238076910,
            "range": "± 2392943",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/11",
            "value": 419554827,
            "range": "± 7296810",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/12",
            "value": 761904673,
            "range": "± 16990887",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/13",
            "value": 1413082948,
            "range": "± 16333538",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/14",
            "value": 2661543693,
            "range": "± 28577354",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/15",
            "value": 5057113148,
            "range": "± 32421811",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/16",
            "value": 9803084143,
            "range": "± 40828108",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/8",
            "value": 4556327,
            "range": "± 71381",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/9",
            "value": 6624719,
            "range": "± 486512",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/10",
            "value": 10621726,
            "range": "± 197893",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/11",
            "value": 17445568,
            "range": "± 493835",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/12",
            "value": 29953936,
            "range": "± 938583",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/13",
            "value": 51906832,
            "range": "± 3973174",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/14",
            "value": 92448749,
            "range": "± 4489070",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/15",
            "value": 166273425,
            "range": "± 7162113",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/16",
            "value": 309095589,
            "range": "± 13082059",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "yaojgalteland@yao.home",
            "name": "YaoGalteland"
          },
          "committer": {
            "email": "yaojgalteland@yao.home",
            "name": "YaoGalteland"
          },
          "distinct": true,
          "id": "23a295172b6c8cb0b642512109801b5116d79c6a",
          "message": "test error",
          "timestamp": "2024-04-29T20:22:09+02:00",
          "tree_id": "0a8b1144fee957a494808cbd19872120040f916e",
          "url": "https://github.com/QED-it/halo2/commit/23a295172b6c8cb0b642512109801b5116d79c6a"
        },
        "date": 1714417735902,
        "tool": "cargo",
        "benches": [
          {
            "name": "WIDTH = 3, RATE = 2-prover",
            "value": 71412657,
            "range": "± 4284721",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 3, RATE = 2-verifier",
            "value": 4036460,
            "range": "± 47043",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 9, RATE = 8-prover",
            "value": 136464174,
            "range": "± 3534882",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 9, RATE = 8-verifier",
            "value": 4562263,
            "range": "± 251650",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 12, RATE = 11-prover",
            "value": 187965884,
            "range": "± 1190466",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 12, RATE = 11-verifier",
            "value": 5000559,
            "range": "± 101461",
            "unit": "ns/iter"
          },
          {
            "name": "Poseidon/2-to-1",
            "value": 31141,
            "range": "± 679",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash-to-point/510",
            "value": 133002,
            "range": "± 197",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash/510",
            "value": 145987,
            "range": "± 432",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/commit/510",
            "value": 236355,
            "range": "± 387",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/short-commit/510",
            "value": 236383,
            "range": "± 294",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash-to-point/520",
            "value": 135684,
            "range": "± 1197",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash/520",
            "value": 148692,
            "range": "± 519",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/commit/520",
            "value": 239024,
            "range": "± 501",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/short-commit/520",
            "value": 239090,
            "range": "± 266",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash-to-point/1086",
            "value": 284205,
            "range": "± 3137",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash/1086",
            "value": 297197,
            "range": "± 948",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/commit/1086",
            "value": 387509,
            "range": "± 813",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/short-commit/1086",
            "value": 387627,
            "range": "± 1013",
            "unit": "ns/iter"
          },
          {
            "name": "double-and-add",
            "value": 2912792,
            "range": "± 25279",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/14",
            "value": 4719156,
            "range": "± 104268",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/15",
            "value": 8457428,
            "range": "± 45922",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/16",
            "value": 18119142,
            "range": "± 99417",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/17",
            "value": 32273936,
            "range": "± 717971",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/18",
            "value": 63152971,
            "range": "± 113837",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/3",
            "value": 8743,
            "range": "± 2251",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/4",
            "value": 9036,
            "range": "± 1643",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/5",
            "value": 14075,
            "range": "± 1925",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/6",
            "value": 18456,
            "range": "± 448",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/7",
            "value": 26900,
            "range": "± 2141",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/8",
            "value": 46133,
            "range": "± 3583",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/9",
            "value": 93225,
            "range": "± 1807",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/10",
            "value": 163753,
            "range": "± 1476",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/11",
            "value": 296984,
            "range": "± 7174",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/12",
            "value": 576445,
            "range": "± 6953",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/13",
            "value": 1146603,
            "range": "± 36914",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/14",
            "value": 2370591,
            "range": "± 27321",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/15",
            "value": 5021351,
            "range": "± 66724",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/16",
            "value": 10865097,
            "range": "± 168220",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/17",
            "value": 22430094,
            "range": "± 426760",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/18",
            "value": 47880418,
            "range": "± 1105248",
            "unit": "ns/iter"
          },
          {
            "name": "hash-to-curve/Pallas",
            "value": 29196,
            "range": "± 48",
            "unit": "ns/iter"
          },
          {
            "name": "hash-to-curve/Vesta",
            "value": 29274,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/8",
            "value": 129429926,
            "range": "± 2682735",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/9",
            "value": 270998057,
            "range": "± 4040648",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/10",
            "value": 596166838,
            "range": "± 22173628",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/11",
            "value": 1230945731,
            "range": "± 28698883",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/12",
            "value": 2571373568,
            "range": "± 63385513",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/13",
            "value": 5486987802,
            "range": "± 129784400",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/14",
            "value": 11646262908,
            "range": "± 207977874",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/15",
            "value": 24449960126,
            "range": "± 367299239",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/16",
            "value": 51683486090,
            "range": "± 550704493",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/8",
            "value": 84879823,
            "range": "± 1047897",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/9",
            "value": 139371315,
            "range": "± 1510833",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/10",
            "value": 236257377,
            "range": "± 1929137",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/11",
            "value": 419602524,
            "range": "± 9762791",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/12",
            "value": 758593173,
            "range": "± 2000220",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/13",
            "value": 1411593943,
            "range": "± 2569184",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/14",
            "value": 2656077473,
            "range": "± 5491886",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/15",
            "value": 5044801060,
            "range": "± 19190277",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/16",
            "value": 9758976867,
            "range": "± 19825358",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/8",
            "value": 4543519,
            "range": "± 86396",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/9",
            "value": 6651771,
            "range": "± 235508",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/10",
            "value": 10611920,
            "range": "± 179605",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/11",
            "value": 17360004,
            "range": "± 210419",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/12",
            "value": 29917693,
            "range": "± 313104",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/13",
            "value": 51492894,
            "range": "± 609035",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/14",
            "value": 92164779,
            "range": "± 842277",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/15",
            "value": 166381466,
            "range": "± 1280735",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/16",
            "value": 309749164,
            "range": "± 1471205",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}