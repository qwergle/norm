#![allow(clippy::single_range_in_vec_init)]

mod fzf_common;

use fzf_common as common;
use norm::fzf::FzfV1;

#[test]
fn fzf_v1_upstream_empty() {
    common::upstream_empty::<FzfV1>();
}

#[test]
fn fzf_v1_upstream_exact_1() {
    common::upstream_exact_1::<FzfV1>();
}

#[test]
fn fzf_v1_upstream_exact_2() {
    common::upstream_exact_2::<FzfV1>();
}

#[test]
fn fzf_v1_upstream_exact_3() {
    common::upstream_exact_3::<FzfV1>()
}

#[test]
fn fzf_v1_upstream_exact_4() {
    common::upstream_exact_4::<FzfV1>()
}

#[test]
fn fzf_v1_upstream_exact_5() {
    common::upstream_exact_5::<FzfV1>()
}

#[test]
fn fzf_v1_upstream_exact_6() {
    common::upstream_exact_6::<FzfV1>()
}

#[test]
fn fzf_v1_upstream_exact_7() {
    common::upstream_exact_7::<FzfV1>()
}

#[test]
fn fzf_v1_upstream_fuzzy_1() {
    common::upstream_fuzzy_1::<FzfV1>();
}

#[test]
fn fzf_v1_upstream_fuzzy_2() {
    common::upstream_fuzzy_2::<FzfV1>();
}

#[test]
fn fzf_v1_upstream_fuzzy_3() {
    common::upstream_fuzzy_3::<FzfV1>()
}

#[test]
fn fzf_v1_upstream_fuzzy_4() {
    common::upstream_fuzzy_4::<FzfV1>()
}

#[test]
fn fzf_v1_upstream_fuzzy_5() {
    common::upstream_fuzzy_5::<FzfV1>()
}

#[test]
fn fzf_v1_upstream_fuzzy_6() {
    common::upstream_fuzzy_6::<FzfV1>()
}

#[test]
fn fzf_v1_upstream_fuzzy_7() {
    common::upstream_fuzzy_7::<FzfV1>()
}

#[test]
fn fzf_v1_upstream_fuzzy_8() {
    common::upstream_fuzzy_8::<FzfV1>();
}

#[test]
fn fzf_v1_upstream_fuzzy_9() {
    common::upstream_fuzzy_9::<FzfV1>();
}

#[test]
fn fzf_v1_upstream_fuzzy_10() {
    common::upstream_fuzzy_10::<FzfV1>();
}

#[test]
fn fzf_v1_upstream_fuzzy_11() {
    common::upstream_fuzzy_11::<FzfV1>();
}

#[test]
fn fzf_v1_upstream_fuzzy_12() {
    common::upstream_fuzzy_12::<FzfV1>();
}

#[test]
fn fzf_v1_upstream_fuzzy_13() {
    common::upstream_fuzzy_13::<FzfV1>();
}

#[test]
fn fzf_v1_upstream_fuzzy_14() {
    common::upstream_fuzzy_14::<FzfV1>();
}

#[test]
fn fzf_v1_upstream_fuzzy_15() {
    common::upstream_fuzzy_15::<FzfV1>();
}

#[test]
fn fzf_v1_upstream_fuzzy_16() {
    common::upstream_fuzzy_16::<FzfV1>();
}

#[test]
fn fzf_v1_upstream_fuzzy_17() {
    common::upstream_fuzzy_17::<FzfV1>();
}

#[test]
fn fzf_v1_upstream_fuzzy_18() {
    common::upstream_fuzzy_18::<FzfV1>();
}

#[test]
fn fzf_v1_upstream_fuzzy_19() {
    common::upstream_fuzzy_19::<FzfV1>();
}

#[test]
fn fzf_v1_upstream_fuzzy_20() {
    common::upstream_fuzzy_20::<FzfV1>();
}

#[test]
fn fzf_v1_upstream_prefix_1() {
    common::upstream_prefix_1::<FzfV1>();
}

#[test]
fn fzf_v1_upstream_prefix_2() {
    common::upstream_prefix_2::<FzfV1>();
}

#[test]
fn fzf_v1_upstream_prefix_3() {
    common::upstream_prefix_3::<FzfV1>()
}

#[test]
fn fzf_v1_upstream_prefix_4() {
    common::upstream_prefix_4::<FzfV1>()
}

#[test]
fn fzf_v1_upstream_prefix_5() {
    common::upstream_prefix_5::<FzfV1>()
}

#[test]
fn fzf_v1_upstream_prefix_6() {
    common::upstream_prefix_6::<FzfV1>()
}

#[test]
fn fzf_v1_upstream_prefix_7() {
    common::upstream_prefix_7::<FzfV1>()
}

#[test]
fn fzf_v1_upstream_prefix_8() {
    common::upstream_prefix_8::<FzfV1>()
}

#[test]
fn fzf_v1_upstream_suffix_1() {
    common::upstream_suffix_1::<FzfV1>();
}

#[test]
fn fzf_v1_upstream_suffix_2() {
    common::upstream_suffix_2::<FzfV1>();
}

#[test]
fn fzf_v1_upstream_suffix_3() {
    common::upstream_suffix_3::<FzfV1>()
}

#[test]
fn fzf_v1_upstream_suffix_4() {
    common::upstream_suffix_4::<FzfV1>()
}

#[test]
fn fzf_v1_upstream_suffix_5() {
    common::upstream_suffix_5::<FzfV1>()
}

#[test]
fn fzf_v1_upstream_suffix_6() {
    common::upstream_suffix_6::<FzfV1>()
}
