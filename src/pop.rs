#![allow(unused_imports)]
#![allow(unused_variables)]
extern crate bellman; // 用于创建 zk-SNARK 算术电路的 crate
extern crate pairing; // 配对友好的椭圆曲线算法库
extern crate rand; // 随机数生成库

// 随机数生成器 (参数以及证明生成的过程中)
use self::rand::{thread_rng, Rng};

// 椭圆曲线的一些相关工具
use self::pairing::Engine;

use ff::PrimeField;

// BLS12-381 椭圆曲线算法
use bls12_381::{
    Bls12
};

// 电路构造接口
use self::bellman::{
    Circuit,
    ConstraintSystem,
    SynthesisError
};

// groth16 证明系统
use self::bellman::groth16::{
    Proof,
    generate_random_parameters,
    prepare_verifying_key,
    create_random_proof,
    verify_proof
};

// 证明成绩大于 60

pub struct Pop<Scalar: PrimeField> {
    pub x: Option<Scalar>,
}

impl<Scalar: PrimeField> Circuit<Scalar> for Pop<Scalar> {
    fn synthesize<CS: ConstraintSystem<Scalar>> (
        self,
        cs: &mut CS
    ) -> Result<(), SynthesisError> {
        

        Ok(())
    }
}

mod test {
    use super::*;

    #[test]
    fn test_pop() {
        let rng = &mut thread_rng();

        println!("参数生成中...");

        

    }

}




