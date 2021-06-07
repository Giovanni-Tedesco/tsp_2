use rand::Rng;

pub fn generate_floats(n: usize) -> Vec<f64> {

    let mut v: Vec<f64> = Vec::new(); 
    let mut rng = rand::thread_rng();

    v.push(0f64);

    for _ in 1..n {
        let x: f64 = rng.gen();
        v.push(x);
    }

    return v;

}

pub fn generate_path(float_vec: &Vec<f64>) -> Vec<usize> {
    let n = float_vec.len();
    
    let mut indicies: Vec<usize> = (1..n).collect();
    indicies.sort_by(|&a, &b| float_vec[a].partial_cmp(&float_vec[b]).unwrap());

    indicies.insert(0, 0);
    
    return indicies;
}


