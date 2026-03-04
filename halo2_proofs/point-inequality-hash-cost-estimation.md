cargo run --example cost-model -- \
  -a 0,1 \
  -a 0 \
  -i 0 \
  -f 0 \
  -f 0 \
  -g 3 \
  -p 3 \
  5

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
    column_queries: 12,
    point_sets: 3,
    estimator: Estimator,
}
Proof size: 1376 bytes
Verification: at least 15.86ms