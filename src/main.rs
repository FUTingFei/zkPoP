#![allow(unused_imports)]
#![allow(unused_variables)]
extern crate bellman;
extern crate pairing;
extern crate rand;
use bellman::{Circuit, ConstraintSystem, SynthesisError};
use pairing::{Engine, Field, PrimeField};

mod cube;

fn main() {
    use pairing::bls12_381::{Bls12, Fr};
    use rand::thread_rng;
    use bellman::groth16::{
        create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof, Proof,
    };

    println!("证明我及格了，即我的分数 c 高于某个及格线 x 。");

    let rng = &mut thread_rng();

    println!("生成参数中...");

    // 为算术电路生成参数

    let parms = {

    };


}