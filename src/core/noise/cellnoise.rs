// use rand_core::SeedableRng;
use rand;
use rand::prelude::*;
// use rand_chacha::ChaCha20Rng;

#[allow(dead_code)]
pub fn cellnoise (seed: u32, rnd_tab_size: usize) -> Box<dyn Fn(f32) -> f32> {
    let mut data = Vec::new();
    data.resize(rnd_tab_size, seed);
    rand::thread_rng().fill(&mut data[..]);
    eprintln!("{:?}", data);

    let rnd_tbl: Vec<f32> = data.iter()
        .map(|&i| i as f32)
        .collect ();

    Box::new(
        move |x| {
            let i = x as usize % 0x20;
            rnd_tbl[i]
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let celln = cellnoise(0, 32);
        for x in  1..30 {
            eprintln!("{:?}", celln(x as f32) );
        }
        assert!(true)
    }
}
