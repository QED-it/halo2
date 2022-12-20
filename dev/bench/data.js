window.BENCHMARK_DATA = {
  "lastUpdate": 1671536041204,
  "repoUrl": "https://github.com/QED-it/halo2",
  "entries": {
    "halo2 Benchmark": [
      {
        "commit": {
          "author": {
            "email": "jack@electriccoin.co",
            "name": "str4d",
            "username": "str4d"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "7239a02ea34008b8973d742b3e918bbfbc01b75e",
          "message": "Merge pull request #701 from zcash/ff-0.13\n\nMigrate to published `ff 0.13`",
          "timestamp": "2022-12-07T10:21:33Z",
          "tree_id": "990a399f5ef1f3a10a716fd6523b240f8547a695",
          "url": "https://github.com/QED-it/halo2/commit/7239a02ea34008b8973d742b3e918bbfbc01b75e"
        },
        "date": 1671535913507,
        "tool": "cargo",
        "benches": [
          {
            "name": "WIDTH = 3, RATE = 2-prover",
            "value": 74425363,
            "range": "± 6444362",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 3, RATE = 2-verifier",
            "value": 3689057,
            "range": "± 412382",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 9, RATE = 8-prover",
            "value": 156494236,
            "range": "± 3206448",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 9, RATE = 8-verifier",
            "value": 4913444,
            "range": "± 565069",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 12, RATE = 11-prover",
            "value": 219747721,
            "range": "± 7883565",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 12, RATE = 11-verifier",
            "value": 6066770,
            "range": "± 595948",
            "unit": "ns/iter"
          },
          {
            "name": "Poseidon/2-to-1",
            "value": 45188,
            "range": "± 658",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash-to-point/510",
            "value": 162374,
            "range": "± 3153",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash/510",
            "value": 176390,
            "range": "± 3159",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/commit/510",
            "value": 281015,
            "range": "± 5496",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/short-commit/510",
            "value": 290291,
            "range": "± 3596",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash-to-point/520",
            "value": 168912,
            "range": "± 2719",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash/520",
            "value": 183132,
            "range": "± 2938",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/commit/520",
            "value": 290608,
            "range": "± 5252",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/short-commit/520",
            "value": 288528,
            "range": "± 4464",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash-to-point/1086",
            "value": 359731,
            "range": "± 5278",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash/1086",
            "value": 373284,
            "range": "± 5237",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/commit/1086",
            "value": 477390,
            "range": "± 7506",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/short-commit/1086",
            "value": 474554,
            "range": "± 6579",
            "unit": "ns/iter"
          },
          {
            "name": "double-and-add",
            "value": 3538136,
            "range": "± 52767",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/14",
            "value": 6774700,
            "range": "± 76111",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/15",
            "value": 12138306,
            "range": "± 80618",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/16",
            "value": 25768903,
            "range": "± 277700",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/17",
            "value": 48198753,
            "range": "± 619312",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/18",
            "value": 95266308,
            "range": "± 454672",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/3",
            "value": 8842,
            "range": "± 558",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/4",
            "value": 10129,
            "range": "± 605",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/5",
            "value": 17723,
            "range": "± 555",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/6",
            "value": 21381,
            "range": "± 1046",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/7",
            "value": 30751,
            "range": "± 3373",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/8",
            "value": 49887,
            "range": "± 7116",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/9",
            "value": 114497,
            "range": "± 18366",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/10",
            "value": 243256,
            "range": "± 44861",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/11",
            "value": 525936,
            "range": "± 87972",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/12",
            "value": 920510,
            "range": "± 121134",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/13",
            "value": 1866723,
            "range": "± 193216",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/14",
            "value": 4052678,
            "range": "± 384158",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/15",
            "value": 8339629,
            "range": "± 346569",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/16",
            "value": 18548121,
            "range": "± 574818",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/17",
            "value": 40842399,
            "range": "± 1474348",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/18",
            "value": 90784484,
            "range": "± 3033615",
            "unit": "ns/iter"
          },
          {
            "name": "hash-to-curve/Pallas",
            "value": 33729,
            "range": "± 4581",
            "unit": "ns/iter"
          },
          {
            "name": "hash-to-curve/Vesta",
            "value": 32868,
            "range": "± 512",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/8",
            "value": 181473407,
            "range": "± 14298902",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/9",
            "value": 396324977,
            "range": "± 9420843",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/10",
            "value": 833878755,
            "range": "± 12537737",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/11",
            "value": 1848680417,
            "range": "± 33213386",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/12",
            "value": 3878266419,
            "range": "± 35201521",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/13",
            "value": 8179714940,
            "range": "± 37951314",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/14",
            "value": 17574799010,
            "range": "± 263101973",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/15",
            "value": 37010653946,
            "range": "± 470231970",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/16",
            "value": 78477941389,
            "range": "± 1274802662",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/8",
            "value": 121150700,
            "range": "± 4294520",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/9",
            "value": 207118041,
            "range": "± 6113895",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/10",
            "value": 360896093,
            "range": "± 6727058",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/11",
            "value": 646074276,
            "range": "± 11129211",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/12",
            "value": 1155975303,
            "range": "± 19202674",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/13",
            "value": 2190422546,
            "range": "± 22988678",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/14",
            "value": 4384598973,
            "range": "± 152511282",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/15",
            "value": 8007434625,
            "range": "± 102107532",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/16",
            "value": 15360142554,
            "range": "± 322338208",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/8",
            "value": 6082270,
            "range": "± 591748",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/9",
            "value": 9130521,
            "range": "± 798460",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/10",
            "value": 15055343,
            "range": "± 1230950",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/11",
            "value": 25586499,
            "range": "± 1608798",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/12",
            "value": 43447973,
            "range": "± 2916180",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/13",
            "value": 78883088,
            "range": "± 5359365",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/14",
            "value": 143655405,
            "range": "± 6426738",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/15",
            "value": 259011099,
            "range": "± 15091179",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/16",
            "value": 470274011,
            "range": "± 9853209",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "jack@electriccoin.co",
            "name": "str4d",
            "username": "str4d"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "7239a02ea34008b8973d742b3e918bbfbc01b75e",
          "message": "Merge pull request #701 from zcash/ff-0.13\n\nMigrate to published `ff 0.13`",
          "timestamp": "2022-12-07T10:21:33Z",
          "tree_id": "990a399f5ef1f3a10a716fd6523b240f8547a695",
          "url": "https://github.com/QED-it/halo2/commit/7239a02ea34008b8973d742b3e918bbfbc01b75e"
        },
        "date": 1671536039156,
        "tool": "cargo",
        "benches": [
          {
            "name": "WIDTH = 3, RATE = 2-prover",
            "value": 79391303,
            "range": "± 7310507",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 3, RATE = 2-verifier",
            "value": 4112138,
            "range": "± 502465",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 9, RATE = 8-prover",
            "value": 166364612,
            "range": "± 5481397",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 9, RATE = 8-verifier",
            "value": 5048188,
            "range": "± 570179",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 12, RATE = 11-prover",
            "value": 227893734,
            "range": "± 7178689",
            "unit": "ns/iter"
          },
          {
            "name": "WIDTH = 12, RATE = 11-verifier",
            "value": 5951966,
            "range": "± 593585",
            "unit": "ns/iter"
          },
          {
            "name": "Poseidon/2-to-1",
            "value": 47452,
            "range": "± 1665",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash-to-point/510",
            "value": 173618,
            "range": "± 2598",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash/510",
            "value": 189583,
            "range": "± 6190",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/commit/510",
            "value": 307556,
            "range": "± 12003",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/short-commit/510",
            "value": 306997,
            "range": "± 10018",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash-to-point/520",
            "value": 178257,
            "range": "± 6784",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash/520",
            "value": 194656,
            "range": "± 11907",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/commit/520",
            "value": 309417,
            "range": "± 13253",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/short-commit/520",
            "value": 308923,
            "range": "± 9876",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash-to-point/1086",
            "value": 370420,
            "range": "± 33664",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/hash/1086",
            "value": 386615,
            "range": "± 12694",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/commit/1086",
            "value": 502260,
            "range": "± 13694",
            "unit": "ns/iter"
          },
          {
            "name": "Sinsemilla/short-commit/1086",
            "value": 502259,
            "range": "± 12132",
            "unit": "ns/iter"
          },
          {
            "name": "double-and-add",
            "value": 3963877,
            "range": "± 95719",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/14",
            "value": 7113357,
            "range": "± 77093",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/15",
            "value": 12844182,
            "range": "± 171748",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/16",
            "value": 26611852,
            "range": "± 450261",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/17",
            "value": 50193225,
            "range": "± 725070",
            "unit": "ns/iter"
          },
          {
            "name": "dev-lookup/18",
            "value": 98799435,
            "range": "± 1700953",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/3",
            "value": 9245,
            "range": "± 1213",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/4",
            "value": 10423,
            "range": "± 1293",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/5",
            "value": 17673,
            "range": "± 983",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/6",
            "value": 21605,
            "range": "± 2276",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/7",
            "value": 31806,
            "range": "± 4638",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/8",
            "value": 53871,
            "range": "± 8973",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/9",
            "value": 119949,
            "range": "± 15806",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/10",
            "value": 253284,
            "range": "± 39013",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/11",
            "value": 545379,
            "range": "± 80826",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/12",
            "value": 948871,
            "range": "± 111297",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/13",
            "value": 1869588,
            "range": "± 117074",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/14",
            "value": 4051974,
            "range": "± 356232",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/15",
            "value": 8743345,
            "range": "± 849563",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/16",
            "value": 19335811,
            "range": "± 2201761",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/17",
            "value": 41811405,
            "range": "± 2622150",
            "unit": "ns/iter"
          },
          {
            "name": "fft/k/18",
            "value": 98021447,
            "range": "± 3474757",
            "unit": "ns/iter"
          },
          {
            "name": "hash-to-curve/Pallas",
            "value": 34624,
            "range": "± 1692",
            "unit": "ns/iter"
          },
          {
            "name": "hash-to-curve/Vesta",
            "value": 34751,
            "range": "± 1575",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/8",
            "value": 190551902,
            "range": "± 12141329",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/9",
            "value": 411326787,
            "range": "± 8408845",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/10",
            "value": 886096973,
            "range": "± 15874494",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/11",
            "value": 1887497407,
            "range": "± 18253085",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/12",
            "value": 4050509704,
            "range": "± 33362561",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/13",
            "value": 8611287007,
            "range": "± 51737996",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/14",
            "value": 18370779492,
            "range": "± 61251036",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/15",
            "value": 38845490903,
            "range": "± 169301043",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-keygen/16",
            "value": 79766403374,
            "range": "± 1680504485",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/8",
            "value": 125972539,
            "range": "± 7726284",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/9",
            "value": 217992502,
            "range": "± 8586788",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/10",
            "value": 368861954,
            "range": "± 11422758",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/11",
            "value": 669858854,
            "range": "± 10527679",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/12",
            "value": 1188286914,
            "range": "± 27832025",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/13",
            "value": 2263450464,
            "range": "± 28504862",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/14",
            "value": 4400771559,
            "range": "± 406346962",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/15",
            "value": 8601920883,
            "range": "± 63347562",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-prover/16",
            "value": 16317507011,
            "range": "± 62024358",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/8",
            "value": 6321822,
            "range": "± 736224",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/9",
            "value": 9749997,
            "range": "± 1292480",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/10",
            "value": 15403439,
            "range": "± 1676521",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/11",
            "value": 26564291,
            "range": "± 1932114",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/12",
            "value": 46292043,
            "range": "± 2622551",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/13",
            "value": 80471039,
            "range": "± 4998405",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/14",
            "value": 146998345,
            "range": "± 6970923",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/15",
            "value": 271525736,
            "range": "± 18807580",
            "unit": "ns/iter"
          },
          {
            "name": "plonk-verifier/16",
            "value": 488382604,
            "range": "± 12417574",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}