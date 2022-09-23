//! packed_simd-like wrapper layer

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
#[cfg(target_arch = "wasm32")]
use std::arch::wasm32::*;

#[cfg(not(target_arch = "wasm32"))]
#[allow(non_camel_case_types)]
pub(crate) type i32x4 = __m128i;

#[cfg(target_arch = "wasm32")]
#[allow(non_camel_case_types)]
pub(crate) type i32x4 = v128;

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn new(e0: i32, e1: i32, e2: i32, e3: i32) -> i32x4 {
    unsafe { _mm_set_epi32(e3, e2, e1, e0) }
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn new(e0: i32, e1: i32, e2: i32, e3: i32) -> i32x4 {
    i32x4(e0, e1, e2, e3)
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn zero() -> i32x4 {
    unsafe { _mm_setzero_si128() }
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn zero() -> i32x4 {
    i32x4(0,0,0,0)
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn extract(vals: i32x4, imm: usize) -> u32 {
    unsafe {
        match imm {
            0 => _mm_extract_epi32(vals, 0) as u32,
            1 => _mm_extract_epi32(vals, 1) as u32,
            2 => _mm_extract_epi32(vals, 2) as u32,
            3 => _mm_extract_epi32(vals, 3) as u32,
            _ => core::hint::unreachable_unchecked(),
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn extract(vals: i32x4, imm: usize) -> u32 {
    unsafe {
        match imm {
            0 => u32x4_extract_lane::<0>(vals),
            1 => u32x4_extract_lane::<1>(vals),
            2 => u32x4_extract_lane::<2>(vals),
            3 => u32x4_extract_lane::<3>(vals),
            _ => core::hint::unreachable_unchecked(),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn insert(vals: &mut i32x4, val: i32, imm: usize) {
    let updated = unsafe {
        match imm {
            0 => _mm_insert_epi32(*vals, val, 0),
            1 => _mm_insert_epi32(*vals, val, 1),
            2 => _mm_insert_epi32(*vals, val, 2),
            3 => _mm_insert_epi32(*vals, val, 3),
            _ => core::hint::unreachable_unchecked(),
        }
    };
    unsafe {
        ::std::ptr::write(vals, updated);
    }
}
#[cfg(target_arch = "wasm32")]
pub(crate) fn insert(vals: &mut i32x4, val: i32, imm: usize) {
    let updated = unsafe {
        match imm {
            0 => i32x4_replace_lane::<0>(*vals, val),
            1 => i32x4_replace_lane::<1>(*vals, val),
            2 => i32x4_replace_lane::<2>(*vals, val),
            3 => i32x4_replace_lane::<3>(*vals, val),
            _ => core::hint::unreachable_unchecked(),
        }
    };
    unsafe {
        ::std::ptr::write(vals, updated);
    }
}
