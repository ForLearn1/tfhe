use rand::Rng;

#[derive(Debug)]
pub struct keypairs {
    secret_key : Vec<i64>,
    public_key : Vec<i64>
}



pub fn keygen(lattice_dim : i64, p_modulus : i32, q_modulus : i32) -> keypairs {
  
  let mut rng = rand::thread_rng();
  let secret_key : Vec<i64> = (0..lattice_dim).map(|_| rng.gen_range(0..=1)).collect();
  let public_key : Vec<i64> = (0..lattice_dim).map(|_| rng.gen_range(0..= (p_modulus-1) as i64)).collect();

  let keys = keypairs {secret_key, public_key};
  // println!("{:?} La clé privé est : ", keys.secret_key);
  // println!("{:?} La clé publique est : ", keys.public_key);
  keys 
}
