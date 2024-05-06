use std::io::{Read, Write};
use pasta_curves::{pallas, vesta};
use pasta_curves::vesta::Affine;
use rand::rngs::OsRng;
use halo2_proofs::plonk;
use halo2_proofs::plonk::{Circuit, SingleVerifier, VerifyingKey};
use halo2_proofs::poly::commitment::Params;
use halo2_proofs::transcript::{Blake2bRead, Blake2bWrite};

/// A proof structure
#[derive(Clone, Debug)]
pub struct Proof(Vec<u8>);
impl AsRef<[u8]> for Proof {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Proof {
    /// Creates a proof for the given circuits and instances.
    pub fn create<C>(
        vk: &VerifyingKey<Affine>,
        params: &Params<Affine>,
        circuit: C,
    )
        -> Result<Self, plonk::Error>
        where
            C: Circuit<pallas::Base>,
    {
        let pk = plonk::keygen_pk(&params, vk.clone(), &circuit).unwrap();

        let mut transcript = Blake2bWrite::<_, vesta::Affine, _>::init(vec![]);
        plonk::create_proof(
            &params,
            &pk,
            &[circuit],
            &[&[]],
            OsRng,
            &mut transcript,
        )?;
        let proof = transcript.finalize();

        Ok(Proof(proof))
    }

    /// Verifies this proof with the instances.
    pub fn verify(
        &self,
        vk: &VerifyingKey<Affine>,
        params: &Params<Affine>,
    ) -> Result<(), plonk::Error> {

        let strategy = SingleVerifier::new(&params);
        let mut transcript = Blake2bRead::init(&self.0[..]);
        plonk::verify_proof(&params, &vk, strategy, &[&[]], &mut transcript)
    }
    pub fn new(bytes: Vec<u8>) -> Self {
        Proof(bytes)
    }
}

pub(crate) fn test_proof_size<C>(
    k: u32,
    circuit: C,
    params: Params<Affine>,
    vk: VerifyingKey<Affine>,
) where
    C: Circuit<pallas::Base>,
{
    // Test that the proof size is as expected.
    let circuit_cost =
        halo2_proofs::dev::CircuitCost::<pasta_curves::vesta::Point, _>::measure(k, &circuit);
    let expected_proof_size = usize::from(circuit_cost.proof_size(1));


    let proof = Proof::create(&vk,&params,circuit).unwrap();

    assert!(proof.verify(&vk, &params).is_ok());
    assert_eq!(proof.as_ref().len(), expected_proof_size);
}
pub fn write_test_case<W: Write>(
    mut w: W,
    proof: &Proof,
) -> std::io::Result<()> {
    w.write_all(proof.as_ref())?;
    Ok(())
}

pub fn read_test_case<R: Read>(mut r: R) -> std::io::Result<Proof> {
    let mut proof_bytes = vec![];
    r.read_to_end(&mut proof_bytes)?;
    let proof = Proof::new(proof_bytes);

    Ok(proof)
}