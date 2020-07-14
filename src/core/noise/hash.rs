// use crate::terrain::Height;

static MASK: u64 = (2<<32) -1;
static LARGE_PRIME: u64 = 0xffffef95;

#[allow(dead_code)]
pub fn hash_rng(seed: u64, p: f32) -> f32 {
    let _p = p.floor() as u64;
    let mut x: u64 = (_p + seed)*LARGE_PRIME;
    x ^= x << 27;
    x ^= x >> 19;
    x ^= x << 5;
    (x & MASK )as f32 / MASK as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_inside_range() {
        let n = 10000u64;
        let v: f32 = (1..n).map(|x| hash_rng(0, x as f32)).sum::<f32>() / n as f32;
        assert!(v <= 1f32);
        assert!(v >= 0f32);
    }

    #[test]
    fn hash_average_fits() {
        let s = (1..10000u64)
            .map(|x| hash_rng(0,x as f32));
        let average = s.clone()
            .sum::<f32>() / 10000f32;
        assert!((average - 0.5).powf(2f32) < 0.01f32);
    }

    #[test]
    fn hash_variation_good_enough() {
        let n =100u64;
        let dn = 1f32/n as f32;
        let s = (1..n)
            .map(|x| hash_rng(0, x as f32));
        let variance = s.clone()
            .map(|x| (x-0.5).powf(2f32))
            .sum::<f32>() *dn;
        assert!(variance < 0.1f32);
    }

    #[test]
    fn hash_distribution_quality_test() {
        let n = 1000u64;
        let dn = 1f32/(n as f32);

        let mut m=[0f32; 10];
        let siz = 10f32;
        let guess = 1f32/siz; // probability to be in one slot of the array

        (1..n)
            .map(|x| hash_rng(0, x as f32))
            .for_each(|v| {
                let i = (v*siz).floor();
                m[i as usize] += dn;
            });

        let epsilon = m.iter()
            .map(|x| (x - guess).abs())
            .fold(0f32, |x,c| x.max(c));

        assert!( epsilon < 0.01)
    }
}
