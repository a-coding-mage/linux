/* SPDX-License-Identifier: GPL-2.0 */

// The C header guard and HYP_EVENT_MULTI_READ conditional are preserved here
// as source-level conditional intent; Rust module inclusion supplies the guard.

// Under __KVM_NVHE_HYPERVISOR__, the C source includes <nvhe/trace.h>.
// The corresponding tracing definitions are supplied by the surrounding build.

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum hyp_enter_exit_reason {
    HYP_REASON_SMC,
    HYP_REASON_HVC,
    HYP_REASON_SYS,
    HYP_REASON_PSCI,
    HYP_REASON_HOST_ABORT,
    HYP_REASON_GUEST_EXIT,
    HYP_REASON_ERET_HOST,
    HYP_REASON_ERET_GUEST,
    HYP_REASON_UNKNOWN, // Must be last
}

HYP_EVENT!(hyp_enter,
    HE_PROTO!(host_ctxt: *mut kvm_cpu_context, reason: u8),
    HE_STRUCT!(
        he_field!(u8, reason),
        he_field!(pid_t, vcpu)
    ),
    HE_ASSIGN!({
        __entry.reason = reason;
        __entry.vcpu = __tracing_get_vcpu_pid(host_ctxt);
    }),
    HE_PRINTK!("reason=%s vcpu=%d", __hyp_enter_exit_reason_str(__entry.reason), __entry.vcpu)
);

HYP_EVENT!(hyp_exit,
    HE_PROTO!(host_ctxt: *mut kvm_cpu_context, reason: u8),
    HE_STRUCT!(
        he_field!(u8, reason),
        he_field!(pid_t, vcpu)
    ),
    HE_ASSIGN!({
        __entry.reason = reason;
        __entry.vcpu = __tracing_get_vcpu_pid(host_ctxt);
    }),
    HE_PRINTK!("reason=%s vcpu=%d", __hyp_enter_exit_reason_str(__entry.reason), __entry.vcpu)
);

HYP_EVENT!(selftest,
    HE_PROTO!(id: u64),
    HE_STRUCT!(
        he_field!(u64, id)
    ),
    HE_ASSIGN!({
        __entry.id = id;
    }),
    RE_PRINTK!("id=%llu", __entry.id)
);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
