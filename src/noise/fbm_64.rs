use crate::noise::simplex_64::{simplex_1d, simplex_2d, simplex_3d, simplex_4d};

use simdeez::Simd;

#[inline(always)]
pub unsafe fn fbm_1d<S: Simd>(
    mut x: S::Vf64,
    lacunarity: S::Vf64,
    gain: S::Vf64,
    octaves: u8,
    seed: i64,
) -> S::Vf64 {
    let mut amp = S::set1_pd(1.0);
    let mut scale = amp;
    let mut result = simplex_1d::<S>(x, seed);

    for _ in 1..octaves {
        x = S::mul_pd(x, lacunarity);
        amp = S::mul_pd(amp, gain);
        scale = S::add_pd(scale, amp);
        result = S::add_pd(result, simplex_1d::<S>(x, seed));
    }

    result = S::div_pd(result, scale);
    result
}

#[inline(always)]
pub unsafe fn fbm_2d<S: Simd>(
    mut x: S::Vf64,
    mut y: S::Vf64,
    lac: S::Vf64,
    gain: S::Vf64,
    octaves: u8,
    seed: i64,
) -> S::Vf64 {
    let mut amp = S::set1_pd(1.0);
    let mut scale = amp;
    let mut result = simplex_2d::<S>(x, y, seed);

    for _ in 1..octaves {
        x = S::mul_pd(x, lac);
        y = S::mul_pd(y, lac);
        amp = S::mul_pd(amp, gain);
        scale = S::add_pd(scale, amp);
        result = S::add_pd(S::mul_pd(simplex_2d::<S>(x, y, seed), amp), result);
    }

    result = S::div_pd(result, scale);
    result
}

#[inline(always)]
pub unsafe fn fbm_3d<S: Simd>(
    mut x: S::Vf64,
    mut y: S::Vf64,
    mut z: S::Vf64,
    lac: S::Vf64,
    gain: S::Vf64,
    octaves: u8,
    seed: i64,
) -> S::Vf64 {
    let mut amp = S::set1_pd(1.0);
    let mut scale = amp;
    let mut result = simplex_3d::<S>(x, y, z, seed);

    for _ in 1..octaves {
        x = S::mul_pd(x, lac);
        y = S::mul_pd(y, lac);
        z = S::mul_pd(z, lac);
        amp = S::mul_pd(amp, gain);
        scale = S::add_pd(scale, amp);
        result = S::add_pd(S::mul_pd(simplex_3d::<S>(x, y, z, seed), amp), result);
    }

    result = S::div_pd(result, scale);
    result
}

#[inline(always)]
pub unsafe fn fbm_4d<S: Simd>(
    mut x: S::Vf64,
    mut y: S::Vf64,
    mut z: S::Vf64,
    mut w: S::Vf64,
    lac: S::Vf64,
    gain: S::Vf64,
    octaves: u8,
    seed: i64,
) -> S::Vf64 {
    let mut amp = S::set1_pd(1.0);
    let mut scale = amp;
    let mut result = simplex_4d::<S>(x, y, z, w, seed);

    for _ in 1..octaves {
        x = S::mul_pd(x, lac);
        y = S::mul_pd(y, lac);
        z = S::mul_pd(z, lac);
        w = S::mul_pd(w, lac);
        amp = S::mul_pd(amp, gain);
        scale = S::add_pd(scale, amp);
        result = S::add_pd(result, S::mul_pd(simplex_4d::<S>(x, y, z, w, seed), amp));
    }

    result = S::div_pd(result, scale);
    result
}
