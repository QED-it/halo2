window.BENCHMARK_DATA = {
  "lastUpdate": 1750057110401,
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
          "id": "301e7d82e1cc8dee43389d6602b13e8a7a57f601",
          "message": "Revert \"test error\"\n\nThis reverts commit 23a295172b6c8cb0b642512109801b5116d79c6a.",
          "timestamp": "2024-04-29T20:35:32+02:00",
          "tree_id": "76a058cdb3e6c6aee9ec313880b5924b830526c7",
          "url": "https://github.com/QED-it/halo2/commit/301e7d82e1cc8dee43389d6602b13e8a7a57f601"
        },
        "date": 1714418597452,
        "tool": "cargo",
        "benches": [
          {
            "name": "WIDTH = 3, RATE = 2-prover",
            "value": 71366030,
            "range": "± 4916106",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 3, RATE = 2-verifier",
            "value": 4045479,
            "range": "± 56532",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 9, RATE = 8-prover",
            "value": 136277318,
            "range": "± 3305187",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 9, RATE = 8-verifier",
            "value": 4559409,
            "range": "± 239101",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 12, RATE = 11-prover",
            "value": 187939680,
            "range": "± 1788537",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 12, RATE = 11-verifier",
            "value": 4996083,
            "range": "± 116581",
            "unit": "ns/iter"
          },
          {
            "name": "Poseidon/2-to-1",
            "value": 31148,
            "range": "± 105",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash-to-point/510",
            "value": 132960,
            "range": "± 854",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash/510",
            "value": 145990,
            "range": "± 555",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/commit/510",
            "value": 234913,
            "range": "± 466",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/short-commit/510",
            "value": 234921,
            "range": "± 365",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash-to-point/520",
            "value": 135658,
            "range": "± 312",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash/520",
            "value": 148646,
            "range": "± 148",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/commit/520",
            "value": 237523,
            "range": "± 737",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/short-commit/520",
            "value": 237556,
            "range": "± 560",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash-to-point/1086",
            "value": 284245,
            "range": "± 435",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash/1086",
            "value": 297295,
            "range": "± 12972",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/commit/1086",
            "value": 386037,
            "range": "± 505",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/short-commit/1086",
            "value": 386058,
            "range": "± 1150",
            "unit": "ns/iter"
          },
          {
            "name": "double-and-add",
            "value": 2909516,
            "range": "± 14830",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/14",
            "value": 4679145,
            "range": "± 4943",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/15",
            "value": 8068927,
            "range": "± 14947",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/16",
            "value": 17884363,
            "range": "± 26697",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/17",
            "value": 32593798,
            "range": "± 342708",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/18",
            "value": 62320248,
            "range": "± 164906",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/3",
            "value": 7019,
            "range": "± 1677",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/4",
            "value": 8750,
            "range": "± 991",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/5",
            "value": 13971,
            "range": "± 1131",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/6",
            "value": 18276,
            "range": "± 592",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/7",
            "value": 26906,
            "range": "± 1292",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/8",
            "value": 46321,
            "range": "± 2405",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/9",
            "value": 93064,
            "range": "± 1780",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/10",
            "value": 162888,
            "range": "± 2079",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/11",
            "value": 296538,
            "range": "± 7908",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/12",
            "value": 576327,
            "range": "± 13831",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/13",
            "value": 1157897,
            "range": "± 48453",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/14",
            "value": 2384918,
            "range": "± 79488",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/15",
            "value": 5107796,
            "range": "± 149167",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/16",
            "value": 10702313,
            "range": "± 137743",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/17",
            "value": 22287771,
            "range": "± 430965",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/18",
            "value": 51205022,
            "range": "± 2555639",
            "unit": "ns/iter"
          },
          {
            "name": "hash-to-curve/Pallas",
            "value": 29175,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "hash-to-curve/Vesta",
            "value": 29278,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/8",
            "value": 129150244,
            "range": "± 3107238",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/9",
            "value": 271153671,
            "range": "± 6219952",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/10",
            "value": 576703636,
            "range": "± 19944996",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/11",
            "value": 1223902803,
            "range": "± 35987134",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/12",
            "value": 2599175720,
            "range": "± 80771725",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/13",
            "value": 5479364044,
            "range": "± 102550487",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/14",
            "value": 11647442990,
            "range": "± 260450545",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/15",
            "value": 23804037672,
            "range": "± 504017672",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/16",
            "value": 51612380292,
            "range": "± 699443586",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/8",
            "value": 85153937,
            "range": "± 1723052",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/9",
            "value": 138166260,
            "range": "± 1139448",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/10",
            "value": 237526933,
            "range": "± 2033427",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/11",
            "value": 419683568,
            "range": "± 2362484",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/12",
            "value": 758258982,
            "range": "± 5233456",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/13",
            "value": 1409194767,
            "range": "± 4725618",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/14",
            "value": 2661364904,
            "range": "± 27231711",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/15",
            "value": 5055493355,
            "range": "± 24555547",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/16",
            "value": 9742863249,
            "range": "± 36399900",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/8",
            "value": 4585757,
            "range": "± 60559",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/9",
            "value": 6629509,
            "range": "± 267002",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/10",
            "value": 10600160,
            "range": "± 151204",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/11",
            "value": 17370544,
            "range": "± 298702",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/12",
            "value": 29962908,
            "range": "± 238668",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/13",
            "value": 51637255,
            "range": "± 515649",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/14",
            "value": 92179108,
            "range": "± 970621",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/15",
            "value": 166264627,
            "range": "± 4663570",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/16",
            "value": 309299128,
            "range": "± 1605880",
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
        "date": 1730127945053,
        "tool": "cargo",
        "benches": [
          {
            "name": "WIDTH = 3, RATE = 2-prover",
            "value": 71399890,
            "range": "± 2624817",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 3, RATE = 2-verifier",
            "value": 4046242,
            "range": "± 51028",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 9, RATE = 8-prover",
            "value": 136494442,
            "range": "± 3019655",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 9, RATE = 8-verifier",
            "value": 4538377,
            "range": "± 137269",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 12, RATE = 11-prover",
            "value": 188290212,
            "range": "± 2652373",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 12, RATE = 11-verifier",
            "value": 5050104,
            "range": "± 103101",
            "unit": "ns/iter"
          },
          {
            "name": "Poseidon/2-to-1",
            "value": 31155,
            "range": "± 163",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash-to-point/510",
            "value": 133445,
            "range": "± 319",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash/510",
            "value": 145812,
            "range": "± 1743",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/commit/510",
            "value": 235547,
            "range": "± 290",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/short-commit/510",
            "value": 235518,
            "range": "± 838",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash-to-point/520",
            "value": 136159,
            "range": "± 400",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash/520",
            "value": 148517,
            "range": "± 1153",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/commit/520",
            "value": 238155,
            "range": "± 750",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/short-commit/520",
            "value": 238184,
            "range": "± 612",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash-to-point/1086",
            "value": 284973,
            "range": "± 2661",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash/1086",
            "value": 296841,
            "range": "± 570",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/commit/1086",
            "value": 387055,
            "range": "± 498",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/short-commit/1086",
            "value": 387041,
            "range": "± 5933",
            "unit": "ns/iter"
          },
          {
            "name": "double-and-add",
            "value": 2908343,
            "range": "± 53953",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/14",
            "value": 4714296,
            "range": "± 9245",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/15",
            "value": 8158949,
            "range": "± 10826",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/16",
            "value": 18892989,
            "range": "± 91611",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/17",
            "value": 32208967,
            "range": "± 184334",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/18",
            "value": 59925363,
            "range": "± 166420",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/3",
            "value": 9117,
            "range": "± 2175",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/4",
            "value": 8864,
            "range": "± 1313",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/5",
            "value": 13968,
            "range": "± 1452",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/6",
            "value": 18429,
            "range": "± 1383",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/7",
            "value": 26896,
            "range": "± 1157",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/8",
            "value": 45311,
            "range": "± 1676",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/9",
            "value": 92086,
            "range": "± 1656",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/10",
            "value": 162907,
            "range": "± 1713",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/11",
            "value": 297293,
            "range": "± 4396",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/12",
            "value": 573629,
            "range": "± 25963",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/13",
            "value": 1155693,
            "range": "± 28417",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/14",
            "value": 2382804,
            "range": "± 24408",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/15",
            "value": 5059370,
            "range": "± 78220",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/16",
            "value": 10828141,
            "range": "± 161062",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/17",
            "value": 22884282,
            "range": "± 437380",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/18",
            "value": 53655270,
            "range": "± 1450744",
            "unit": "ns/iter"
          },
          {
            "name": "hash-to-curve/Pallas",
            "value": 29174,
            "range": "± 484",
            "unit": "ns/iter"
          },
          {
            "name": "hash-to-curve/Vesta",
            "value": 29270,
            "range": "± 89",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/8",
            "value": 131189263,
            "range": "± 3664031",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/9",
            "value": 277900408,
            "range": "± 6969346",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/10",
            "value": 576644315,
            "range": "± 12419066",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/11",
            "value": 1222852824,
            "range": "± 15430399",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/12",
            "value": 2591030856,
            "range": "± 83310239",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/13",
            "value": 5444815161,
            "range": "± 124838518",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/14",
            "value": 11610643527,
            "range": "± 188540092",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/15",
            "value": 23859015038,
            "range": "± 482554452",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/16",
            "value": 52086887996,
            "range": "± 929411194",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/8",
            "value": 85277077,
            "range": "± 1063277",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/9",
            "value": 138989608,
            "range": "± 937887",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/10",
            "value": 236390615,
            "range": "± 2373549",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/11",
            "value": 417694112,
            "range": "± 1900305",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/12",
            "value": 757915685,
            "range": "± 13542594",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/13",
            "value": 1411185671,
            "range": "± 6441492",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/14",
            "value": 2652467342,
            "range": "± 17768946",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/15",
            "value": 5043675719,
            "range": "± 21036858",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/16",
            "value": 9746168748,
            "range": "± 29551585",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/8",
            "value": 4573459,
            "range": "± 55043",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/9",
            "value": 6618200,
            "range": "± 194256",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/10",
            "value": 10622337,
            "range": "± 137719",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/11",
            "value": 17396408,
            "range": "± 373659",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/12",
            "value": 30009477,
            "range": "± 251653",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/13",
            "value": 51614535,
            "range": "± 484219",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/14",
            "value": 92756965,
            "range": "± 3933415",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/15",
            "value": 166238129,
            "range": "± 4109345",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/16",
            "value": 309331406,
            "range": "± 1475683",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "daira@jacaranda.org",
            "name": "Daira-Emma Hopwood",
            "username": "daira"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "7df93fd855395dcdb301a857d4b33f37903bbf76",
          "message": "Merge pull request #814 from adria0/fix/mdbook\n\nFix MD book generation",
          "timestamp": "2024-02-26T23:50:17Z",
          "tree_id": "ef67992c62cbe95d9ecf5d0fada00c9835333a5b",
          "url": "https://github.com/QED-it/halo2/commit/7df93fd855395dcdb301a857d4b33f37903bbf76"
        },
        "date": 1730128012044,
        "tool": "cargo",
        "benches": [
          {
            "name": "WIDTH = 3, RATE = 2-prover",
            "value": 71777813,
            "range": "± 4336954",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 3, RATE = 2-verifier",
            "value": 4041223,
            "range": "± 63274",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 9, RATE = 8-prover",
            "value": 136928677,
            "range": "± 3754621",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 9, RATE = 8-verifier",
            "value": 4588903,
            "range": "± 148103",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 12, RATE = 11-prover",
            "value": 188248237,
            "range": "± 1122785",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 12, RATE = 11-verifier",
            "value": 5009619,
            "range": "± 243825",
            "unit": "ns/iter"
          },
          {
            "name": "Poseidon/2-to-1",
            "value": 31208,
            "range": "± 1199",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash-to-point/510",
            "value": 133110,
            "range": "± 328",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash/510",
            "value": 146266,
            "range": "± 3411",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/commit/510",
            "value": 236918,
            "range": "± 1075",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/short-commit/510",
            "value": 237034,
            "range": "± 682",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash-to-point/520",
            "value": 135794,
            "range": "± 479",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash/520",
            "value": 149042,
            "range": "± 491",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/commit/520",
            "value": 239432,
            "range": "± 451",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/short-commit/520",
            "value": 239569,
            "range": "± 513",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash-to-point/1086",
            "value": 284274,
            "range": "± 734",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash/1086",
            "value": 297980,
            "range": "± 1842",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/commit/1086",
            "value": 388075,
            "range": "± 1103",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/short-commit/1086",
            "value": 388088,
            "range": "± 742",
            "unit": "ns/iter"
          },
          {
            "name": "double-and-add",
            "value": 2864248,
            "range": "± 11366",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/14",
            "value": 4718085,
            "range": "± 17119",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/15",
            "value": 8122318,
            "range": "± 33558",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/16",
            "value": 18347398,
            "range": "± 76099",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/17",
            "value": 33661840,
            "range": "± 72482",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/18",
            "value": 63730259,
            "range": "± 73130",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/3",
            "value": 9067,
            "range": "± 2571",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/4",
            "value": 9480,
            "range": "± 1753",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/5",
            "value": 14117,
            "range": "± 1730",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/6",
            "value": 18551,
            "range": "± 1031",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/7",
            "value": 26881,
            "range": "± 528",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/8",
            "value": 45227,
            "range": "± 3018",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/9",
            "value": 92690,
            "range": "± 1981",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/10",
            "value": 163306,
            "range": "± 1635",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/11",
            "value": 296048,
            "range": "± 4813",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/12",
            "value": 574748,
            "range": "± 16744",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/13",
            "value": 1163219,
            "range": "± 30260",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/14",
            "value": 2414758,
            "range": "± 35050",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/15",
            "value": 5078138,
            "range": "± 58638",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/16",
            "value": 10773440,
            "range": "± 205862",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/17",
            "value": 22546528,
            "range": "± 571511",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/18",
            "value": 50791835,
            "range": "± 1402436",
            "unit": "ns/iter"
          },
          {
            "name": "hash-to-curve/Pallas",
            "value": 29190,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "hash-to-curve/Vesta",
            "value": 29264,
            "range": "± 103",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/8",
            "value": 130601256,
            "range": "± 3585988",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/9",
            "value": 273918042,
            "range": "± 5576510",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/10",
            "value": 566014494,
            "range": "± 16491894",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/11",
            "value": 1215203403,
            "range": "± 30338899",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/12",
            "value": 2585750027,
            "range": "± 72351828",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/13",
            "value": 5502425149,
            "range": "± 122620479",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/14",
            "value": 11579280365,
            "range": "± 224561478",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/15",
            "value": 24572038692,
            "range": "± 494166084",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/16",
            "value": 51543271588,
            "range": "± 890116620",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/8",
            "value": 85575213,
            "range": "± 635447",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/9",
            "value": 139088666,
            "range": "± 1814357",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/10",
            "value": 237525945,
            "range": "± 2421478",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/11",
            "value": 418608989,
            "range": "± 2904674",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/12",
            "value": 761840451,
            "range": "± 4387076",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/13",
            "value": 1414447583,
            "range": "± 12678895",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/14",
            "value": 2659575817,
            "range": "± 3970064",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/15",
            "value": 5055362669,
            "range": "± 14312688",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/16",
            "value": 9781077848,
            "range": "± 34247128",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/8",
            "value": 4589986,
            "range": "± 105534",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/9",
            "value": 6644189,
            "range": "± 307084",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/10",
            "value": 10624870,
            "range": "± 250078",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/11",
            "value": 17499873,
            "range": "± 345437",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/12",
            "value": 30032646,
            "range": "± 1644471",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/13",
            "value": 51801461,
            "range": "± 860092",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/14",
            "value": 92433820,
            "range": "± 820124",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/15",
            "value": 166192660,
            "range": "± 1198129",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/16",
            "value": 309086625,
            "range": "± 3328468",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "3682187+PaulLaux@users.noreply.github.com",
            "name": "Paul",
            "username": "PaulLaux"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c385bb5fec32aeb783ee40b8b286546ca09254e6",
          "message": "Merge pull request #37 from zcash/main\n\nupstream update",
          "timestamp": "2025-01-27T16:47:41+02:00",
          "tree_id": "052371a115a6f3a4d91bcbdf00fa4ee5680d4ee7",
          "url": "https://github.com/QED-it/halo2/commit/c385bb5fec32aeb783ee40b8b286546ca09254e6"
        },
        "date": 1737989800858,
        "tool": "cargo",
        "benches": [
          {
            "name": "WIDTH = 3, RATE = 2-prover",
            "value": 71954687,
            "range": "± 5685430",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 3, RATE = 2-verifier",
            "value": 4108172,
            "range": "± 295679",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 9, RATE = 8-prover",
            "value": 137238516,
            "range": "± 3090575",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 9, RATE = 8-verifier",
            "value": 4630713,
            "range": "± 109667",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 12, RATE = 11-prover",
            "value": 189395120,
            "range": "± 957892",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 12, RATE = 11-verifier",
            "value": 5072850,
            "range": "± 46971",
            "unit": "ns/iter"
          },
          {
            "name": "Poseidon/2-to-1",
            "value": 29939,
            "range": "± 64",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash-to-point/510",
            "value": 134312,
            "range": "± 719",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash/510",
            "value": 146856,
            "range": "± 753",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/commit/510",
            "value": 238592,
            "range": "± 373",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/short-commit/510",
            "value": 238596,
            "range": "± 807",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash-to-point/520",
            "value": 137071,
            "range": "± 345",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash/520",
            "value": 149563,
            "range": "± 772",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/commit/520",
            "value": 241249,
            "range": "± 546",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/short-commit/520",
            "value": 241258,
            "range": "± 571",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash-to-point/1086",
            "value": 287165,
            "range": "± 831",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash/1086",
            "value": 298928,
            "range": "± 1227",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/commit/1086",
            "value": 391066,
            "range": "± 1495",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/short-commit/1086",
            "value": 390974,
            "range": "± 697",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "jack@electriccoin.co",
            "name": "Jack Grigg",
            "username": "str4d"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "fed6b000857f27e23ddb07454da8bde4697204f7",
          "message": "Merge pull request #835 from zcash/halo2_poseidon-0.1.0\n\nhalo2_poseidon 0.1.0",
          "timestamp": "2024-12-17T08:02:56+13:00",
          "tree_id": "052371a115a6f3a4d91bcbdf00fa4ee5680d4ee7",
          "url": "https://github.com/QED-it/halo2/commit/fed6b000857f27e23ddb07454da8bde4697204f7"
        },
        "date": 1737989988788,
        "tool": "cargo",
        "benches": [
          {
            "name": "WIDTH = 3, RATE = 2-prover",
            "value": 72268276,
            "range": "± 5592755",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 3, RATE = 2-verifier",
            "value": 4111448,
            "range": "± 53363",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 9, RATE = 8-prover",
            "value": 137436025,
            "range": "± 3237734",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 9, RATE = 8-verifier",
            "value": 4621871,
            "range": "± 117691",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 12, RATE = 11-prover",
            "value": 189413264,
            "range": "± 1596953",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 12, RATE = 11-verifier",
            "value": 5043284,
            "range": "± 45126",
            "unit": "ns/iter"
          },
          {
            "name": "Poseidon/2-to-1",
            "value": 29930,
            "range": "± 665",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash-to-point/510",
            "value": 134019,
            "range": "± 597",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash/510",
            "value": 146559,
            "range": "± 293",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/commit/510",
            "value": 237043,
            "range": "± 1285",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/short-commit/510",
            "value": 237104,
            "range": "± 664",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash-to-point/520",
            "value": 136962,
            "range": "± 2255",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash/520",
            "value": 149267,
            "range": "± 236",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/commit/520",
            "value": 239700,
            "range": "± 2793",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/short-commit/520",
            "value": 239683,
            "range": "± 976",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash-to-point/1086",
            "value": 286957,
            "range": "± 4783",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash/1086",
            "value": 298304,
            "range": "± 842",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/commit/1086",
            "value": 389313,
            "range": "± 4773",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/short-commit/1086",
            "value": 389320,
            "range": "± 1233",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "jack@electriccoin.co",
            "name": "Jack Grigg",
            "username": "str4d"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "8056703404299dd0a1e381ecfaa780f891dfc392",
          "message": "Merge pull request #837 from wowinter13/feat/improve_mock_provider_docs\n\ndocs(halo2_proofs): Improve MockProver::verify description",
          "timestamp": "2025-06-12T13:30:17+01:00",
          "tree_id": "f0a93f27e83d1cabb9886358e96935f4aeeb8f59",
          "url": "https://github.com/QED-it/halo2/commit/8056703404299dd0a1e381ecfaa780f891dfc392"
        },
        "date": 1750057108918,
        "tool": "cargo",
        "benches": [
          {
            "name": "WIDTH = 3, RATE = 2-prover",
            "value": 71849836,
            "range": "± 738190",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 3, RATE = 2-verifier",
            "value": 4074842,
            "range": "± 45767",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 9, RATE = 8-prover",
            "value": 137224772,
            "range": "± 2678030",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 9, RATE = 8-verifier",
            "value": 4545375,
            "range": "± 96433",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 12, RATE = 11-prover",
            "value": 189506914,
            "range": "± 2287357",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 12, RATE = 11-verifier",
            "value": 5013789,
            "range": "± 61261",
            "unit": "ns/iter"
          },
          {
            "name": "Poseidon/2-to-1",
            "value": 29940,
            "range": "± 79",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash-to-point/510",
            "value": 134149,
            "range": "± 383",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash/510",
            "value": 146598,
            "range": "± 598",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/commit/510",
            "value": 237739,
            "range": "± 929",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/short-commit/510",
            "value": 237826,
            "range": "± 331",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash-to-point/520",
            "value": 136922,
            "range": "± 274",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash/520",
            "value": 149402,
            "range": "± 219",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/commit/520",
            "value": 240490,
            "range": "± 305",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/short-commit/520",
            "value": 240496,
            "range": "± 517",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash-to-point/1086",
            "value": 286656,
            "range": "± 897",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash/1086",
            "value": 298384,
            "range": "± 855",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/commit/1086",
            "value": 390240,
            "range": "± 492",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/short-commit/1086",
            "value": 390392,
            "range": "± 592",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}