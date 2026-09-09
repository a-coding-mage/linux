/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding action API and UAPI headers.

#[repr(C)]
pub struct tcf_gact {
    pub common: tc_action,
    // Preserved from CONFIG_GACT_PROB; enable the corresponding Rust feature
    // when this configuration is present.
    #[cfg(feature = "CONFIG_GACT_PROB")]
    pub tcfg_ptype: u16,
    #[cfg(feature = "CONFIG_GACT_PROB")]
    pub tcfg_pval: u16,
    #[cfg(feature = "CONFIG_GACT_PROB")]
    pub tcfg_paction: i32,
    #[cfg(feature = "CONFIG_GACT_PROB")]
    pub packets: core::sync::atomic::AtomicI32,
}

#[inline]
pub unsafe fn to_gact(a: *mut tc_action) -> *mut tcf_gact {
    a.cast::<tcf_gact>()
}

#[inline]
pub unsafe fn __is_tcf_gact_act(a: *const tc_action, act: i32, is_ext: bool) -> bool {
    // CONFIG_NET_CLS_ACT controls this body in the C header.
    #[cfg(feature = "CONFIG_NET_CLS_ACT")]
    {
        let gact: *const tcf_gact;

        if !(*a).ops.is_null() && (*(*a).ops).id != TCA_ID_GACT {
            return false;
        }

        gact = to_gact(a as *mut tc_action);
        if ((!is_ext && (*gact).common.tcf_action == act)
            || (is_ext && TC_ACT_EXT_CMP((*gact).common.tcf_action, act)))
        {
            return true;
        }
    }
    false
}

#[inline]
pub unsafe fn is_tcf_gact_ok(a: *const tc_action) -> bool {
    __is_tcf_gact_act(a, TC_ACT_OK, false)
}

#[inline]
pub unsafe fn is_tcf_gact_shot(a: *const tc_action) -> bool {
    __is_tcf_gact_act(a, TC_ACT_SHOT, false)
}

#[inline]
pub unsafe fn is_tcf_gact_trap(a: *const tc_action) -> bool {
    __is_tcf_gact_act(a, TC_ACT_TRAP, false)
}

#[inline]
pub unsafe fn is_tcf_gact_goto_chain(a: *const tc_action) -> bool {
    __is_tcf_gact_act(a, TC_ACT_GOTO_CHAIN, true)
}

#[inline]
pub unsafe fn tcf_gact_goto_chain_index(a: *const tc_action) -> u32 {
    core::ptr::read_volatile(&(*a).tcfa_action) & TC_ACT_EXT_VAL_MASK
}

#[inline]
pub unsafe fn is_tcf_gact_continue(a: *const tc_action) -> bool {
    __is_tcf_gact_act(a, TC_ACT_UNSPEC, false)
}

#[inline]
pub unsafe fn is_tcf_gact_reclassify(a: *const tc_action) -> bool {
    __is_tcf_gact_act(a, TC_ACT_RECLASSIFY, false)
}

#[inline]
pub unsafe fn is_tcf_gact_pipe(a: *const tc_action) -> bool {
    __is_tcf_gact_act(a, TC_ACT_PIPE, false)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
