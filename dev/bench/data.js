window.BENCHMARK_DATA = {
  "lastUpdate": 1681919565128,
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
      }
    ]
  }
}