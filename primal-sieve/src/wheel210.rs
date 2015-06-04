#![allow(dead_code)]

use wheel::{WheelInfo, WheelElem, WheelInit, BYTE_SIZE, BYTE_MODULO};

///
/// @file   WheelFactorization.cpp
/// @brief  Precomputed arrays for wheel factorization.
///
/// Copyright (C) 2013 Kim Walisch, <kim.walisch@gmail.com>
///
/// This file is distributed under the BSD License. See the COPYING
/// file in the top level directory.
///

pub fn bit_index(n: usize) -> (bool, usize) {
    let init = &INIT[n % MODULO];
    (init.next_mult_factor == 0, (n / MODULO) * SIZE + init.wheel_index as usize)
}
pub fn from_bit_index(bit: usize) -> usize {
    (bit / SIZE) * MODULO + TRUE_AT_BIT[bit % SIZE]
}

pub fn set_bit(x: &mut [u8], si: &mut usize, wi: &mut usize, prime: usize) {
    unsafe {
        let WheelElem { unset_bit, next_mult_factor, correction, next } =
            *WHEEL.get_unchecked(*wi);
        *x.get_unchecked_mut(*si) |= unset_bit;

        *si += prime * next_mult_factor as usize;
        *si += correction as usize;
        *wi = wi.wrapping_add(next as usize);
    }
}

pub fn compute_wheel_elem(p: usize, low: usize) -> WheelInfo {
    let mut mult = p * p;

    let init = &INIT[p % MODULO];
    let next_mult_factor = init.next_mult_factor;
    mult += p * next_mult_factor as usize;

    let low_offset = mult - low;

    let wheel_index = WHEEL_OFFSETS[p % BYTE_MODULO] * SIZE;
    let sieve_index = low_offset * BYTE_SIZE / BYTE_MODULO / 8;

    let ret = WheelInfo {
        true_prime: p,
        prime: p / BYTE_MODULO,
        sieve_index: sieve_index,
        wheel_index: wheel_index,
    };
    ret
}

const WHEEL_OFFSETS: &'static [usize; 30] = &[
    0, 0, 0, 0, 0, 0,
    0, 1, 0, 0, 0, 2,
    0, 3, 0, 0, 0, 4,
    0, 5, 0, 0, 0, 6,
    0, 0, 0, 0, 0, 7,
    ];
pub const SIZE: usize = 48;

pub const MODULO: usize = 210;

const TRUE_AT_BIT: &'static [usize; 48] = &[
    1, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59,
    61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113,
    121, 127, 131, 137, 139, 143, 149, 151, 157, 163, 167, 169,
    173, 179, 181, 187, 191, 193, 197, 199, 209
    ];
const INIT: &'static [WheelInit; 210] = &[
  init!{1,  0}, init!{0,  0}, init!{9,  1}, init!{8,  1},
    init!{7,  1}, init!{6,  1}, init!{5,  1}, init!{4,  1},
  init!{3,  1}, init!{2,  1}, init!{1,  1}, init!{0,  1},
    init!{1,  2}, init!{0,  2}, init!{3,  3}, init!{2,  3},
  init!{1,  3}, init!{0,  3}, init!{1,  4}, init!{0,  4},
    init!{3,  5}, init!{2,  5}, init!{1,  5}, init!{0,  5},
  init!{5,  6}, init!{4,  6}, init!{3,  6}, init!{2,  6},
    init!{1,  6}, init!{0,  6}, init!{1,  7}, init!{0,  7},
  init!{5,  8}, init!{4,  8}, init!{3,  8}, init!{2,  8},
    init!{1,  8}, init!{0,  8}, init!{3,  9}, init!{2,  9},
  init!{1,  9}, init!{0,  9}, init!{1, 10}, init!{0, 10},
    init!{3, 11}, init!{2, 11}, init!{1, 11}, init!{0, 11},
  init!{5, 12}, init!{4, 12}, init!{3, 12}, init!{2, 12},
    init!{1, 12}, init!{0, 12}, init!{5, 13}, init!{4, 13},
  init!{3, 13}, init!{2, 13}, init!{1, 13}, init!{0, 13},
    init!{1, 14}, init!{0, 14}, init!{5, 15}, init!{4, 15},
  init!{3, 15}, init!{2, 15}, init!{1, 15}, init!{0, 15},
    init!{3, 16}, init!{2, 16}, init!{1, 16}, init!{0, 16},
  init!{1, 17}, init!{0, 17}, init!{5, 18}, init!{4, 18},
    init!{3, 18}, init!{2, 18}, init!{1, 18}, init!{0, 18},
  init!{3, 19}, init!{2, 19}, init!{1, 19}, init!{0, 19},
    init!{5, 20}, init!{4, 20}, init!{3, 20}, init!{2, 20},
  init!{1, 20}, init!{0, 20}, init!{7, 21}, init!{6, 21},
    init!{5, 21}, init!{4, 21}, init!{3, 21}, init!{2, 21},
  init!{1, 21}, init!{0, 21}, init!{3, 22}, init!{2, 22},
    init!{1, 22}, init!{0, 22}, init!{1, 23}, init!{0, 23},
  init!{3, 24}, init!{2, 24}, init!{1, 24}, init!{0, 24},
    init!{1, 25}, init!{0, 25}, init!{3, 26}, init!{2, 26},
  init!{1, 26}, init!{0, 26}, init!{7, 27}, init!{6, 27},
    init!{5, 27}, init!{4, 27}, init!{3, 27}, init!{2, 27},
  init!{1, 27}, init!{0, 27}, init!{5, 28}, init!{4, 28},
    init!{3, 28}, init!{2, 28}, init!{1, 28}, init!{0, 28},
  init!{3, 29}, init!{2, 29}, init!{1, 29}, init!{0, 29},
    init!{5, 30}, init!{4, 30}, init!{3, 30}, init!{2, 30},
  init!{1, 30}, init!{0, 30}, init!{1, 31}, init!{0, 31},
    init!{3, 32}, init!{2, 32}, init!{1, 32}, init!{0, 32},
  init!{5, 33}, init!{4, 33}, init!{3, 33}, init!{2, 33},
    init!{1, 33}, init!{0, 33}, init!{1, 34}, init!{0, 34},
  init!{5, 35}, init!{4, 35}, init!{3, 35}, init!{2, 35},
    init!{1, 35}, init!{0, 35}, init!{5, 36}, init!{4, 36},
  init!{3, 36}, init!{2, 36}, init!{1, 36}, init!{0, 36},
    init!{3, 37}, init!{2, 37}, init!{1, 37}, init!{0, 37},
  init!{1, 38}, init!{0, 38}, init!{3, 39}, init!{2, 39},
    init!{1, 39}, init!{0, 39}, init!{5, 40}, init!{4, 40},
  init!{3, 40}, init!{2, 40}, init!{1, 40}, init!{0, 40},
    init!{1, 41}, init!{0, 41}, init!{5, 42}, init!{4, 42},
  init!{3, 42}, init!{2, 42}, init!{1, 42}, init!{0, 42},
    init!{3, 43}, init!{2, 43}, init!{1, 43}, init!{0, 43},
  init!{1, 44}, init!{0, 44}, init!{3, 45}, init!{2, 45},
    init!{1, 45}, init!{0, 45}, init!{1, 46}, init!{0, 46},
  init!{9, 47}, init!{8, 47}, init!{7, 47}, init!{6, 47},
    init!{5, 47}, init!{4, 47}, init!{3, 47}, init!{2, 47},
  init!{1, 47}, init!{0, 47}
];

const WHEEL: &'static [WheelElem; 8*48] = &[
    // remainder 1
    elem!(0u8,6,0,1),elem!(1u8,4,0,1),elem!(2u8,2,0,1),elem!(3u8,4,0,1),
        elem!(4u8,2,0,1),elem!(5u8,4,0,1),elem!(6u8,6,0,1),elem!(7u8,2,1,1),
        elem!(0u8,6,0,1),elem!(1u8,4,0,1),elem!(2u8,2,0,1),elem!(3u8,4,0,1),
        elem!(4u8,2,0,1),elem!(5u8,4,0,1),elem!(6u8,6,0,1),elem!(7u8,2,1,1),
        elem!(0u8,6,0,1),elem!(1u8,4,0,1),elem!(2u8,2,0,1),elem!(3u8,4,0,1),
        elem!(4u8,2,0,1),elem!(5u8,4,0,1),elem!(6u8,6,0,1),elem!(7u8,2,1,1),
        elem!(0u8,6,0,1),elem!(1u8,4,0,1),elem!(2u8,2,0,1),elem!(3u8,4,0,1),
        elem!(4u8,2,0,1),elem!(5u8,4,0,1),elem!(6u8,6,0,1),elem!(7u8,2,1,1),
        elem!(0u8,6,0,1),elem!(1u8,4,0,1),elem!(2u8,2,0,1),elem!(3u8,4,0,1),
        elem!(4u8,2,0,1),elem!(5u8,4,0,1),elem!(6u8,6,0,1),elem!(7u8,2,1,1),
        elem!(0u8,6,0,1),elem!(1u8,4,0,1),elem!(2u8,2,0,1),elem!(3u8,4,0,1),
        elem!(4u8,2,0,1),elem!(5u8,4,0,1),elem!(6u8,6,0,1),elem!(7u8,2,1,-47),

    // remainder 7
    elem!(5u8,4,1,1),elem!(4u8,2,1,1),elem!(0u8,4,0,1),elem!(7u8,2,1,1),
        elem!(3u8,4,1,1),elem!(2u8,6,1,1),elem!(6u8,2,1,1),elem!(1u8,6,1,1),
        elem!(5u8,4,1,1),elem!(4u8,2,1,1),elem!(0u8,4,0,1),elem!(7u8,2,1,1),
        elem!(3u8,4,1,1),elem!(2u8,6,1,1),elem!(6u8,2,1,1),elem!(1u8,6,1,1),
        elem!(5u8,4,1,1),elem!(4u8,2,1,1),elem!(0u8,4,0,1),elem!(7u8,2,1,1),
        elem!(3u8,4,1,1),elem!(2u8,6,1,1),elem!(6u8,2,1,1),elem!(1u8,6,1,1),
        elem!(5u8,4,1,1),elem!(4u8,2,1,1),elem!(0u8,4,0,1),elem!(7u8,2,1,1),
        elem!(3u8,4,1,1),elem!(2u8,6,1,1),elem!(6u8,2,1,1),elem!(1u8,6,1,1),
        elem!(5u8,4,1,1),elem!(4u8,2,1,1),elem!(0u8,4,0,1),elem!(7u8,2,1,1),
        elem!(3u8,4,1,1),elem!(2u8,6,1,1),elem!(6u8,2,1,1),elem!(1u8,6,1,1),
        elem!(5u8,4,1,1),elem!(4u8,2,1,1),elem!(0u8,4,0,1),elem!(7u8,2,1,1),
        elem!(3u8,4,1,1),elem!(2u8,6,1,1),elem!(6u8,2,1,1),elem!(1u8,6,1,-47),

    // remainder 11
    elem!(0u8,2,0,1),elem!(6u8,4,2,1),elem!(1u8,2,0,1),elem!(7u8,4,2,1),
        elem!(3u8,6,2,1),elem!(5u8,2,1,1),elem!(2u8,6,2,1),elem!(4u8,4,2,1),
        elem!(0u8,2,0,1),elem!(6u8,4,2,1),elem!(1u8,2,0,1),elem!(7u8,4,2,1),
        elem!(3u8,6,2,1),elem!(5u8,2,1,1),elem!(2u8,6,2,1),elem!(4u8,4,2,1),
        elem!(0u8,2,0,1),elem!(6u8,4,2,1),elem!(1u8,2,0,1),elem!(7u8,4,2,1),
        elem!(3u8,6,2,1),elem!(5u8,2,1,1),elem!(2u8,6,2,1),elem!(4u8,4,2,1),
        elem!(0u8,2,0,1),elem!(6u8,4,2,1),elem!(1u8,2,0,1),elem!(7u8,4,2,1),
        elem!(3u8,6,2,1),elem!(5u8,2,1,1),elem!(2u8,6,2,1),elem!(4u8,4,2,1),
        elem!(0u8,2,0,1),elem!(6u8,4,2,1),elem!(1u8,2,0,1),elem!(7u8,4,2,1),
        elem!(3u8,6,2,1),elem!(5u8,2,1,1),elem!(2u8,6,2,1),elem!(4u8,4,2,1),
        elem!(0u8,2,0,1),elem!(6u8,4,2,1),elem!(1u8,2,0,1),elem!(7u8,4,2,1),
        elem!(3u8,6,2,1),elem!(5u8,2,1,1),elem!(2u8,6,2,1),elem!(4u8,4,2,-47),

    // remainder 13
    elem!(5u8,4,2,1),elem!(2u8,2,1,1),elem!(1u8,4,1,1),elem!(7u8,6,3,1),
        elem!(4u8,2,1,1),elem!(3u8,6,3,1),elem!(0u8,4,1,1),elem!(6u8,2,1,1),
        elem!(5u8,4,2,1),elem!(2u8,2,1,1),elem!(1u8,4,1,1),elem!(7u8,6,3,1),
        elem!(4u8,2,1,1),elem!(3u8,6,3,1),elem!(0u8,4,1,1),elem!(6u8,2,1,1),
        elem!(5u8,4,2,1),elem!(2u8,2,1,1),elem!(1u8,4,1,1),elem!(7u8,6,3,1),
        elem!(4u8,2,1,1),elem!(3u8,6,3,1),elem!(0u8,4,1,1),elem!(6u8,2,1,1),
        elem!(5u8,4,2,1),elem!(2u8,2,1,1),elem!(1u8,4,1,1),elem!(7u8,6,3,1),
        elem!(4u8,2,1,1),elem!(3u8,6,3,1),elem!(0u8,4,1,1),elem!(6u8,2,1,1),
        elem!(5u8,4,2,1),elem!(2u8,2,1,1),elem!(1u8,4,1,1),elem!(7u8,6,3,1),
        elem!(4u8,2,1,1),elem!(3u8,6,3,1),elem!(0u8,4,1,1),elem!(6u8,2,1,1),
        elem!(5u8,4,2,1),elem!(2u8,2,1,1),elem!(1u8,4,1,1),elem!(7u8,6,3,1),
        elem!(4u8,2,1,1),elem!(3u8,6,3,1),elem!(0u8,4,1,1),elem!(6u8,2,1,-47),

    // remainder 17
    elem!(5u8,2,1,1),elem!(6u8,4,3,1),elem!(0u8,6,3,1),elem!(3u8,2,1,1),
        elem!(4u8,6,3,1),elem!(7u8,4,3,1),elem!(1u8,2,1,1),elem!(2u8,4,2,1),
        elem!(5u8,2,1,1),elem!(6u8,4,3,1),elem!(0u8,6,3,1),elem!(3u8,2,1,1),
        elem!(4u8,6,3,1),elem!(7u8,4,3,1),elem!(1u8,2,1,1),elem!(2u8,4,2,1),
        elem!(5u8,2,1,1),elem!(6u8,4,3,1),elem!(0u8,6,3,1),elem!(3u8,2,1,1),
        elem!(4u8,6,3,1),elem!(7u8,4,3,1),elem!(1u8,2,1,1),elem!(2u8,4,2,1),
        elem!(5u8,2,1,1),elem!(6u8,4,3,1),elem!(0u8,6,3,1),elem!(3u8,2,1,1),
        elem!(4u8,6,3,1),elem!(7u8,4,3,1),elem!(1u8,2,1,1),elem!(2u8,4,2,1),
        elem!(5u8,2,1,1),elem!(6u8,4,3,1),elem!(0u8,6,3,1),elem!(3u8,2,1,1),
        elem!(4u8,6,3,1),elem!(7u8,4,3,1),elem!(1u8,2,1,1),elem!(2u8,4,2,1),
        elem!(5u8,2,1,1),elem!(6u8,4,3,1),elem!(0u8,6,3,1),elem!(3u8,2,1,1),
        elem!(4u8,6,3,1),elem!(7u8,4,3,1),elem!(1u8,2,1,1),elem!(2u8,4,2,-47),

    // remainder 19
    elem!(0u8,4,2,1),elem!(4u8,6,4,1),elem!(2u8,2,1,1),elem!(5u8,6,4,1),
        elem!(3u8,4,2,1),elem!(7u8,2,2,1),elem!(1u8,4,2,1),elem!(6u8,2,2,1),
        elem!(0u8,4,2,1),elem!(4u8,6,4,1),elem!(2u8,2,1,1),elem!(5u8,6,4,1),
        elem!(3u8,4,2,1),elem!(7u8,2,2,1),elem!(1u8,4,2,1),elem!(6u8,2,2,1),
        elem!(0u8,4,2,1),elem!(4u8,6,4,1),elem!(2u8,2,1,1),elem!(5u8,6,4,1),
        elem!(3u8,4,2,1),elem!(7u8,2,2,1),elem!(1u8,4,2,1),elem!(6u8,2,2,1),
        elem!(0u8,4,2,1),elem!(4u8,6,4,1),elem!(2u8,2,1,1),elem!(5u8,6,4,1),
        elem!(3u8,4,2,1),elem!(7u8,2,2,1),elem!(1u8,4,2,1),elem!(6u8,2,2,1),
        elem!(0u8,4,2,1),elem!(4u8,6,4,1),elem!(2u8,2,1,1),elem!(5u8,6,4,1),
        elem!(3u8,4,2,1),elem!(7u8,2,2,1),elem!(1u8,4,2,1),elem!(6u8,2,2,1),
        elem!(0u8,4,2,1),elem!(4u8,6,4,1),elem!(2u8,2,1,1),elem!(5u8,6,4,1),
        elem!(3u8,4,2,1),elem!(7u8,2,2,1),elem!(1u8,4,2,1),elem!(6u8,2,2,-47),

    // remainder 23
    elem!(5u8,6,5,1),elem!(1u8,2,1,1),elem!(6u8,6,5,1),elem!(2u8,4,3,1),
        elem!(3u8,2,1,1),elem!(7u8,4,4,1),elem!(0u8,2,1,1),elem!(4u8,4,3,1),
        elem!(5u8,6,5,1),elem!(1u8,2,1,1),elem!(6u8,6,5,1),elem!(2u8,4,3,1),
        elem!(3u8,2,1,1),elem!(7u8,4,4,1),elem!(0u8,2,1,1),elem!(4u8,4,3,1),
        elem!(5u8,6,5,1),elem!(1u8,2,1,1),elem!(6u8,6,5,1),elem!(2u8,4,3,1),
        elem!(3u8,2,1,1),elem!(7u8,4,4,1),elem!(0u8,2,1,1),elem!(4u8,4,3,1),
        elem!(5u8,6,5,1),elem!(1u8,2,1,1),elem!(6u8,6,5,1),elem!(2u8,4,3,1),
        elem!(3u8,2,1,1),elem!(7u8,4,4,1),elem!(0u8,2,1,1),elem!(4u8,4,3,1),
        elem!(5u8,6,5,1),elem!(1u8,2,1,1),elem!(6u8,6,5,1),elem!(2u8,4,3,1),
        elem!(3u8,2,1,1),elem!(7u8,4,4,1),elem!(0u8,2,1,1),elem!(4u8,4,3,1),
        elem!(5u8,6,5,1),elem!(1u8,2,1,1),elem!(6u8,6,5,1),elem!(2u8,4,3,1),
        elem!(3u8,2,1,1),elem!(7u8,4,4,1),elem!(0u8,2,1,1),elem!(4u8,4,3,-47),

    // remainder 29
    elem!(0u8,2,1,1),elem!(7u8,6,6,1),elem!(6u8,4,4,1),elem!(5u8,2,2,1),
        elem!(4u8,4,4,1),elem!(3u8,2,2,1),elem!(2u8,4,4,1),elem!(1u8,6,6,1),
        elem!(0u8,2,1,1),elem!(7u8,6,6,1),elem!(6u8,4,4,1),elem!(5u8,2,2,1),
        elem!(4u8,4,4,1),elem!(3u8,2,2,1),elem!(2u8,4,4,1),elem!(1u8,6,6,1),
        elem!(0u8,2,1,1),elem!(7u8,6,6,1),elem!(6u8,4,4,1),elem!(5u8,2,2,1),
        elem!(4u8,4,4,1),elem!(3u8,2,2,1),elem!(2u8,4,4,1),elem!(1u8,6,6,1),
        elem!(0u8,2,1,1),elem!(7u8,6,6,1),elem!(6u8,4,4,1),elem!(5u8,2,2,1),
        elem!(4u8,4,4,1),elem!(3u8,2,2,1),elem!(2u8,4,4,1),elem!(1u8,6,6,1),
        elem!(0u8,2,1,1),elem!(7u8,6,6,1),elem!(6u8,4,4,1),elem!(5u8,2,2,1),
        elem!(4u8,4,4,1),elem!(3u8,2,2,1),elem!(2u8,4,4,1),elem!(1u8,6,6,1),
        elem!(0u8,2,1,1),elem!(7u8,6,6,1),elem!(6u8,4,4,1),elem!(5u8,2,2,1),
        elem!(4u8,4,4,1),elem!(3u8,2,2,1),elem!(2u8,4,4,1),elem!(1u8,6,6,-47),
    ];
