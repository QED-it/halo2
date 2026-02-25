CLI: cargo run --example cost-model -p halo2_proofs -- -a 0,1 -a 0,1 -f 0 -g 2 -p 3 5

Circuit {
    k: 5,
    max_deg: 4,
    advice_columns: 2,
    lookups: 0,
    permutations: [
        Permutation {
            columns: 3,
        },
    ],
    column_queries: 10,
    point_sets: 3,
    estimator: Estimator,
}
Proof size: 1312 bytes
Verification: at least 13.26ms