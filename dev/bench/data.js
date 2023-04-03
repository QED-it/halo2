window.BENCHMARK_DATA = {
  "lastUpdate": 1680516581094,
  "repoUrl": "https://github.com/QED-it/halo2",
  "entries": {
    "halo2 Benchmark": [
      {
        "commit": {
          "author": {
            "email": "ewillbefull@gmail.com",
            "name": "ebfull",
            "username": "ebfull"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "263356784042d7d4c1c17d357c94c1acaeb75ab5",
          "message": "Merge pull request #758 from zcash/release-0.3.0\n\nRelease 0.3.0 of halo2_proofs and halo2_gadgets",
          "timestamp": "2023-03-22T12:32:26-06:00",
          "tree_id": "16087d2e72223f8ad75a504c7125d88b237fc70a",
          "url": "https://github.com/QED-it/halo2/commit/263356784042d7d4c1c17d357c94c1acaeb75ab5"
        },
        "date": 1680516578256,
        "tool": "cargo",
        "benches": [
          {
            "name": "WIDTH = 3, RATE = 2-prover",
            "value": 116397578,
            "range": "± 10736318",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 3, RATE = 2-verifier",
            "value": 6715504,
            "range": "± 752323",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 9, RATE = 8-prover",
            "value": 221787487,
            "range": "± 9502693",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 9, RATE = 8-verifier",
            "value": 7986003,
            "range": "± 920367",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 12, RATE = 11-prover",
            "value": 301518578,
            "range": "± 10439690",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 12, RATE = 11-verifier",
            "value": 8395267,
            "range": "± 1061923",
            "unit": "ns/iter"
          },
          {
            "name": "Poseidon/2-to-1",
            "value": 54003,
            "range": "± 1686",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash-to-point/510",
            "value": 186562,
            "range": "± 5275",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash/510",
            "value": 201438,
            "range": "± 7108",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/commit/510",
            "value": 330750,
            "range": "± 12568",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/short-commit/510",
            "value": 333184,
            "range": "± 10191",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash-to-point/520",
            "value": 192021,
            "range": "± 5758",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash/520",
            "value": 204948,
            "range": "± 5381",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/commit/520",
            "value": 338520,
            "range": "± 10024",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/short-commit/520",
            "value": 335854,
            "range": "± 12535",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash-to-point/1086",
            "value": 396627,
            "range": "± 13025",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash/1086",
            "value": 418852,
            "range": "± 15234",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/commit/1086",
            "value": 519210,
            "range": "± 18207",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/short-commit/1086",
            "value": 543347,
            "range": "± 18513",
            "unit": "ns/iter"
          },
          {
            "name": "double-and-add",
            "value": 3954350,
            "range": "± 151683",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/14",
            "value": 8090106,
            "range": "± 124131",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/15",
            "value": 14170321,
            "range": "± 342600",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/16",
            "value": 31795300,
            "range": "± 586091",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/17",
            "value": 59844638,
            "range": "± 651794",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/18",
            "value": 114600461,
            "range": "± 1543663",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/3",
            "value": 9190,
            "range": "± 729",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/4",
            "value": 11000,
            "range": "± 959",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/5",
            "value": 20503,
            "range": "± 1836",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/6",
            "value": 26004,
            "range": "± 1176",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/7",
            "value": 35222,
            "range": "± 5238",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/8",
            "value": 59386,
            "range": "± 9601",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/9",
            "value": 141424,
            "range": "± 29079",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/10",
            "value": 291040,
            "range": "± 47319",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/11",
            "value": 586612,
            "range": "± 87648",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/12",
            "value": 1077767,
            "range": "± 119072",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/13",
            "value": 2148683,
            "range": "± 164577",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/14",
            "value": 4845046,
            "range": "± 540878",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/15",
            "value": 10196041,
            "range": "± 1208793",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/16",
            "value": 22732831,
            "range": "± 2746223",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/17",
            "value": 46515266,
            "range": "± 2593564",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/18",
            "value": 100831798,
            "range": "± 3814961",
            "unit": "ns/iter"
          },
          {
            "name": "hash-to-curve/Pallas",
            "value": 36862,
            "range": "± 1159",
            "unit": "ns/iter"
          },
          {
            "name": "hash-to-curve/Vesta",
            "value": 37258,
            "range": "± 1164",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/8",
            "value": 205377932,
            "range": "± 2613369",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/9",
            "value": 449541941,
            "range": "± 7573773",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/10",
            "value": 961462684,
            "range": "± 15435996",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/11",
            "value": 2059731707,
            "range": "± 24028077",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/12",
            "value": 4405054966,
            "range": "± 61944050",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/13",
            "value": 9385122383,
            "range": "± 367719917",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/14",
            "value": 19819707250,
            "range": "± 74798523",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/15",
            "value": 42095762079,
            "range": "± 187913393",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/16",
            "value": 89125426511,
            "range": "± 265398964",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/8",
            "value": 142537601,
            "range": "± 4741639",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/9",
            "value": 246228362,
            "range": "± 5194468",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/10",
            "value": 422054672,
            "range": "± 7065838",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/11",
            "value": 750530841,
            "range": "± 17158796",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/12",
            "value": 1362722681,
            "range": "± 18331844",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/13",
            "value": 2573827852,
            "range": "± 29111218",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/14",
            "value": 4775344791,
            "range": "± 51223058",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/15",
            "value": 9194069227,
            "range": "± 111998799",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/16",
            "value": 17982442353,
            "range": "± 123113009",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/8",
            "value": 7069230,
            "range": "± 620187",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/9",
            "value": 10903019,
            "range": "± 1187387",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/10",
            "value": 17747218,
            "range": "± 1967114",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/11",
            "value": 29480902,
            "range": "± 2570403",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/12",
            "value": 51196546,
            "range": "± 4873291",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/13",
            "value": 91447274,
            "range": "± 7265070",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/14",
            "value": 162637042,
            "range": "± 12384974",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/15",
            "value": 304583252,
            "range": "± 12205718",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/16",
            "value": 559265174,
            "range": "± 16544794",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}