//! Tests for ARM+v7+neon table lookup (vtbl, vtbx) intrinsics.
//!
//! These are included in `{arm, aarch64}::neon`.

use super::*;

#[cfg(target_arch = "aarch64")]
use core_arch::aarch64::*;

#[cfg(target_arch = "arm")]
use core_arch::arm::*;

use core_arch::simd::*;
use std::mem;
use stdsimd_test::simd_test;

macro_rules! test_vtbl {
    ($test_name:ident => $fn_id:ident:
     - table[$table_t:ident]: [$($table_v:expr),*] |
     $(- ctrl[$ctrl_t:ident]: [$($ctrl_v:expr),*] => [$($exp_v:expr),*])|*
    ) => {
        #[simd_test(enable = "neon")]
        unsafe fn $test_name() {
            // create table as array, and transmute it to
            // arm's table type
            let table: $table_t = ::mem::transmute([$($table_v),*]);

            // For each control vector, perform a table lookup and
            // verify the result:
            $(
                {
                    let ctrl: $ctrl_t = ::mem::transmute([$($ctrl_v),*]);
                    let result = $fn_id(table, ::mem::transmute(ctrl));
                    let result: $ctrl_t = ::mem::transmute(result);
                    let expected: $ctrl_t = ::mem::transmute([$($exp_v),*]);
                    assert_eq!(result, expected);
                }
            )*
        }
    }
}

// ARM+v7+neon and AArch64+neon tests

test_vtbl!(
    test_vtbl1_s8 => vtbl1_s8:
    - table[int8x8_t]: [0_i8, -11, 2, 3, 4, 5, 6, 7] |
    - ctrl[i8x8]: [3_i8, 4, 1, 6, 0, 2, 7, 5] => [3_i8, 4, -11, 6, 0, 2, 7, 5] |
    - ctrl[i8x8]: [3_i8, 8, 1, -9, 10, 2, 15, 5] => [3_i8, 0, -11, 0, 0, 2, 0, 5]
);

test_vtbl!(
    test_vtbl1_u8 => vtbl1_u8:
    - table[uint8x8_t]: [0_u8, 1, 2, 3, 4, 5, 6, 7] |
    - ctrl[u8x8]: [3_u8, 4, 1, 6, 0, 2, 7, 5] => [3_u8, 4, 1, 6, 0, 2, 7, 5] |
    - ctrl[u8x8]: [3_u8, 8, 1, 9, 10, 2, 15, 5] => [3_u8, 0, 1, 0, 0, 2, 0, 5]
);

test_vtbl!(
    test_vtbl1_p8 => vtbl1_p8:
    - table[poly8x8_t]: [0_u8, 1, 2, 3, 4, 5, 6, 7] |
    - ctrl[u8x8]: [3_u8, 4, 1, 6, 0, 2, 7, 5] => [3_u8, 4, 1, 6, 0, 2, 7, 5] |
    - ctrl[u8x8]: [3_u8, 8, 1, 9, 10, 2, 15, 5] => [3_u8, 0, 1, 0, 0, 2, 0, 5]
);

test_vtbl!(
    test_vtbl2_s8 => vtbl2_s8:
    - table[int8x8x2_t]: [
        0_i8, -17, 34, 51, 68, 85, 102, 119,
        -106, -93, -84, -117, -104, -116, -72, -121
    ] |
    - ctrl[i8x8]: [127_i8, 15, 1, 14, 2, 13, 3, 12] => [0_i8, -121, -17, -72, 34, -116, 51, -104] |
    - ctrl[i8x8]: [4_i8, 11, 16, 10, 6, -19, 7, 18] => [68_i8, -117, 0, -84, 102, 0, 119, 0]
);

test_vtbl!(
    test_vtbl2_u8 => vtbl2_u8:
    - table[uint8x8x2_t]: [
        0_u8, 17, 34, 51, 68, 85, 102, 119,
        136, 153, 170, 187, 204, 221, 238, 255
    ] |
    - ctrl[u8x8]: [127_u8, 15, 1, 14, 2, 13, 3, 12] => [0_u8, 255, 17, 238, 34, 221, 51, 204] |
    - ctrl[u8x8]: [4_u8, 11, 16, 10, 6, 19, 7, 18] => [68_u8, 187, 0, 170, 102, 0, 119, 0]
);

test_vtbl!(
    test_vtbl2_p8 => vtbl2_p8:
    - table[poly8x8x2_t]: [
        0_u8, 17, 34, 51, 68, 85, 102, 119,
        136, 153, 170, 187, 204, 221, 238, 255
    ] |
    - ctrl[u8x8]: [127_u8, 15, 1, 14, 2, 13, 3, 12] => [0_u8, 255, 17, 238, 34, 221, 51, 204] |
    - ctrl[u8x8]: [4_u8, 11, 16, 10, 6, 19, 7, 18] => [68_u8, 187, 0, 170, 102, 0, 119, 0]
);

test_vtbl!(
    test_vtbl3_s8 => vtbl3_s8:
    - table[int8x8x3_t]: [
        0_i8, -17, 34, 51, 68, 85, 102, 119,
        -106, -93, -84, -117, -104, -116, -72, -121,
        0, 1, -2, 3, 4, -5, 6, 7
    ] |
    - ctrl[i8x8]: [127_i8, 15, 1, 19, 2, 13, 21, 12] => [0_i8, -121, -17, 3, 34, -116, -5, -104] |
    - ctrl[i8x8]: [4_i8, 11, 16, 10, 6, -27, 7, 18] => [68_i8, -117, 0, -84, 102, 0, 119, -2]
);

test_vtbl!(
    test_vtbl3_u8 => vtbl3_u8:
    - table[uint8x8x3_t]: [
        0_u8, 17, 34, 51, 68, 85, 102, 119,
        136, 153, 170, 187, 204, 221, 238, 255,
        0, 1, 2, 3, 4, 5, 6, 7
    ] |
    - ctrl[u8x8]: [127_u8, 15, 1, 19, 2, 13, 21, 12] => [0_u8, 255, 17, 3, 34, 221, 5, 204] |
    - ctrl[u8x8]: [4_u8, 11, 16, 10, 6, 27, 7, 18] => [68_u8, 187, 0, 170, 102, 0, 119, 2]
);

test_vtbl!(
    test_vtbl3_p8 => vtbl3_p8:
    - table[poly8x8x3_t]: [
        0_u8, 17, 34, 51, 68, 85, 102, 119,
        136, 153, 170, 187, 204, 221, 238, 255,
        0, 1, 2, 3, 4, 5, 6, 7
    ] |
    - ctrl[u8x8]: [127_u8, 15, 1, 19, 2, 13, 21, 12] => [0_u8, 255, 17, 3, 34, 221, 5, 204] |
    - ctrl[u8x8]: [4_u8, 11, 16, 10, 6, 27, 7, 18] => [68_u8, 187, 0, 170, 102, 0, 119, 2]
);

test_vtbl!(
    test_vtbl4_s8 => vtbl4_s8:
    - table[int8x8x4_t]: [
        0_i8, -17, 34, 51, 68, 85, 102, 119,
        -106, -93, -84, -117, -104, -116, -72, -121,
        0, 1, -2, 3, 4, -5, 6, 7,
        8, -9, 10, 11, 12, -13, 14, 15
    ] |
    - ctrl[i8x8]: [127_i8, 15, 1, 19, 2, 13, 25, 12] => [0_i8, -121, -17, 3, 34, -116, -9, -104] |
    - ctrl[i8x8]: [4_i8, 11, 32, 10, -33, 27, 7, 18] => [68_i8, -117, 0, -84, 0, 11, 119, -2]
);

test_vtbl!(
    test_vtbl4_u8 => vtbl4_u8:
    - table[uint8x8x4_t]: [
        0_u8, 17, 34, 51, 68, 85, 102, 119,
        136, 153, 170, 187, 204, 221, 238, 255,
        0, 1, 2, 3, 4, 5, 6, 7,
        8, 9, 10, 11, 12, 13, 14, 15
    ] |
    - ctrl[u8x8]: [127_u8, 15, 1, 19, 2, 13, 21, 12] => [0_u8, 255, 17, 3, 34, 221, 5, 204] |
    - ctrl[u8x8]: [4_u8, 11, 16, 10, 6, 27, 7, 18] => [68_u8, 187, 0, 170, 102, 11, 119, 2]
);

test_vtbl!(
    test_vtbl4_p8 => vtbl4_p8:
    - table[poly8x8x4_t]: [
        0_u8, 17, 34, 51, 68, 85, 102, 119,
        136, 153, 170, 187, 204, 221, 238, 255,
        0, 1, 2, 3, 4, 5, 6, 7,
            8, 9, 10, 11, 12, 13, 14, 15
    ] |
    - ctrl[u8x8]: [127_u8, 15, 1, 19, 2, 13, 21, 12] => [0_u8, 255, 17, 3, 34, 221, 5, 204] |
    - ctrl[u8x8]: [4_u8, 11, 16, 10, 6, 27, 7, 18] => [68_u8, 187, 0, 170, 102, 11, 119, 2]
);

macro_rules! test_vtbx {
    ($test_name:ident => $fn_id:ident:
     - table[$table_t:ident]: [$($table_v:expr),*] |
     - ext[$ext_t:ident]: [$($ext_v:expr),*] |
     $(- ctrl[$ctrl_t:ident]: [$($ctrl_v:expr),*] => [$($exp_v:expr),*])|*
    ) => {
        #[simd_test(enable = "neon")]
        unsafe fn $test_name() {
            // create table as array, and transmute it to
            // arm's table type
            let table: $table_t = ::mem::transmute([$($table_v),*]);
            let ext: $ext_t = ::mem::transmute([$($ext_v),*]);

            // For each control vector, perform a table lookup and
            // verify the result:
            $(
                {
                    let ctrl: $ctrl_t = ::mem::transmute([$($ctrl_v),*]);
                    let result = $fn_id(ext, table, ::mem::transmute(ctrl));
                    let result: $ctrl_t = ::mem::transmute(result);
                    let expected: $ctrl_t = ::mem::transmute([$($exp_v),*]);
                    assert_eq!(result, expected);
                }
            )*
        }
    }
}

test_vtbx!(
    test_vtbx1_s8 => vtbx1_s8:
    - table[int8x8_t]: [0_i8, 1, 2, -3, 4, 5, 6, 7] |
    - ext[int8x8_t]: [50_i8, 51, 52, 53, 54, 55, 56, 57] |
    - ctrl[i8x8]: [3_i8, 4, 1, 6, 0, 2, 7, 5] => [-3_i8, 4, 1, 6, 0, 2, 7, 5] |
    - ctrl[i8x8]: [3_i8, 8, 1, 9, 10, 2, -15, 5] => [-3_i8, 51, 1, 53, 54, 2, 56, 5]
);

test_vtbx!(
    test_vtbx1_u8 => vtbx1_u8:
    - table[uint8x8_t]: [0_u8, 1, 2, 3, 4, 5, 6, 7] |
    - ext[uint8x8_t]: [50_u8, 51, 52, 53, 54, 55, 56, 57] |
    - ctrl[u8x8]: [3_u8, 4, 1, 6, 0, 2, 7, 5] => [3_u8, 4, 1, 6, 0, 2, 7, 5] |
    - ctrl[u8x8]: [3_u8, 8, 1, 9, 10, 2, 15, 5] => [3_u8, 51, 1, 53, 54, 2, 56, 5]
);

test_vtbx!(
    test_vtbx1_p8 => vtbx1_p8:
    - table[poly8x8_t]: [0_u8, 1, 2, 3, 4, 5, 6, 7] |
    - ext[poly8x8_t]: [50_u8, 51, 52, 53, 54, 55, 56, 57] |
    - ctrl[u8x8]: [3_u8, 4, 1, 6, 0, 2, 7, 5] => [3_u8, 4, 1, 6, 0, 2, 7, 5] |
    - ctrl[u8x8]: [3_u8, 8, 1, 9, 10, 2, 15, 5] => [3_u8, 51, 1, 53, 54, 2, 56, 5]
);

test_vtbx!(
    test_vtbx2_s8 => vtbx2_s8:
    - table[int8x8x2_t]: [0_i8, 1, 2, -3, 4, 5, 6, 7, 8, 9, -10, 11, 12, -13, 14, 15] |
    - ext[int8x8_t]: [50_i8, 51, 52, 53, 54, 55, 56, 57] |
    - ctrl[i8x8]: [3_i8, 4, 1, 6, 10, 2, 7, 15] => [-3_i8, 4, 1, 6, -10, 2, 7, 15] |
    - ctrl[i8x8]: [3_i8, 8, 1, 10, 17, 2, 15, -19] => [-3_i8, 8, 1, -10, 54, 2, 15, 57]
);

test_vtbx!(
    test_vtbx2_u8 => vtbx2_u8:
    - table[uint8x8x2_t]: [0_i8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15] |
    - ext[uint8x8_t]: [50_i8, 51, 52, 53, 54, 55, 56, 57] |
    - ctrl[u8x8]: [3_u8, 4, 1, 6, 10, 2, 7, 15] => [3_i8, 4, 1, 6, 10, 2, 7, 15] |
    - ctrl[u8x8]: [3_u8, 8, 1, 10, 17, 2, 15, 19] => [3_i8, 8, 1, 10, 54, 2, 15, 57]
);

test_vtbx!(
    test_vtbx2_p8 => vtbx2_p8:
    - table[poly8x8x2_t]: [0_i8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15] |
    - ext[poly8x8_t]: [50_i8, 51, 52, 53, 54, 55, 56, 57] |
    - ctrl[u8x8]: [3_u8, 4, 1, 6, 10, 2, 7, 15] => [3_i8, 4, 1, 6, 10, 2, 7, 15] |
    - ctrl[u8x8]: [3_u8, 8, 1, 10, 17, 2, 15, 19] => [3_i8, 8, 1, 10, 54, 2, 15, 57]
);

test_vtbx!(
    test_vtbx3_s8 => vtbx3_s8:
    - table[int8x8x3_t]: [
        0_i8, 1, 2, -3, 4, 5, 6, 7,
        8, 9, -10, 11, 12, -13, 14, 15,
        16, -17, 18, 19, 20, 21, 22, 23 ] |
    - ext[int8x8_t]: [50_i8, 51, 52, 53, 54, 55, 56, 57] |
    - ctrl[i8x8]: [3_i8, 4, 17, 22, 10, 2, 7, 15] => [-3_i8, 4, -17, 22, -10, 2, 7, 15] |
    - ctrl[i8x8]: [3_i8, 8, 17, 10, 37, 2, 19, -29] => [-3_i8, 8, -17, -10, 54, 2, 19, 57]
);

test_vtbx!(
    test_vtbx3_u8 => vtbx3_u8:
    - table[uint8x8x3_t]: [
        0_i8, 1, 2, 3, 4, 5, 6, 7,
        8, 9, 10, 11, 12, 13, 14, 15,
        16, 17, 18, 19, 20, 21, 22, 23 ] |
    - ext[uint8x8_t]: [50_i8, 51, 52, 53, 54, 55, 56, 57] |
    - ctrl[u8x8]: [3_u8, 4, 17, 22, 10, 2, 7, 15] => [3_i8, 4, 17, 22, 10, 2, 7, 15] |
    - ctrl[u8x8]: [3_u8, 8, 17, 10, 37, 2, 19, 29] => [3_i8, 8, 17, 10, 54, 2, 19, 57]
);

test_vtbx!(
    test_vtbx3_p8 => vtbx3_p8:
    - table[poly8x8x3_t]: [
        0_i8, 1, 2, 3, 4, 5, 6, 7,
        8, 9, 10, 11, 12, 13, 14, 15,
        16, 17, 18, 19, 20, 21, 22, 23 ] |
    - ext[poly8x8_t]: [50_i8, 51, 52, 53, 54, 55, 56, 57] |
    - ctrl[u8x8]: [3_u8, 4, 17, 22, 10, 2, 7, 15] => [3_i8, 4, 17, 22, 10, 2, 7, 15] |
    - ctrl[u8x8]: [3_u8, 8, 17, 10, 37, 2, 19, 29] => [3_i8, 8, 17, 10, 54, 2, 19, 57]
);

test_vtbx!(
    test_vtbx4_s8 => vtbx4_s8:
    - table[int8x8x4_t]: [
        0_i8, 1, 2, -3, 4, 5, 6, 7,
        8, 9, -10, 11, 12, -13, 14, 15,
        16, -17, 18, 19, 20, 21, 22, 23,
        -24, 25, 26, -27, 28, -29, 30, 31] |
    - ext[int8x8_t]: [50_i8, 51, 52, 53, 54, 55, 56, 57] |
    - ctrl[i8x8]: [3_i8, 31, 17, 22, 10, 29, 7, 15] => [-3_i8, 31, -17, 22, -10, -29, 7, 15] |
    - ctrl[i8x8]: [3_i8, 8, 17, 10, 37, 2, 19, -42] => [-3_i8, 8, -17, -10, 54, 2, 19, 57]
);

test_vtbx!(
    test_vtbx4_u8 => vtbx4_u8:
    - table[uint8x8x4_t]: [
        0_i8, 1, 2, 3, 4, 5, 6, 7,
        8, 9, 10, 11, 12, 13, 14, 15,
        16, 17, 18, 19, 20, 21, 22, 23,
        24, 25, 26, 27, 28, 29, 30, 31] |
    - ext[uint8x8_t]: [50_i8, 51, 52, 53, 54, 55, 56, 57] |
    - ctrl[u8x8]: [3_u8, 31, 17, 22, 10, 29, 7, 15] => [3_i8, 31, 17, 22, 10, 29, 7, 15] |
    - ctrl[u8x8]: [3_u8, 8, 17, 10, 37, 2, 19, 42] => [3_i8, 8, 17, 10, 54, 2, 19, 57]
);

test_vtbx!(
    test_vtbx4_p8 => vtbx4_p8:
    - table[poly8x8x4_t]: [
        0_i8, 1, 2, 3, 4, 5, 6, 7,
        8, 9, 10, 11, 12, 13, 14, 15,
        16, 17, 18, 19, 20, 21, 22, 23,
        24, 25, 26, 27, 28, 29, 30, 31] |
    - ext[poly8x8_t]: [50_i8, 51, 52, 53, 54, 55, 56, 57] |
    - ctrl[u8x8]: [3_u8, 31, 17, 22, 10, 29, 7, 15] => [3_i8, 31, 17, 22, 10, 29, 7, 15] |
    - ctrl[u8x8]: [3_u8, 8, 17, 10, 37, 2, 19, 42] => [3_i8, 8, 17, 10, 54, 2, 19, 57]
);

// Aarch64 tests

#[cfg(target_arch = "aarch64")]
test_vtbl!(
    test_vqtbl1_s8 => vqtbl1_s8:
    - table[int8x16_t]: [
        0_i8, -17, 34, 51, 68, 85, 102, 119,
        -106, -93, -84, -117, -104, -116, -72, -121
    ] |
    - ctrl[i8x8]: [127_i8, 15, 1, 14, 2, 13, 3, 12] => [0_i8, -121, -17, -72, 34, -116, 51, -104] |
    - ctrl[i8x8]: [4_i8, 11, 16, 10, 6, 19, 7, 18] => [68_i8, -117, 0, -84, 102, 0, 119, 0]
);

#[cfg(target_arch = "aarch64")]
test_vtbl!(
    test_vqtbl1q_s8 => vqtbl1q_s8:
    - table[int8x16_t]: [
        0_i8, -17, 34, 51, 68, 85, 102, 119,
        -106, -93, -84, -117, -104, -116, -72, -121
    ] |
    - ctrl[i8x16]: [127_i8, 15, 1, 14, 2, 13, 3, 12, 4_i8, 11, 16, 10, 6, 19, 7, 18]
        => [0_i8, -121, -17, -72, 34, -116, 51, -104, 68, -117, 0, -84, 102, 0, 119, 0]
);

#[cfg(target_arch = "aarch64")]
test_vtbl!(
    test_vqtbl1_u8 => vqtbl1_u8:
    - table[uint8x16_t]: [
        0_u8, 17, 34, 51, 68, 85, 102, 119,
        106, 93, 84, 117, 104, 116, 72, 121
    ] |
    - ctrl[u8x8]: [127_u8, 15, 1, 14, 2, 13, 3, 12] => [0_u8, 121, 17, 72, 34, 116, 51, 104] |
    - ctrl[u8x8]: [4_u8, 11, 16, 10, 6, 19, 7, 18] => [68_u8, 117, 0, 84, 102, 0, 119, 0]
);

#[cfg(target_arch = "aarch64")]
test_vtbl!(
    test_vqtbl1q_u8 => vqtbl1q_u8:
    - table[uint8x16_t]: [
        0_u8, 17, 34, 51, 68, 85, 102, 119,
        106, 93, 84, 117, 104, 116, 72, 121
    ] |
    - ctrl[u8x16]: [127_u8, 15, 1, 14, 2, 13, 3, 12, 4_u8, 11, 16, 10, 6, 19, 7, 18]
        => [0_u8, 121, 17, 72, 34, 116, 51, 104, 68, 117, 0, 84, 102, 0, 119, 0]
);

#[cfg(target_arch = "aarch64")]
test_vtbl!(
    test_vqtbl1_p8 => vqtbl1_p8:
    - table[poly8x16_t]: [
        0_u8, 17, 34, 51, 68, 85, 102, 119,
        106, 93, 84, 117, 104, 116, 72, 121
    ] |
    - ctrl[u8x8]: [127_u8, 15, 1, 14, 2, 13, 3, 12] => [0_u8, 121, 17, 72, 34, 116, 51, 104] |
    - ctrl[u8x8]: [4_u8, 11, 16, 10, 6, 19, 7, 18] => [68_u8, 117, 0, 84, 102, 0, 119, 0]
);

#[cfg(target_arch = "aarch64")]
test_vtbl!(
    test_vqtbl1q_p8 => vqtbl1q_p8:
    - table[poly8x16_t]: [
        0_u8, 17, 34, 51, 68, 85, 102, 119,
        106, 93, 84, 117, 104, 116, 72, 121
    ] |
    - ctrl[u8x16]: [127_u8, 15, 1, 14, 2, 13, 3, 12, 4_u8, 11, 16, 10, 6, 19, 7, 18]
        => [0_u8, 121, 17, 72, 34, 116, 51, 104, 68, 117, 0, 84, 102, 0, 119, 0]
);

#[cfg(target_arch = "aarch64")]
test_vtbl!(
    test_vqtbl2_s8 => vqtbl2_s8:
    - table[int8x16x2_t]: [
        0_i8, -1, 2, -3, 4, -5, 6, -7,
        8, -9, 10, -11, 12, -13, 14, -15,
        16, -17, 18, -19, 20, -21, 22, -23,
        24, -25, 26, -27, 28, -29, 30, -31
    ] |
    - ctrl[i8x8]: [80_i8, 15, 1, 24, 2, 13, 3, 29] => [0_i8, -15, -1, 24, 2, -13, -3, -29] |
    - ctrl[i8x8]: [4_i8, 31, 32, 10, 6, 49, 7, 18] => [4_i8, -31, 0, 10, 6, 0, -7, 18]
);

#[cfg(target_arch = "aarch64")]
test_vtbl!(
    test_vqtbl2q_s8 => vqtbl2q_s8:
    - table[int8x16x2_t]: [
        0_i8, -1, 2, -3, 4, -5, 6, -7,
        8, -9, 10, -11, 12, -13, 14, -15,
        16, -17, 18, -19, 20, -21, 22, -23,
        24, -25, 26, -27, 28, -29, 30, -31
    ] |
    - ctrl[i8x16]: [80_i8, 15, 1, 24, 2, 13, 3, 29, 4_i8, 31, 32, 10, 6, 49, 7, 18]
        => [0_i8, -15, -1, 24, 2, -13, -3, -29, 4, -31, 0, 10, 6, 0, -7, 18]
);

#[cfg(target_arch = "aarch64")]
test_vtbl!(
    test_vqtbl2_u8 => vqtbl2_u8:
    - table[uint8x16x2_t]: [
        0_u8, 1, 2, 3, 4, 5, 6, 7,
        8, 9, 10, 11, 12, 13, 14, 15,
        16, 17, 18, 19, 20, 21, 22, 23,
        24, 25, 26, 27, 28, 29, 30, 31
    ] |
    - ctrl[u8x8]: [80_u8, 15, 1, 24, 2, 13, 3, 29] => [0_u8, 15, 1, 24, 2, 13, 3, 29] |
    - ctrl[u8x8]: [4_u8, 31, 32, 10, 6, 49, 7, 18] => [4_u8, 31, 0, 10, 6, 0, 7, 18]
);

#[cfg(target_arch = "aarch64")]
test_vtbl!(
    test_vqtbl2q_u8 => vqtbl2q_u8:
    - table[uint8x16x2_t]: [
        0_u8, 1, 2, 3, 4, 5, 6, 7,
        8, 9, 10, 11, 12, 13, 14, 15,
        16, 17, 18, 19, 20, 21, 22, 23,
        24, 25, 26, 27, 28, 29, 30, 31
    ] |
    - ctrl[u8x16]: [80_u8, 15, 1, 24, 2, 13, 3, 29, 4_u8, 31, 32, 10, 6, 49, 7, 18]
        => [0_u8, 15, 1, 24, 2, 13, 3, 29, 4, 31, 0, 10, 6, 0, 7, 18]
);

#[cfg(target_arch = "aarch64")]
test_vtbl!(
    test_vqtbl2_p8 => vqtbl2_p8:
    - table[poly8x16x2_t]: [
        0_u8, 1, 2, 3, 4, 5, 6, 7,
        8, 9, 10, 11, 12, 13, 14, 15,
        16, 17, 18, 19, 20, 21, 22, 23,
        24, 25, 26, 27, 28, 29, 30, 31
    ] |
    - ctrl[u8x8]: [80_u8, 15, 1, 24, 2, 13, 3, 29] => [0_u8, 15, 1, 24, 2, 13, 3, 29] |
    - ctrl[u8x8]: [4_u8, 31, 32, 10, 6, 49, 7, 18] => [4_u8, 31, 0, 10, 6, 0, 7, 18]
);

#[cfg(target_arch = "aarch64")]
test_vtbl!(
    test_vqtbl2q_p8 => vqtbl2q_p8:
    - table[poly8x16x2_t]: [
        0_u8, 1, 2, 3, 4, 5, 6, 7,
        8, 9, 10, 11, 12, 13, 14, 15,
        16, 17, 18, 19, 20, 21, 22, 23,
        24, 25, 26, 27, 28, 29, 30, 31
    ] |
    - ctrl[u8x16]: [80_u8, 15, 1, 24, 2, 13, 3, 29, 4_u8, 31, 32, 10, 6, 49, 7, 18]
        => [0_u8, 15, 1, 24, 2, 13, 3, 29, 4, 31, 0, 10, 6, 0, 7, 18]
);

#[cfg(target_arch = "aarch64")]
test_vtbl!(
    test_vqtbl3_s8 => vqtbl3_s8:
    - table[int8x16x3_t]: [
        0_i8, -1, 2, -3, 4, -5, 6, -7,
        8, -9, 10, -11, 12, -13, 14, -15,
        16, -17, 18, -19, 20, -21, 22, -23,
        24, -25, 26, -27, 28, -29, 30, -31,
        32, -33, 34, -35, 36, -37, 38, -39,
        40, -41, 42, -43, 44, -45, 46, -47
    ] |
    - ctrl[i8x8]: [80_i8, 15, 1, 24, 2, 13, 3, 29] => [0_i8, -15, -1, 24, 2, -13, -3, -29] |
    - ctrl[i8x8]: [4_i8, 32, 46, 51, 6, 49, 7, 18] => [4_i8, 32, 46, 0, 6, 0, -7, 18]
);

#[cfg(target_arch = "aarch64")]
test_vtbl!(
    test_vqtbl3q_s8 => vqtbl3q_s8:
    - table[int8x16x3_t]: [
        0_i8, -1, 2, -3, 4, -5, 6, -7,
        8, -9, 10, -11, 12, -13, 14, -15,
        16, -17, 18, -19, 20, -21, 22, -23,
        24, -25, 26, -27, 28, -29, 30, -31,
        32, -33, 34, -35, 36, -37, 38, -39,
        40, -41, 42, -43, 44, -45, 46, -47
    ] |
    - ctrl[i8x16]: [80_i8, 15, 1, 24, 2, 13, 3, 29, 4_i8, 32, 46, 51, 6, 49, 7, 18]
        => [0_i8, -15, -1, 24, 2, -13, -3, -29, 4, 32, 46, 0, 6, 0, -7, 18]
);

#[cfg(target_arch = "aarch64")]
test_vtbl!(
    test_vqtbl3_u8 => vqtbl3_u8:
    - table[uint8x16x3_t]: [
        0_u8, 1, 2, 3, 4, 5, 6, 7,
        8, 9, 10, 11, 12, 13, 14, 15,
        16, 17, 18, 19, 20, 21, 22, 23,
        24, 25, 26, 27, 28, 29, 30, 31,
        32, 33, 34, 35, 36, 37, 38, 39,
        40, 41, 42, 43, 44, 45, 46, 47
    ] |
    - ctrl[u8x8]: [80_u8, 15, 1, 24, 2, 13, 3, 29] => [0_u8, 15, 1, 24, 2, 13, 3, 29] |
    - ctrl[u8x8]: [4_u8, 32, 46, 51, 6, 49, 7, 18] => [4_u8, 32, 46, 0, 6, 0, 7, 18]
);

#[cfg(target_arch = "aarch64")]
test_vtbl!(
    test_vqtbl3q_u8 => vqtbl3q_u8:
    - table[uint8x16x3_t]: [
        0_u8, 1, 2, 3, 4, 5, 6, 7,
        8, 9, 10, 11, 12, 13, 14, 15,
        16, 17, 18, 19, 20, 21, 22, 23,
        24, 25, 26, 27, 28, 29, 30, 31,
        32, 33, 34, 35, 36, 37, 38, 39,
        40, 41, 42, 43, 44, 45, 46, 47
    ] |
    - ctrl[u8x16]: [80_u8, 15, 1, 24, 2, 13, 3, 29, 4_u8, 32, 46, 51, 6, 49, 7, 18]
        => [0_u8, 15, 1, 24, 2, 13, 3, 29, 4, 32, 46, 0, 6, 0, 7, 18]
);

#[cfg(target_arch = "aarch64")]
test_vtbl!(
    test_vqtbl3_p8 => vqtbl3_p8:
    - table[poly8x16x3_t]: [
        0_u8, 1, 2, 3, 4, 5, 6, 7,
        8, 9, 10, 11, 12, 13, 14, 15,
        16, 17, 18, 19, 20, 21, 22, 23,
        24, 25, 26, 27, 28, 29, 30, 31,
        32, 33, 34, 35, 36, 37, 38, 39,
        40, 41, 42, 43, 44, 45, 46, 47
    ] |
    - ctrl[u8x8]: [80_u8, 15, 1, 24, 2, 13, 3, 29] => [0_u8, 15, 1, 24, 2, 13, 3, 29] |
    - ctrl[u8x8]: [4_u8, 32, 46, 51, 6, 49, 7, 18] => [4_u8, 32, 46, 0, 6, 0, 7, 18]
);

#[cfg(target_arch = "aarch64")]
test_vtbl!(
    test_vqtbl3q_p8 => vqtbl3q_p8:
    - table[poly8x16x3_t]: [
        0_u8, 1, 2, 3, 4, 5, 6, 7,
        8, 9, 10, 11, 12, 13, 14, 15,
        16, 17, 18, 19, 20, 21, 22, 23,
        24, 25, 26, 27, 28, 29, 30, 31,
        32, 33, 34, 35, 36, 37, 38, 39,
        40, 41, 42, 43, 44, 45, 46, 47
    ] |
    - ctrl[u8x16]: [80_u8, 15, 1, 24, 2, 13, 3, 29, 4_u8, 32, 46, 51, 6, 49, 7, 18]
        => [0_u8, 15, 1, 24, 2, 13, 3, 29, 4, 32, 46, 0, 6, 0, 7, 18]
);

#[cfg(target_arch = "aarch64")]
test_vtbl!(
    test_vqtbl4_s8 => vqtbl4_s8:
    - table[int8x16x4_t]: [
        0_i8, -1, 2, -3, 4, -5, 6, -7,
        8, -9, 10, -11, 12, -13, 14, -15,
        16, -17, 18, -19, 20, -21, 22, -23,
        24, -25, 26, -27, 28, -29, 30, -31,
        32, -33, 34, -35, 36, -37, 38, -39,
        40, -41, 42, -43, 44, -45, 46, -47,
        48, -49, 50, -51, 52, -53, 54, -55,
        56, -57, 58, -59, 60, -61, 62, -63
    ] |
    - ctrl[i8x8]: [80_i8, 15, 1, 24, 2, 13, 3, 29] => [0_i8, -15, -1, 24, 2, -13, -3, -29] |
    - ctrl[i8x8]: [4_i8, 46, 64, 51, 6, 71, 7, 18] => [4_i8, 46, 0, -51, 6, 0, -7, 18]
);

#[cfg(target_arch = "aarch64")]
test_vtbl!(
    test_vqtbl4q_s8 => vqtbl4q_s8:
    - table[int8x16x4_t]: [
        0_i8, -1, 2, -3, 4, -5, 6, -7,
        8, -9, 10, -11, 12, -13, 14, -15,
        16, -17, 18, -19, 20, -21, 22, -23,
        24, -25, 26, -27, 28, -29, 30, -31,
        32, -33, 34, -35, 36, -37, 38, -39,
        40, -41, 42, -43, 44, -45, 46, -47,
        48, -49, 50, -51, 52, -53, 54, -55,
        56, -57, 58, -59, 60, -61, 62, -63
    ] |
    - ctrl[i8x16]: [80_i8, 15, 1, 24, 2, 13, 3, 29, 4_i8, 46, 64, 51, 6, 71, 7, 18]
        => [0_i8, -15, -1, 24, 2, -13, -3, -29, 4, 46, 0, -51, 6, 0, -7, 18]
);

#[cfg(target_arch = "aarch64")]
test_vtbl!(
    test_vqtbl4_u8 => vqtbl4_u8:
    - table[uint8x16x4_t]: [
        0_u8, 1, 2, 3, 4, 5, 6, 7,
        8, 9, 10, 11, 12, 13, 14, 15,
        16, 17, 18, 19, 20, 21, 22, 23,
        24, 25, 26, 27, 28, 29, 30, 31,
        32, 33, 34, 35, 36, 37, 38, 39,
        40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52, 53, 54, 55,
        56, 57, 58, 59, 60, 61, 62, 63
    ] |
    - ctrl[u8x8]: [80_u8, 15, 1, 24, 2, 13, 3, 29] => [0_u8, 15, 1, 24, 2, 13, 3, 29] |
    - ctrl[u8x8]: [4_u8, 46, 64, 51, 6, 71, 7, 18] => [4_u8, 46, 0, 51, 6, 0, 7, 18]
);

#[cfg(target_arch = "aarch64")]
test_vtbl!(
    test_vqtbl4q_u8 => vqtbl4q_u8:
    - table[uint8x16x4_t]: [
        0_u8, 1, 2, 3, 4, 5, 6, 7,
        8, 9, 10, 11, 12, 13, 14, 15,
        16, 17, 18, 19, 20, 21, 22, 23,
        24, 25, 26, 27, 28, 29, 30, 31,
        32, 33, 34, 35, 36, 37, 38, 39,
        40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52, 53, 54, 55,
        56, 57, 58, 59, 60, 61, 62, 63
    ] |
    - ctrl[u8x16]: [80_u8, 15, 1, 24, 2, 13, 3, 29, 4_u8, 46, 64, 51, 6, 71, 7, 18]
        => [0_u8, 15, 1, 24, 2, 13, 3, 29, 4, 46, 0, 51, 6, 0, 7, 18]
);

#[cfg(target_arch = "aarch64")]
test_vtbl!(
    test_vqtbl4_p8 => vqtbl4_p8:
    - table[poly8x16x4_t]: [
        0_u8, 1, 2, 3, 4, 5, 6, 7,
        8, 9, 10, 11, 12, 13, 14, 15,
        16, 17, 18, 19, 20, 21, 22, 23,
        24, 25, 26, 27, 28, 29, 30, 31,
        32, 33, 34, 35, 36, 37, 38, 39,
        40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52, 53, 54, 55,
        56, 57, 58, 59, 60, 61, 62, 63
    ] |
    - ctrl[u8x8]: [80_u8, 15, 1, 24, 2, 13, 3, 29] => [0_u8, 15, 1, 24, 2, 13, 3, 29] |
    - ctrl[u8x8]: [4_u8, 46, 64, 51, 6, 71, 7, 18] => [4_u8, 46, 0, 51, 6, 0, 7, 18]
);

#[cfg(target_arch = "aarch64")]
test_vtbl!(
    test_vqtbl4q_p8 => vqtbl4q_p8:
    - table[poly8x16x4_t]: [
        0_u8, 1, 2, 3, 4, 5, 6, 7,
        8, 9, 10, 11, 12, 13, 14, 15,
        16, 17, 18, 19, 20, 21, 22, 23,
        24, 25, 26, 27, 28, 29, 30, 31,
        32, 33, 34, 35, 36, 37, 38, 39,
        40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52, 53, 54, 55,
        56, 57, 58, 59, 60, 61, 62, 63
    ] |
    - ctrl[u8x16]: [80_u8, 15, 1, 24, 2, 13, 3, 29, 4_u8, 46, 64, 51, 6, 71, 7, 18]
        => [0_u8, 15, 1, 24, 2, 13, 3, 29, 4, 46, 0, 51, 6, 0, 7, 18]
);

#[cfg(target_arch = "aarch64")]
test_vtbx!(
    test_vqtbx1_s8 => vqtbx1_s8:
    - table[int8x16_t]: [
        0_i8, -17, 34, 51, 68, 85, 102, 119,
        -106, -93, -84, -117, -104, -116, -72, -121
    ] |
    - ext[int8x8_t]: [100_i8, -101, 102, -103, 104, -105, 106, -107] |
    - ctrl[i8x8]: [127_i8, 15, 1, 14, 2, 13, 3, 12] => [100_i8, -121, -17, -72, 34, -116, 51, -104] |
    - ctrl[i8x8]: [4_i8, 11, 16, 10, 6, 19, 7, 18] => [68_i8, -117, 102, -84, 102, -105, 119, -107]
);

#[cfg(target_arch = "aarch64")]
test_vtbx!(
    test_vqtbx1q_s8 => vqtbx1q_s8:
    - table[int8x16_t]: [
        0_i8, -17, 34, 51, 68, 85, 102, 119,
        -106, -93, -84, -117, -104, -116, -72, -121
    ] |
    - ext[int8x16_t]: [
        100_i8, -101, 102, -103, 104, -105, 106, -107,
        108, -109, 110, -111, 112, -113, 114, -115
    ] |
    - ctrl[i8x16]: [127_i8, 15, 1, 14, 2, 13, 3, 12, 4_i8, 11, 16, 10, 6, 19, 7, 18]
        => [100_i8, -121, -17, -72, 34, -116, 51, -104, 68, -117, 110, -84, 102, -113, 119, -115]
);

#[cfg(target_arch = "aarch64")]
test_vtbx!(
    test_vqtbx1_u8 => vqtbx1_u8:
    - table[uint8x16_t]: [
        0_u8, 17, 34, 51, 68, 85, 102, 119,
        106, 93, 84, 117, 104, 116, 72, 121
    ] |
    - ext[uint8x8_t]: [100_u8, 101, 102, 103, 104, 105, 106, 107] |
    - ctrl[u8x8]: [127_u8, 15, 1, 14, 2, 13, 3, 12] => [100_u8, 121, 17, 72, 34, 116, 51, 104] |
    - ctrl[u8x8]: [4_u8, 11, 16, 10, 6, 19, 7, 18] => [68_u8, 117, 102, 84, 102, 105, 119, 107]
);

#[cfg(target_arch = "aarch64")]
test_vtbx!(
    test_vqtbx1q_u8 => vqtbx1q_u8:
    - table[uint8x16_t]: [
        0_u8, 17, 34, 51, 68, 85, 102, 119,
        106, 93, 84, 117, 104, 116, 72, 121
    ] |
    - ext[uint8x16_t]: [
        100_u8, 101, 102, 103, 104, 105, 106, 107,
        108, 109, 110, 111, 112, 113, 114, 115
    ] |
    - ctrl[u8x16]: [127_u8, 15, 1, 14, 2, 13, 3, 12, 4_u8, 11, 16, 10, 6, 19, 7, 18]
        => [100_u8, 121, 17, 72, 34, 116, 51, 104, 68, 117, 110, 84, 102, 113, 119, 115]
);

#[cfg(target_arch = "aarch64")]
test_vtbx!(
    test_vqtbx1_p8 => vqtbx1_p8:
    - table[poly8x16_t]: [
        0_u8, 17, 34, 51, 68, 85, 102, 119,
        106, 93, 84, 117, 104, 116, 72, 121
    ] |
    - ext[poly8x8_t]: [100_u8, 101, 102, 103, 104, 105, 106, 107] |
    - ctrl[u8x8]: [127_u8, 15, 1, 14, 2, 13, 3, 12] => [100_u8, 121, 17, 72, 34, 116, 51, 104] |
    - ctrl[u8x8]: [4_u8, 11, 16, 10, 6, 19, 7, 18] => [68_u8, 117, 102, 84, 102, 105, 119, 107]
);

#[cfg(target_arch = "aarch64")]
test_vtbx!(
    test_vqtbx1q_p8 => vqtbx1q_p8:
    - table[poly8x16_t]: [
        0_u8, 17, 34, 51, 68, 85, 102, 119,
        106, 93, 84, 117, 104, 116, 72, 121
    ] |
    - ext[poly8x16_t]: [
        100_u8, 101, 102, 103, 104, 105, 106, 107,
        108, 109, 110, 111, 112, 113, 114, 115
    ] |
    - ctrl[u8x16]: [127_u8, 15, 1, 14, 2, 13, 3, 12, 4_u8, 11, 16, 10, 6, 19, 7, 18]
        => [100_u8, 121, 17, 72, 34, 116, 51, 104, 68, 117, 110, 84, 102, 113, 119, 115]
);

#[cfg(target_arch = "aarch64")]
test_vtbx!(
    test_vqtbx2_s8 => vqtbx2_s8:
    - table[int8x16x2_t]: [
        0_i8, -1, 2, -3, 4, -5, 6, -7,
        8, -9, 10, -11, 12, -13, 14, -15,
        16, -17, 18, -19, 20, -21, 22, -23,
        24, -25, 26, -27, 28, -29, 30, -31
    ] |
    - ext[int8x8_t]: [100_i8, -101, 102, -103, 104, -105, 106, -107] |
    - ctrl[i8x8]: [80_i8, 15, 1, 24, 2, 13, 3, 29] => [100_i8, -15, -1, 24, 2, -13, -3, -29] |
    - ctrl[i8x8]: [4_i8, 31, 32, 10, 6, 49, 7, 18] => [4_i8, -31, 102, 10, 6, -105, -7, 18]
);

#[cfg(target_arch = "aarch64")]
test_vtbx!(
    test_vqtbx2q_s8 => vqtbx2q_s8:
    - table[int8x16x2_t]: [
        0_i8, -1, 2, -3, 4, -5, 6, -7,
        8, -9, 10, -11, 12, -13, 14, -15,
        16, -17, 18, -19, 20, -21, 22, -23,
        24, -25, 26, -27, 28, -29, 30, -31
    ] |
    - ext[int8x16_t]: [
        100_i8, -101, 102, -103, 104, -105, 106, -107,
        108, -109, 110, -111, 112, -113, 114, -115
    ] |
    - ctrl[i8x16]: [80_i8, 15, 1, 24, 2, 13, 3, 29, 4_i8, 31, 32, 10, 6, 49, 7, 18]
        => [100_i8, -15, -1, 24, 2, -13, -3, -29, 4, -31, 110, 10, 6, -113, -7, 18]
);

#[cfg(target_arch = "aarch64")]
test_vtbx!(
    test_vqtbx2_u8 => vqtbx2_u8:
    - table[uint8x16x2_t]: [
        0_u8, 1, 2, 3, 4, 5, 6, 7,
        8, 9, 10, 11, 12, 13, 14, 15,
        16, 17, 18, 19, 20, 21, 22, 23,
        24, 25, 26, 27, 28, 29, 30, 31
    ] |
    - ext[uint8x8_t]: [100_u8, 101, 102, 103, 104, 105, 106, 107] |
    - ctrl[u8x8]: [80_u8, 15, 1, 24, 2, 13, 3, 29] => [100_u8, 15, 1, 24, 2, 13, 3, 29] |
    - ctrl[u8x8]: [4_u8, 31, 32, 10, 6, 49, 7, 18] => [4_u8, 31, 102, 10, 6, 105, 7, 18]
);

#[cfg(target_arch = "aarch64")]
test_vtbx!(
    test_vqtbx2q_u8 => vqtbx2q_u8:
    - table[uint8x16x2_t]: [
        0_u8, 1, 2, 3, 4, 5, 6, 7,
        8, 9, 10, 11, 12, 13, 14, 15,
        16, 17, 18, 19, 20, 21, 22, 23,
        24, 25, 26, 27, 28, 29, 30, 31
    ] |
    - ext[uint8x16_t]: [
        100_u8, 101, 102, 103, 104, 105, 106, 107,
        108, 109, 110, 111, 112, 113, 114, 115
    ] |
    - ctrl[u8x16]: [80_u8, 15, 1, 24, 2, 13, 3, 29, 4_u8, 31, 32, 10, 6, 49, 7, 18]
        => [100_u8, 15, 1, 24, 2, 13, 3, 29, 4, 31, 110, 10, 6, 113, 7, 18]
);

#[cfg(target_arch = "aarch64")]
test_vtbx!(
    test_vqtbx2_p8 => vqtbx2_p8:
    - table[poly8x16x2_t]: [
        0_u8, 1, 2, 3, 4, 5, 6, 7,
        8, 9, 10, 11, 12, 13, 14, 15,
        16, 17, 18, 19, 20, 21, 22, 23,
        24, 25, 26, 27, 28, 29, 30, 31
    ] |
    - ext[poly8x8_t]: [100_u8, 101, 102, 103, 104, 105, 106, 107] |
    - ctrl[u8x8]: [80_u8, 15, 1, 24, 2, 13, 3, 29] => [100_u8, 15, 1, 24, 2, 13, 3, 29] |
    - ctrl[u8x8]: [4_u8, 31, 32, 10, 6, 49, 7, 18] => [4_u8, 31, 102, 10, 6, 105, 7, 18]
);

#[cfg(target_arch = "aarch64")]
test_vtbx!(
    test_vqtbx2q_p8 => vqtbx2q_p8:
    - table[poly8x16x2_t]: [
        0_u8, 1, 2, 3, 4, 5, 6, 7,
        8, 9, 10, 11, 12, 13, 14, 15,
        16, 17, 18, 19, 20, 21, 22, 23,
        24, 25, 26, 27, 28, 29, 30, 31
    ] |
    - ext[poly8x16_t]: [
        100_u8, 101, 102, 103, 104, 105, 106, 107,
        108, 109, 110, 111, 112, 113, 114, 115
    ] |
    - ctrl[u8x16]: [80_u8, 15, 1, 24, 2, 13, 3, 29, 4_u8, 31, 32, 10, 6, 49, 7, 18]
        => [100_u8, 15, 1, 24, 2, 13, 3, 29, 4, 31, 110, 10, 6, 113, 7, 18]
);

#[cfg(target_arch = "aarch64")]
test_vtbx!(
    test_vqtbx3_s8 => vqtbx3_s8:
    - table[int8x16x3_t]: [
        0_i8, -1, 2, -3, 4, -5, 6, -7,
        8, -9, 10, -11, 12, -13, 14, -15,
        16, -17, 18, -19, 20, -21, 22, -23,
        24, -25, 26, -27, 28, -29, 30, -31,
        32, -33, 34, -35, 36, -37, 38, -39,
        40, -41, 42, -43, 44, -45, 46, -47
    ] |
    - ext[int8x8_t]: [100_i8, -101, 102, -103, 104, -105, 106, -107] |
    - ctrl[i8x8]: [80_i8, 15, 1, 24, 2, 13, 3, 29] => [100_i8, -15, -1, 24, 2, -13, -3, -29] |
    - ctrl[i8x8]: [4_i8, 32, 46, 51, 6, 49, 7, 18] => [4_i8, 32, 46, -103, 6, -105, -7, 18]
);

#[cfg(target_arch = "aarch64")]
test_vtbx!(
    test_vqtbx3q_s8 => vqtbx3q_s8:
    - table[int8x16x3_t]: [
        0_i8, -1, 2, -3, 4, -5, 6, -7,
        8, -9, 10, -11, 12, -13, 14, -15,
        16, -17, 18, -19, 20, -21, 22, -23,
        24, -25, 26, -27, 28, -29, 30, -31,
        32, -33, 34, -35, 36, -37, 38, -39,
        40, -41, 42, -43, 44, -45, 46, -47
    ] |
    - ext[int8x16_t]: [
        100_i8, -101, 102, -103, 104, -105, 106, -107,
        108, -109, 110, -111, 112, -113, 114, -115
    ] |
    - ctrl[i8x16]: [80_i8, 15, 1, 24, 2, 13, 3, 29, 4_i8, 32, 46, 51, 6, 49, 7, 18]
        => [100_i8, -15, -1, 24, 2, -13, -3, -29, 4, 32, 46, -111, 6, -113, -7, 18]
);

#[cfg(target_arch = "aarch64")]
test_vtbx!(
    test_vqtbx3_u8 => vqtbx3_u8:
    - table[uint8x16x3_t]: [
        0_u8, 1, 2, 3, 4, 5, 6, 7,
        8, 9, 10, 11, 12, 13, 14, 15,
        16, 17, 18, 19, 20, 21, 22, 23,
        24, 25, 26, 27, 28, 29, 30, 31,
        32, 33, 34, 35, 36, 37, 38, 39,
        40, 41, 42, 43, 44, 45, 46, 47
    ] |
    - ext[uint8x8_t]: [100_u8, 101, 102, 103, 104, 105, 106, 107] |
    - ctrl[u8x8]: [80_u8, 15, 1, 24, 2, 13, 3, 29] => [100_u8, 15, 1, 24, 2, 13, 3, 29] |
    - ctrl[u8x8]: [4_u8, 32, 46, 51, 6, 49, 7, 18] => [4_u8, 32, 46, 103, 6, 105, 7, 18]
);

#[cfg(target_arch = "aarch64")]
test_vtbx!(
    test_vqtbx3q_u8 => vqtbx3q_u8:
    - table[uint8x16x3_t]: [
        0_u8, 1, 2, 3, 4, 5, 6, 7,
        8, 9, 10, 11, 12, 13, 14, 15,
        16, 17, 18, 19, 20, 21, 22, 23,
        24, 25, 26, 27, 28, 29, 30, 31,
        32, 33, 34, 35, 36, 37, 38, 39,
        40, 41, 42, 43, 44, 45, 46, 47
    ] |
    - ext[uint8x16_t]: [
        100_u8, 101, 102, 103, 104, 105, 106, 107,
        108, 109, 110, 111, 112, 113, 114, 115
    ] |
    - ctrl[u8x16]: [80_u8, 15, 1, 24, 2, 13, 3, 29, 4_u8, 32, 46, 51, 6, 49, 7, 18]
        => [100_u8, 15, 1, 24, 2, 13, 3, 29, 4, 32, 46, 111, 6, 113, 7, 18]
);

#[cfg(target_arch = "aarch64")]
test_vtbx!(
    test_vqtbx3_p8 => vqtbx3_p8:
    - table[poly8x16x3_t]: [
        0_u8, 1, 2, 3, 4, 5, 6, 7,
        8, 9, 10, 11, 12, 13, 14, 15,
        16, 17, 18, 19, 20, 21, 22, 23,
        24, 25, 26, 27, 28, 29, 30, 31,
        32, 33, 34, 35, 36, 37, 38, 39,
        40, 41, 42, 43, 44, 45, 46, 47
    ] |
    - ext[poly8x8_t]: [100_u8, 101, 102, 103, 104, 105, 106, 107] |
    - ctrl[u8x8]: [80_u8, 15, 1, 24, 2, 13, 3, 29] => [100_u8, 15, 1, 24, 2, 13, 3, 29] |
    - ctrl[u8x8]: [4_u8, 32, 46, 51, 6, 49, 7, 18] => [4_u8, 32, 46, 103, 6, 105, 7, 18]
);

#[cfg(target_arch = "aarch64")]
test_vtbx!(
    test_vqtbx3q_p8 => vqtbx3q_p8:
    - table[poly8x16x3_t]: [
        0_u8, 1, 2, 3, 4, 5, 6, 7,
        8, 9, 10, 11, 12, 13, 14, 15,
        16, 17, 18, 19, 20, 21, 22, 23,
        24, 25, 26, 27, 28, 29, 30, 31,
        32, 33, 34, 35, 36, 37, 38, 39,
        40, 41, 42, 43, 44, 45, 46, 47
    ] |
    - ext[poly8x16_t]: [
        100_u8, 101, 102, 103, 104, 105, 106, 107,
        108, 109, 110, 111, 112, 113, 114, 115
    ] |
    - ctrl[u8x16]: [80_u8, 15, 1, 24, 2, 13, 3, 29, 4_u8, 32, 46, 51, 6, 49, 7, 18]
        => [100_u8, 15, 1, 24, 2, 13, 3, 29, 4, 32, 46, 111, 6, 113, 7, 18]
);

#[cfg(target_arch = "aarch64")]
test_vtbx!(
    test_vqtbx4_s8 => vqtbx4_s8:
    - table[int8x16x4_t]: [
        0_i8, -1, 2, -3, 4, -5, 6, -7,
        8, -9, 10, -11, 12, -13, 14, -15,
        16, -17, 18, -19, 20, -21, 22, -23,
        24, -25, 26, -27, 28, -29, 30, -31,
        32, -33, 34, -35, 36, -37, 38, -39,
        40, -41, 42, -43, 44, -45, 46, -47,
        48, -49, 50, -51, 52, -53, 54, -55,
        56, -57, 58, -59, 60, -61, 62, -63
    ] |
    - ext[int8x8_t]: [100_i8, -101, 102, -103, 104, -105, 106, -107] |
    - ctrl[i8x8]: [80_i8, 15, 1, 24, 2, 13, 3, 29] => [100_i8, -15, -1, 24, 2, -13, -3, -29] |
    - ctrl[i8x8]: [4_i8, 46, 64, 51, 6, 71, 7, 18] => [4_i8, 46, 102, -51, 6, -105, -7, 18]
);

#[cfg(target_arch = "aarch64")]
test_vtbx!(
    test_vqtbx4q_s8 => vqtbx4q_s8:
    - table[int8x16x4_t]: [
        0_i8, -1, 2, -3, 4, -5, 6, -7,
        8, -9, 10, -11, 12, -13, 14, -15,
        16, -17, 18, -19, 20, -21, 22, -23,
        24, -25, 26, -27, 28, -29, 30, -31,
        32, -33, 34, -35, 36, -37, 38, -39,
        40, -41, 42, -43, 44, -45, 46, -47,
        48, -49, 50, -51, 52, -53, 54, -55,
        56, -57, 58, -59, 60, -61, 62, -63
    ] |
    - ext[int8x16_t]: [
        100_i8, -101, 102, -103, 104, -105, 106, -107,
        108, -109, 110, -111, 112, -113, 114, -115
    ] |
    - ctrl[i8x16]: [80_i8, 15, 1, 24, 2, 13, 3, 29, 4_i8, 46, 64, 51, 6, 71, 7, 18]
        => [100_i8, -15, -1, 24, 2, -13, -3, -29, 4, 46, 110, -51, 6, -113, -7, 18]
);

#[cfg(target_arch = "aarch64")]
test_vtbx!(
    test_vqtbx4_u8 => vqtbx4_u8:
    - table[uint8x16x4_t]: [
        0_u8, 1, 2, 3, 4, 5, 6, 7,
        8, 9, 10, 11, 12, 13, 14, 15,
        16, 17, 18, 19, 20, 21, 22, 23,
        24, 25, 26, 27, 28, 29, 30, 31,
        32, 33, 34, 35, 36, 37, 38, 39,
        40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52, 53, 54, 55,
        56, 57, 58, 59, 60, 61, 62, 63
    ] |
    - ext[uint8x8_t]: [100_u8, 101, 102, 103, 104, 105, 106, 107] |
    - ctrl[u8x8]: [80_u8, 15, 1, 24, 2, 13, 3, 29] => [100_u8, 15, 1, 24, 2, 13, 3, 29] |
    - ctrl[u8x8]: [4_u8, 46, 64, 51, 6, 71, 7, 18] => [4_u8, 46, 102, 51, 6, 105, 7, 18]
);

#[cfg(target_arch = "aarch64")]
test_vtbx!(
    test_vqtbx4q_u8 => vqtbx4q_u8:
    - table[uint8x16x4_t]: [
        0_u8, 1, 2, 3, 4, 5, 6, 7,
        8, 9, 10, 11, 12, 13, 14, 15,
        16, 17, 18, 19, 20, 21, 22, 23,
        24, 25, 26, 27, 28, 29, 30, 31,
        32, 33, 34, 35, 36, 37, 38, 39,
        40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52, 53, 54, 55,
        56, 57, 58, 59, 60, 61, 62, 63
    ] |
    - ext[uint8x16_t]: [
        100_u8, 101, 102, 103, 104, 105, 106, 107,
        108, 109, 110, 111, 112, 113, 114, 115
    ] |
    - ctrl[u8x16]: [80_u8, 15, 1, 24, 2, 13, 3, 29, 4_u8, 46, 64, 51, 6, 71, 7, 18]
        => [100_u8, 15, 1, 24, 2, 13, 3, 29, 4, 46, 110, 51, 6, 113, 7, 18]
);

#[cfg(target_arch = "aarch64")]
test_vtbx!(
    test_vqtbx4_p8 => vqtbx4_p8:
    - table[poly8x16x4_t]: [
        0_u8, 1, 2, 3, 4, 5, 6, 7,
        8, 9, 10, 11, 12, 13, 14, 15,
        16, 17, 18, 19, 20, 21, 22, 23,
        24, 25, 26, 27, 28, 29, 30, 31,
        32, 33, 34, 35, 36, 37, 38, 39,
        40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52, 53, 54, 55,
        56, 57, 58, 59, 60, 61, 62, 63
    ] |
    - ext[poly8x8_t]: [100_u8, 101, 102, 103, 104, 105, 106, 107] |
    - ctrl[u8x8]: [80_u8, 15, 1, 24, 2, 13, 3, 29] => [100_u8, 15, 1, 24, 2, 13, 3, 29] |
    - ctrl[u8x8]: [4_u8, 46, 64, 51, 6, 71, 7, 18] => [4_u8, 46, 102, 51, 6, 105, 7, 18]
);

#[cfg(target_arch = "aarch64")]
test_vtbx!(
    test_vqtbx4q_p8 => vqtbx4q_p8:
    - table[poly8x16x4_t]: [
        0_u8, 1, 2, 3, 4, 5, 6, 7,
        8, 9, 10, 11, 12, 13, 14, 15,
        16, 17, 18, 19, 20, 21, 22, 23,
        24, 25, 26, 27, 28, 29, 30, 31,
        32, 33, 34, 35, 36, 37, 38, 39,
        40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52, 53, 54, 55,
        56, 57, 58, 59, 60, 61, 62, 63
    ] |
    - ext[poly8x16_t]: [
        100_u8, 101, 102, 103, 104, 105, 106, 107,
        108, 109, 110, 111, 112, 113, 114, 115
    ] |
    - ctrl[u8x16]: [80_u8, 15, 1, 24, 2, 13, 3, 29, 4_u8, 46, 64, 51, 6, 71, 7, 18]
        => [100_u8, 15, 1, 24, 2, 13, 3, 29, 4, 46, 110, 51, 6, 113, 7, 18]
);