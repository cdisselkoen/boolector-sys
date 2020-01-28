/* automatically generated by rust-bindgen */

use libc::FILE;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Btor {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BtorNode {
    _unused: [u8; 0],
}
pub const BtorSolverResult_BTOR_RESULT_SAT: BtorSolverResult = 10;
pub const BtorSolverResult_BTOR_RESULT_UNSAT: BtorSolverResult = 20;
pub const BtorSolverResult_BTOR_RESULT_UNKNOWN: BtorSolverResult = 0;
pub type BtorSolverResult = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BoolectorNode {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BoolectorAnonymous {
    _unused: [u8; 0],
}
pub type BoolectorSort = *mut BoolectorAnonymous;
pub const BtorOption_BTOR_OPT_MODEL_GEN: BtorOption = 0;
pub const BtorOption_BTOR_OPT_INCREMENTAL: BtorOption = 1;
pub const BtorOption_BTOR_OPT_INCREMENTAL_SMT1: BtorOption = 2;
pub const BtorOption_BTOR_OPT_INPUT_FORMAT: BtorOption = 3;
pub const BtorOption_BTOR_OPT_OUTPUT_NUMBER_FORMAT: BtorOption = 4;
pub const BtorOption_BTOR_OPT_OUTPUT_FORMAT: BtorOption = 5;
pub const BtorOption_BTOR_OPT_ENGINE: BtorOption = 6;
pub const BtorOption_BTOR_OPT_SAT_ENGINE: BtorOption = 7;
pub const BtorOption_BTOR_OPT_AUTO_CLEANUP: BtorOption = 8;
pub const BtorOption_BTOR_OPT_PRETTY_PRINT: BtorOption = 9;
pub const BtorOption_BTOR_OPT_EXIT_CODES: BtorOption = 10;
pub const BtorOption_BTOR_OPT_SEED: BtorOption = 11;
pub const BtorOption_BTOR_OPT_VERBOSITY: BtorOption = 12;
pub const BtorOption_BTOR_OPT_LOGLEVEL: BtorOption = 13;
pub const BtorOption_BTOR_OPT_REWRITE_LEVEL: BtorOption = 14;
pub const BtorOption_BTOR_OPT_SKELETON_PREPROC: BtorOption = 15;
pub const BtorOption_BTOR_OPT_ACKERMANN: BtorOption = 16;
pub const BtorOption_BTOR_OPT_BETA_REDUCE: BtorOption = 17;
pub const BtorOption_BTOR_OPT_ELIMINATE_SLICES: BtorOption = 18;
pub const BtorOption_BTOR_OPT_VAR_SUBST: BtorOption = 19;
pub const BtorOption_BTOR_OPT_UCOPT: BtorOption = 20;
pub const BtorOption_BTOR_OPT_MERGE_LAMBDAS: BtorOption = 21;
pub const BtorOption_BTOR_OPT_EXTRACT_LAMBDAS: BtorOption = 22;
pub const BtorOption_BTOR_OPT_NORMALIZE: BtorOption = 23;
pub const BtorOption_BTOR_OPT_NORMALIZE_ADD: BtorOption = 24;
pub const BtorOption_BTOR_OPT_FUN_PREPROP: BtorOption = 25;
pub const BtorOption_BTOR_OPT_FUN_PRESLS: BtorOption = 26;
pub const BtorOption_BTOR_OPT_FUN_DUAL_PROP: BtorOption = 27;
pub const BtorOption_BTOR_OPT_FUN_DUAL_PROP_QSORT: BtorOption = 28;
pub const BtorOption_BTOR_OPT_FUN_JUST: BtorOption = 29;
pub const BtorOption_BTOR_OPT_FUN_JUST_HEURISTIC: BtorOption = 30;
pub const BtorOption_BTOR_OPT_FUN_LAZY_SYNTHESIZE: BtorOption = 31;
pub const BtorOption_BTOR_OPT_FUN_EAGER_LEMMAS: BtorOption = 32;
pub const BtorOption_BTOR_OPT_FUN_STORE_LAMBDAS: BtorOption = 33;
pub const BtorOption_BTOR_OPT_SLS_NFLIPS: BtorOption = 34;
pub const BtorOption_BTOR_OPT_SLS_STRATEGY: BtorOption = 35;
pub const BtorOption_BTOR_OPT_SLS_JUST: BtorOption = 36;
pub const BtorOption_BTOR_OPT_SLS_MOVE_GW: BtorOption = 37;
pub const BtorOption_BTOR_OPT_SLS_MOVE_RANGE: BtorOption = 38;
pub const BtorOption_BTOR_OPT_SLS_MOVE_SEGMENT: BtorOption = 39;
pub const BtorOption_BTOR_OPT_SLS_MOVE_RAND_WALK: BtorOption = 40;
pub const BtorOption_BTOR_OPT_SLS_PROB_MOVE_RAND_WALK: BtorOption = 41;
pub const BtorOption_BTOR_OPT_SLS_MOVE_RAND_ALL: BtorOption = 42;
pub const BtorOption_BTOR_OPT_SLS_MOVE_RAND_RANGE: BtorOption = 43;
pub const BtorOption_BTOR_OPT_SLS_MOVE_PROP: BtorOption = 44;
pub const BtorOption_BTOR_OPT_SLS_MOVE_PROP_N_PROP: BtorOption = 45;
pub const BtorOption_BTOR_OPT_SLS_MOVE_PROP_N_SLS: BtorOption = 46;
pub const BtorOption_BTOR_OPT_SLS_MOVE_PROP_FORCE_RW: BtorOption = 47;
pub const BtorOption_BTOR_OPT_SLS_MOVE_INC_MOVE_TEST: BtorOption = 48;
pub const BtorOption_BTOR_OPT_SLS_USE_RESTARTS: BtorOption = 49;
pub const BtorOption_BTOR_OPT_SLS_USE_BANDIT: BtorOption = 50;
pub const BtorOption_BTOR_OPT_PROP_NPROPS: BtorOption = 51;
pub const BtorOption_BTOR_OPT_PROP_USE_RESTARTS: BtorOption = 52;
pub const BtorOption_BTOR_OPT_PROP_USE_BANDIT: BtorOption = 53;
pub const BtorOption_BTOR_OPT_PROP_PATH_SEL: BtorOption = 54;
pub const BtorOption_BTOR_OPT_PROP_PROB_USE_INV_VALUE: BtorOption = 55;
pub const BtorOption_BTOR_OPT_PROP_PROB_FLIP_COND: BtorOption = 56;
pub const BtorOption_BTOR_OPT_PROP_PROB_FLIP_COND_CONST: BtorOption = 57;
pub const BtorOption_BTOR_OPT_PROP_FLIP_COND_CONST_DELTA: BtorOption = 58;
pub const BtorOption_BTOR_OPT_PROP_FLIP_COND_CONST_NPATHSEL: BtorOption = 59;
pub const BtorOption_BTOR_OPT_PROP_PROB_SLICE_KEEP_DC: BtorOption = 60;
pub const BtorOption_BTOR_OPT_PROP_PROB_CONC_FLIP: BtorOption = 61;
pub const BtorOption_BTOR_OPT_PROP_PROB_SLICE_FLIP: BtorOption = 62;
pub const BtorOption_BTOR_OPT_PROP_PROB_EQ_FLIP: BtorOption = 63;
pub const BtorOption_BTOR_OPT_PROP_PROB_AND_FLIP: BtorOption = 64;
pub const BtorOption_BTOR_OPT_PROP_NO_MOVE_ON_CONFLICT: BtorOption = 65;
pub const BtorOption_BTOR_OPT_AIGPROP_USE_RESTARTS: BtorOption = 66;
pub const BtorOption_BTOR_OPT_AIGPROP_USE_BANDIT: BtorOption = 67;
pub const BtorOption_BTOR_OPT_QUANT_SYNTH: BtorOption = 68;
pub const BtorOption_BTOR_OPT_QUANT_DUAL_SOLVER: BtorOption = 69;
pub const BtorOption_BTOR_OPT_QUANT_SYNTH_LIMIT: BtorOption = 70;
pub const BtorOption_BTOR_OPT_QUANT_SYNTH_QI: BtorOption = 71;
pub const BtorOption_BTOR_OPT_QUANT_DER: BtorOption = 72;
pub const BtorOption_BTOR_OPT_QUANT_CER: BtorOption = 73;
pub const BtorOption_BTOR_OPT_QUANT_MINISCOPE: BtorOption = 74;
pub const BtorOption_BTOR_OPT_SORT_EXP: BtorOption = 75;
pub const BtorOption_BTOR_OPT_SORT_AIG: BtorOption = 76;
pub const BtorOption_BTOR_OPT_SORT_AIGVEC: BtorOption = 77;
pub const BtorOption_BTOR_OPT_AUTO_CLEANUP_INTERNAL: BtorOption = 78;
pub const BtorOption_BTOR_OPT_SIMPLIFY_CONSTRAINTS: BtorOption = 79;
pub const BtorOption_BTOR_OPT_CHK_FAILED_ASSUMPTIONS: BtorOption = 80;
pub const BtorOption_BTOR_OPT_CHK_MODEL: BtorOption = 81;
pub const BtorOption_BTOR_OPT_CHK_UNCONSTRAINED: BtorOption = 82;
pub const BtorOption_BTOR_OPT_PARSE_INTERACTIVE: BtorOption = 83;
pub const BtorOption_BTOR_OPT_SAT_ENGINE_LGL_FORK: BtorOption = 84;
pub const BtorOption_BTOR_OPT_SAT_ENGINE_CADICAL_FREEZE: BtorOption = 85;
pub const BtorOption_BTOR_OPT_SAT_ENGINE_N_THREADS: BtorOption = 86;
pub const BtorOption_BTOR_OPT_SIMP_NORMAMLIZE_ADDERS: BtorOption = 87;
pub const BtorOption_BTOR_OPT_DECLSORT_BV_WIDTH: BtorOption = 88;
pub const BtorOption_BTOR_OPT_QUANT_SYNTH_ITE_COMPLETE: BtorOption = 89;
pub const BtorOption_BTOR_OPT_QUANT_FIXSYNTH: BtorOption = 90;
pub const BtorOption_BTOR_OPT_RW_ZERO_LOWER_SLICE: BtorOption = 91;
pub const BtorOption_BTOR_OPT_NONDESTR_SUBST: BtorOption = 92;
pub const BtorOption_BTOR_OPT_NUM_OPTS: BtorOption = 93;
pub type BtorOption = u32;
pub const BtorOptBetaReduceMode_BTOR_BETA_REDUCE_NONE: BtorOptBetaReduceMode = 0;
pub const BtorOptBetaReduceMode_BTOR_BETA_REDUCE_FUN: BtorOptBetaReduceMode = 1;
pub const BtorOptBetaReduceMode_BTOR_BETA_REDUCE_ALL: BtorOptBetaReduceMode = 2;
pub type BtorOptBetaReduceMode = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BtorAbortCallback {
    pub abort_fun: ::std::option::Option<unsafe extern "C" fn(msg: *const ::std::os::raw::c_char)>,
    pub cb_fun: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_BtorAbortCallback() {
    assert_eq!(
        ::std::mem::size_of::<BtorAbortCallback>(),
        16usize,
        concat!("Size of: ", stringify!(BtorAbortCallback))
    );
    assert_eq!(
        ::std::mem::align_of::<BtorAbortCallback>(),
        8usize,
        concat!("Alignment of ", stringify!(BtorAbortCallback))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<BtorAbortCallback>())).abort_fun as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(BtorAbortCallback),
            "::",
            stringify!(abort_fun)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<BtorAbortCallback>())).cb_fun as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(BtorAbortCallback),
            "::",
            stringify!(cb_fun)
        )
    );
}
extern "C" {
    pub fn boolector_new() -> *mut Btor;
}
extern "C" {
    pub fn boolector_clone(btor: *mut Btor) -> *mut Btor;
}
extern "C" {
    pub fn boolector_delete(btor: *mut Btor);
}
extern "C" {
    pub fn boolector_set_term(
        btor: *mut Btor,
        fun: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> i32>,
        state: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn boolector_terminate(btor: *mut Btor) -> i32;
}
extern "C" {
    pub fn boolector_set_abort(
        fun: ::std::option::Option<unsafe extern "C" fn(msg: *const ::std::os::raw::c_char)>,
    );
}
extern "C" {
    pub fn boolector_set_msg_prefix(btor: *mut Btor, prefix: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn boolector_get_refs(btor: *mut Btor) -> u32;
}
extern "C" {
    pub fn boolector_reset_time(btor: *mut Btor);
}
extern "C" {
    pub fn boolector_reset_stats(btor: *mut Btor);
}
extern "C" {
    pub fn boolector_print_stats(btor: *mut Btor);
}
extern "C" {
    pub fn boolector_set_trapi(btor: *mut Btor, apitrace: *mut FILE);
}
extern "C" {
    pub fn boolector_get_trapi(btor: *mut Btor) -> *mut FILE;
}
extern "C" {
    pub fn boolector_push(btor: *mut Btor, level: u32);
}
extern "C" {
    pub fn boolector_pop(btor: *mut Btor, level: u32);
}
extern "C" {
    pub fn boolector_assert(btor: *mut Btor, node: *mut BoolectorNode);
}
extern "C" {
    pub fn boolector_assume(btor: *mut Btor, node: *mut BoolectorNode);
}
extern "C" {
    pub fn boolector_failed(btor: *mut Btor, node: *mut BoolectorNode) -> bool;
}
extern "C" {
    pub fn boolector_get_failed_assumptions(btor: *mut Btor) -> *mut *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_fixate_assumptions(btor: *mut Btor);
}
extern "C" {
    pub fn boolector_reset_assumptions(btor: *mut Btor);
}
extern "C" {
    pub fn boolector_sat(btor: *mut Btor) -> i32;
}
extern "C" {
    pub fn boolector_limited_sat(btor: *mut Btor, lod_limit: i32, sat_limit: i32) -> i32;
}
extern "C" {
    pub fn boolector_simplify(btor: *mut Btor) -> i32;
}
extern "C" {
    pub fn boolector_set_sat_solver(btor: *mut Btor, solver: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn boolector_set_opt(btor: *mut Btor, opt: BtorOption, val: u32);
}
extern "C" {
    pub fn boolector_get_opt(btor: *mut Btor, opt: BtorOption) -> u32;
}
extern "C" {
    pub fn boolector_get_opt_min(btor: *mut Btor, opt: BtorOption) -> u32;
}
extern "C" {
    pub fn boolector_get_opt_max(btor: *mut Btor, opt: BtorOption) -> u32;
}
extern "C" {
    pub fn boolector_get_opt_dflt(btor: *mut Btor, opt: BtorOption) -> u32;
}
extern "C" {
    pub fn boolector_get_opt_lng(btor: *mut Btor, opt: BtorOption)
        -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn boolector_get_opt_shrt(
        btor: *mut Btor,
        opt: BtorOption,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn boolector_get_opt_desc(
        btor: *mut Btor,
        opt: BtorOption,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn boolector_has_opt(Btor: *mut Btor, opt: BtorOption) -> bool;
}
extern "C" {
    pub fn boolector_first_opt(btor: *mut Btor) -> BtorOption;
}
extern "C" {
    pub fn boolector_next_opt(btor: *mut Btor, opt: BtorOption) -> BtorOption;
}
extern "C" {
    pub fn boolector_copy(btor: *mut Btor, node: *mut BoolectorNode) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_release(btor: *mut Btor, node: *mut BoolectorNode);
}
extern "C" {
    pub fn boolector_release_all(btor: *mut Btor);
}
extern "C" {
    pub fn boolector_true(btor: *mut Btor) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_false(btor: *mut Btor) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_implies(
        btor: *mut Btor,
        n0: *mut BoolectorNode,
        n1: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_iff(
        btor: *mut Btor,
        n0: *mut BoolectorNode,
        n1: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_eq(
        btor: *mut Btor,
        n0: *mut BoolectorNode,
        n1: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_ne(
        btor: *mut Btor,
        n0: *mut BoolectorNode,
        n1: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_is_bv_const_zero(btor: *mut Btor, node: *mut BoolectorNode) -> bool;
}
extern "C" {
    pub fn boolector_is_bv_const_one(btor: *mut Btor, node: *mut BoolectorNode) -> bool;
}
extern "C" {
    pub fn boolector_is_bv_const_ones(btor: *mut Btor, node: *mut BoolectorNode) -> bool;
}
extern "C" {
    pub fn boolector_is_bv_const_max_signed(btor: *mut Btor, node: *mut BoolectorNode) -> bool;
}
extern "C" {
    pub fn boolector_is_bv_const_min_signed(btor: *mut Btor, node: *mut BoolectorNode) -> bool;
}
extern "C" {
    pub fn boolector_const(
        btor: *mut Btor,
        bits: *const ::std::os::raw::c_char,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_constd(
        btor: *mut Btor,
        sort: BoolectorSort,
        str: *const ::std::os::raw::c_char,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_consth(
        btor: *mut Btor,
        sort: BoolectorSort,
        str: *const ::std::os::raw::c_char,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_zero(btor: *mut Btor, sort: BoolectorSort) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_ones(btor: *mut Btor, sort: BoolectorSort) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_one(btor: *mut Btor, sort: BoolectorSort) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_min_signed(btor: *mut Btor, sort: BoolectorSort) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_max_signed(btor: *mut Btor, sort: BoolectorSort) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_unsigned_int(
        btor: *mut Btor,
        u: u32,
        sort: BoolectorSort,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_int(btor: *mut Btor, i: i32, sort: BoolectorSort) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_var(
        btor: *mut Btor,
        sort: BoolectorSort,
        symbol: *const ::std::os::raw::c_char,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_array(
        btor: *mut Btor,
        sort: BoolectorSort,
        symbol: *const ::std::os::raw::c_char,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_const_array(
        btor: *mut Btor,
        sort: BoolectorSort,
        value: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_uf(
        btor: *mut Btor,
        sort: BoolectorSort,
        symbol: *const ::std::os::raw::c_char,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_not(btor: *mut Btor, node: *mut BoolectorNode) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_neg(btor: *mut Btor, node: *mut BoolectorNode) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_redor(btor: *mut Btor, node: *mut BoolectorNode) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_redxor(btor: *mut Btor, node: *mut BoolectorNode) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_redand(btor: *mut Btor, node: *mut BoolectorNode) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_slice(
        btor: *mut Btor,
        node: *mut BoolectorNode,
        upper: u32,
        lower: u32,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_uext(
        btor: *mut Btor,
        node: *mut BoolectorNode,
        width: u32,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_sext(
        btor: *mut Btor,
        node: *mut BoolectorNode,
        width: u32,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_xor(
        btor: *mut Btor,
        n0: *mut BoolectorNode,
        n1: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_xnor(
        btor: *mut Btor,
        n0: *mut BoolectorNode,
        n1: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_and(
        btor: *mut Btor,
        n0: *mut BoolectorNode,
        n1: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_nand(
        btor: *mut Btor,
        n0: *mut BoolectorNode,
        n1: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_or(
        btor: *mut Btor,
        n0: *mut BoolectorNode,
        n1: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_nor(
        btor: *mut Btor,
        n0: *mut BoolectorNode,
        n1: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_add(
        btor: *mut Btor,
        n0: *mut BoolectorNode,
        n1: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_uaddo(
        btor: *mut Btor,
        n0: *mut BoolectorNode,
        n1: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_saddo(
        btor: *mut Btor,
        n0: *mut BoolectorNode,
        n1: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_mul(
        btor: *mut Btor,
        n0: *mut BoolectorNode,
        n1: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_umulo(
        btor: *mut Btor,
        n0: *mut BoolectorNode,
        n1: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_smulo(
        btor: *mut Btor,
        n0: *mut BoolectorNode,
        n1: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_ult(
        btor: *mut Btor,
        n0: *mut BoolectorNode,
        n1: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_slt(
        btor: *mut Btor,
        n0: *mut BoolectorNode,
        n1: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_ulte(
        btor: *mut Btor,
        n0: *mut BoolectorNode,
        n1: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_slte(
        btor: *mut Btor,
        n0: *mut BoolectorNode,
        n1: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_ugt(
        btor: *mut Btor,
        n0: *mut BoolectorNode,
        n1: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_sgt(
        btor: *mut Btor,
        n0: *mut BoolectorNode,
        n1: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_ugte(
        btor: *mut Btor,
        n0: *mut BoolectorNode,
        n1: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_sgte(
        btor: *mut Btor,
        n0: *mut BoolectorNode,
        n1: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_sll(
        btor: *mut Btor,
        n0: *mut BoolectorNode,
        n1: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_srl(
        btor: *mut Btor,
        n0: *mut BoolectorNode,
        n1: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_sra(
        btor: *mut Btor,
        n0: *mut BoolectorNode,
        n1: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_rol(
        btor: *mut Btor,
        n0: *mut BoolectorNode,
        n1: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_ror(
        btor: *mut Btor,
        n0: *mut BoolectorNode,
        n1: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_roli(btor: *mut Btor, n: *mut BoolectorNode, nbits: u32)
        -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_rori(btor: *mut Btor, n: *mut BoolectorNode, nbits: u32)
        -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_sub(
        btor: *mut Btor,
        n0: *mut BoolectorNode,
        n1: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_usubo(
        btor: *mut Btor,
        n0: *mut BoolectorNode,
        n1: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_ssubo(
        btor: *mut Btor,
        n0: *mut BoolectorNode,
        n1: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_udiv(
        btor: *mut Btor,
        n0: *mut BoolectorNode,
        n1: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_sdiv(
        btor: *mut Btor,
        n0: *mut BoolectorNode,
        n1: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_sdivo(
        btor: *mut Btor,
        n0: *mut BoolectorNode,
        n1: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_urem(
        btor: *mut Btor,
        n0: *mut BoolectorNode,
        n1: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_srem(
        btor: *mut Btor,
        n0: *mut BoolectorNode,
        n1: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_smod(
        btor: *mut Btor,
        n0: *mut BoolectorNode,
        n1: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_concat(
        btor: *mut Btor,
        n0: *mut BoolectorNode,
        n1: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_repeat(
        btor: *mut Btor,
        node: *mut BoolectorNode,
        n: u32,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_read(
        btor: *mut Btor,
        n_array: *mut BoolectorNode,
        n_index: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_write(
        btor: *mut Btor,
        n_array: *mut BoolectorNode,
        n_index: *mut BoolectorNode,
        n_value: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_cond(
        btor: *mut Btor,
        n_cond: *mut BoolectorNode,
        n_then: *mut BoolectorNode,
        n_else: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_param(
        btor: *mut Btor,
        sort: BoolectorSort,
        symbol: *const ::std::os::raw::c_char,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_fun(
        btor: *mut Btor,
        param_nodes: *mut *mut BoolectorNode,
        paramc: u32,
        node: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_apply(
        btor: *mut Btor,
        arg_nodes: *mut *mut BoolectorNode,
        argc: u32,
        n_fun: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_inc(btor: *mut Btor, node: *mut BoolectorNode) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_dec(btor: *mut Btor, node: *mut BoolectorNode) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_forall(
        btor: *mut Btor,
        params: *mut *mut BoolectorNode,
        paramc: u32,
        body: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_exists(
        btor: *mut Btor,
        param: *mut *mut BoolectorNode,
        paramc: u32,
        body: *mut BoolectorNode,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_get_btor(node: *mut BoolectorNode) -> *mut Btor;
}
extern "C" {
    pub fn boolector_get_node_id(btor: *mut Btor, node: *mut BoolectorNode) -> i32;
}
extern "C" {
    pub fn boolector_get_sort(btor: *mut Btor, node: *const BoolectorNode) -> BoolectorSort;
}
extern "C" {
    pub fn boolector_fun_get_domain_sort(
        btor: *mut Btor,
        node: *const BoolectorNode,
    ) -> BoolectorSort;
}
extern "C" {
    pub fn boolector_fun_get_codomain_sort(
        btor: *mut Btor,
        node: *const BoolectorNode,
    ) -> BoolectorSort;
}
extern "C" {
    pub fn boolector_match_node_by_id(btor: *mut Btor, id: i32) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_match_node_by_symbol(
        btor: *mut Btor,
        symbol: *const ::std::os::raw::c_char,
    ) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_match_node(btor: *mut Btor, node: *mut BoolectorNode) -> *mut BoolectorNode;
}
extern "C" {
    pub fn boolector_get_symbol(
        btor: *mut Btor,
        node: *mut BoolectorNode,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn boolector_set_symbol(
        btor: *mut Btor,
        node: *mut BoolectorNode,
        symbol: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn boolector_get_width(btor: *mut Btor, node: *mut BoolectorNode) -> u32;
}
extern "C" {
    pub fn boolector_get_index_width(btor: *mut Btor, n_array: *mut BoolectorNode) -> u32;
}
extern "C" {
    pub fn boolector_get_bits(
        btor: *mut Btor,
        node: *mut BoolectorNode,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn boolector_free_bits(btor: *mut Btor, bits: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn boolector_get_fun_arity(btor: *mut Btor, node: *mut BoolectorNode) -> u32;
}
extern "C" {
    pub fn boolector_is_const(btor: *mut Btor, node: *mut BoolectorNode) -> bool;
}
extern "C" {
    pub fn boolector_is_var(btor: *mut Btor, node: *mut BoolectorNode) -> bool;
}
extern "C" {
    pub fn boolector_is_array(btor: *mut Btor, node: *mut BoolectorNode) -> bool;
}
extern "C" {
    pub fn boolector_is_array_var(btor: *mut Btor, node: *mut BoolectorNode) -> bool;
}
extern "C" {
    pub fn boolector_is_param(btor: *mut Btor, node: *mut BoolectorNode) -> bool;
}
extern "C" {
    pub fn boolector_is_bound_param(btor: *mut Btor, node: *mut BoolectorNode) -> bool;
}
extern "C" {
    pub fn boolector_is_uf(btor: *mut Btor, node: *mut BoolectorNode) -> bool;
}
extern "C" {
    pub fn boolector_is_fun(btor: *mut Btor, node: *mut BoolectorNode) -> bool;
}
extern "C" {
    pub fn boolector_fun_sort_check(
        btor: *mut Btor,
        arg_nodes: *mut *mut BoolectorNode,
        argc: u32,
        n_fun: *mut BoolectorNode,
    ) -> i32;
}
extern "C" {
    pub fn boolector_bv_assignment(
        btor: *mut Btor,
        node: *mut BoolectorNode,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn boolector_free_bv_assignment(btor: *mut Btor, assignment: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn boolector_array_assignment(
        btor: *mut Btor,
        n_array: *mut BoolectorNode,
        indices: *mut *mut *mut ::std::os::raw::c_char,
        values: *mut *mut *mut ::std::os::raw::c_char,
        size: *mut u32,
    );
}
extern "C" {
    pub fn boolector_free_array_assignment(
        btor: *mut Btor,
        indices: *mut *mut ::std::os::raw::c_char,
        values: *mut *mut ::std::os::raw::c_char,
        size: u32,
    );
}
extern "C" {
    pub fn boolector_uf_assignment(
        btor: *mut Btor,
        n_uf: *mut BoolectorNode,
        args: *mut *mut *mut ::std::os::raw::c_char,
        values: *mut *mut *mut ::std::os::raw::c_char,
        size: *mut u32,
    );
}
extern "C" {
    pub fn boolector_free_uf_assignment(
        btor: *mut Btor,
        args: *mut *mut ::std::os::raw::c_char,
        values: *mut *mut ::std::os::raw::c_char,
        size: u32,
    );
}
extern "C" {
    pub fn boolector_print_model(
        btor: *mut Btor,
        format: *mut ::std::os::raw::c_char,
        file: *mut FILE,
    );
}
extern "C" {
    pub fn boolector_bool_sort(btor: *mut Btor) -> BoolectorSort;
}
extern "C" {
    pub fn boolector_bitvec_sort(btor: *mut Btor, width: u32) -> BoolectorSort;
}
extern "C" {
    pub fn boolector_fun_sort(
        btor: *mut Btor,
        domain: *mut BoolectorSort,
        arity: u32,
        codomain: BoolectorSort,
    ) -> BoolectorSort;
}
extern "C" {
    pub fn boolector_array_sort(
        btor: *mut Btor,
        index: BoolectorSort,
        element: BoolectorSort,
    ) -> BoolectorSort;
}
extern "C" {
    pub fn boolector_copy_sort(btor: *mut Btor, sort: BoolectorSort) -> BoolectorSort;
}
extern "C" {
    pub fn boolector_release_sort(btor: *mut Btor, sort: BoolectorSort);
}
extern "C" {
    pub fn boolector_is_equal_sort(
        btor: *mut Btor,
        n0: *mut BoolectorNode,
        n1: *mut BoolectorNode,
    ) -> bool;
}
extern "C" {
    pub fn boolector_is_array_sort(btor: *mut Btor, sort: BoolectorSort) -> bool;
}
extern "C" {
    pub fn boolector_is_bitvec_sort(btor: *mut Btor, sort: BoolectorSort) -> bool;
}
extern "C" {
    pub fn boolector_is_fun_sort(btor: *mut Btor, sort: BoolectorSort) -> bool;
}
extern "C" {
    pub fn boolector_bitvec_sort_get_width(btor: *mut Btor, sort: BoolectorSort) -> u32;
}
extern "C" {
    pub fn boolector_parse(
        btor: *mut Btor,
        infile: *mut FILE,
        infile_name: *const ::std::os::raw::c_char,
        outfile: *mut FILE,
        error_msg: *mut *mut ::std::os::raw::c_char,
        status: *mut i32,
        parsed_smt2: *mut bool,
    ) -> i32;
}
extern "C" {
    pub fn boolector_parse_btor(
        btor: *mut Btor,
        infile: *mut FILE,
        infile_name: *const ::std::os::raw::c_char,
        outfile: *mut FILE,
        error_msg: *mut *mut ::std::os::raw::c_char,
        status: *mut i32,
    ) -> i32;
}
extern "C" {
    pub fn boolector_parse_btor2(
        btor: *mut Btor,
        infile: *mut FILE,
        infile_name: *const ::std::os::raw::c_char,
        outfile: *mut FILE,
        error_msg: *mut *mut ::std::os::raw::c_char,
        status: *mut i32,
    ) -> i32;
}
extern "C" {
    pub fn boolector_parse_smt1(
        btor: *mut Btor,
        infile: *mut FILE,
        infile_name: *const ::std::os::raw::c_char,
        outfile: *mut FILE,
        error_msg: *mut *mut ::std::os::raw::c_char,
        status: *mut i32,
    ) -> i32;
}
extern "C" {
    pub fn boolector_parse_smt2(
        btor: *mut Btor,
        infile: *mut FILE,
        infile_name: *const ::std::os::raw::c_char,
        outfile: *mut FILE,
        error_msg: *mut *mut ::std::os::raw::c_char,
        status: *mut i32,
    ) -> i32;
}
extern "C" {
    pub fn boolector_dump_btor_node(btor: *mut Btor, file: *mut FILE, node: *mut BoolectorNode);
}
extern "C" {
    pub fn boolector_dump_btor(btor: *mut Btor, file: *mut FILE);
}
extern "C" {
    pub fn boolector_dump_smt2_node(btor: *mut Btor, file: *mut FILE, node: *mut BoolectorNode);
}
extern "C" {
    pub fn boolector_dump_smt2(btor: *mut Btor, file: *mut FILE);
}
extern "C" {
    pub fn boolector_dump_aiger_ascii(btor: *mut Btor, file: *mut FILE, merge_roots: bool);
}
extern "C" {
    pub fn boolector_dump_aiger_binary(btor: *mut Btor, file: *mut FILE, merge_roots: bool);
}
extern "C" {
    pub fn boolector_copyright(btor: *mut Btor) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn boolector_version(btor: *mut Btor) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn boolector_git_id(btor: *mut Btor) -> *const ::std::os::raw::c_char;
}
