use quad_rand;

// static mut SEED: AtomicUsize = AtomicUsize::new(123456789);
// const A: usize = 1103515245;
// const C: usize = 12345;
// const M: usize = 2_usize.pow(31);

pub fn rand_f64() -> f64 {
    quad_rand::gen_range(0., 1.)
}

pub fn rand_usize(range: std::ops::Range<usize>) -> usize {
    quad_rand::gen_range(range.start, range.end)
}