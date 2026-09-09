/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies supplied by the surrounding kernel translation unit:
// linux/tracepoint.h, asm/sie.h, asm/debug.h, asm/dis.h

// TRACE_SYSTEM = kvm
// TRACE_INCLUDE_PATH = .
// TRACE_INCLUDE_FILE = trace

/*
 * Helpers for vcpu-specific tracepoints containing the same information
 * as s390dbf VCPU_EVENTs.
 */
macro_rules! VCPU_PROTO_COMMON {
    () => { struct kvm_vcpu *vcpu };
}
macro_rules! VCPU_ARGS_COMMON {
    () => { vcpu };
}
macro_rules! VCPU_FIELD_COMMON {
    () => {
        __field!(c_int, id);
        __field!(c_ulong, pswmask);
        __field!(c_ulong, pswaddr);
    };
}
macro_rules! VCPU_ASSIGN_COMMON {
    () => {{
        __entry.id = vcpu.vcpu_id;
        __entry.pswmask = vcpu.arch.sie_block.gpsw.mask;
        __entry.pswaddr = vcpu.arch.sie_block.gpsw.addr;
    }};
}
macro_rules! VCPU_TP_PRINTK {
    ($p_str:expr $(, $p_args:expr)*) => {
        TP_printk!(concat!("%02d[%016lx-%016lx]: ", $p_str),
                   __entry.id, __entry.pswmask, __entry.pswaddr $(, $p_args)*)
    };
}

TRACE_EVENT!(kvm_s390_skey_related_inst,
    TP_PROTO!(VCPU_PROTO_COMMON!()),
    TP_ARGS!(VCPU_ARGS_COMMON!()),
    TP_STRUCT__entry!(VCPU_FIELD_COMMON!()),
    TP_fast_assign!(VCPU_ASSIGN_COMMON!()),
    VCPU_TP_PRINTK!("%s", "storage key related instruction")
);

macro_rules! __trace_event_common_token {
    ($name:ident, $proto:tt, $args:tt, $fields:tt, $assign:tt, $print:tt) => {
        TRACE_EVENT!($name, $proto, $args, $fields, $assign, $print);
    };
}

TRACE_EVENT!(kvm_s390_pfault_init,
    TP_PROTO!(VCPU_PROTO_COMMON!(), long pfault_token),
    TP_ARGS!(VCPU_ARGS_COMMON!(), pfault_token),
    TP_STRUCT__entry!(VCPU_FIELD_COMMON!(); __field!(c_long, pfault_token)),
    TP_fast_assign!(VCPU_ASSIGN_COMMON!(); __entry.pfault_token = pfault_token;),
    VCPU_TP_PRINTK!("init pfault token %ld", __entry.pfault_token)
);
TRACE_EVENT!(kvm_s390_pfault_done,
    TP_PROTO!(VCPU_PROTO_COMMON!(), long pfault_token),
    TP_ARGS!(VCPU_ARGS_COMMON!(), pfault_token),
    TP_STRUCT__entry!(VCPU_FIELD_COMMON!(); __field!(c_long, pfault_token)),
    TP_fast_assign!(VCPU_ASSIGN_COMMON!(); __entry.pfault_token = pfault_token;),
    VCPU_TP_PRINTK!("done pfault token %ld", __entry.pfault_token)
);

// Tracepoints for SIE entry and exit.
TRACE_EVENT!(kvm_s390_sie_enter, TP_PROTO!(VCPU_PROTO_COMMON!(), c_int cpuflags), TP_ARGS!(VCPU_ARGS_COMMON!(), cpuflags), TP_STRUCT__entry!(VCPU_FIELD_COMMON!(); __field!(c_int, cpuflags)), TP_fast_assign!(VCPU_ASSIGN_COMMON!(); __entry.cpuflags = cpuflags;), VCPU_TP_PRINTK!("entering sie flags %x", __entry.cpuflags));
TRACE_EVENT!(kvm_s390_sie_fault, TP_PROTO!(VCPU_PROTO_COMMON!()), TP_ARGS!(VCPU_ARGS_COMMON!()), TP_STRUCT__entry!(VCPU_FIELD_COMMON!()), TP_fast_assign!(VCPU_ASSIGN_COMMON!()), VCPU_TP_PRINTK!("%s", "fault in sie instruction"));
TRACE_EVENT!(kvm_s390_sie_exit, TP_PROTO!(VCPU_PROTO_COMMON!(), u8 icptcode), TP_ARGS!(VCPU_ARGS_COMMON!(), icptcode), TP_STRUCT__entry!(VCPU_FIELD_COMMON!(); __field!(u8, icptcode)), TP_fast_assign!(VCPU_ASSIGN_COMMON!(); __entry.icptcode = icptcode;), VCPU_TP_PRINTK!("exit sie icptcode %d (%s)", __entry.icptcode, __print_symbolic!(__entry.icptcode, sie_intercept_code)));

// Trace point for intercepted instructions.
TRACE_EVENT!(kvm_s390_intercept_instruction, TP_PROTO!(VCPU_PROTO_COMMON!(), u16 ipa, u32 ipb), TP_ARGS!(VCPU_ARGS_COMMON!(), ipa, ipb), TP_STRUCT__entry!(VCPU_FIELD_COMMON!(); __field!(u64, instruction)), TP_fast_assign!(VCPU_ASSIGN_COMMON!(); __entry.instruction = ((ipa as u64) << 48) | ((ipb as u64) << 16);), VCPU_TP_PRINTK!("intercepted instruction %016llx (%s)", __entry.instruction, __print_symbolic!(icpt_insn_decoder(__entry.instruction), icpt_insn_codes)));
// Trace point for intercepted program interruptions.
TRACE_EVENT!(kvm_s390_intercept_prog, TP_PROTO!(VCPU_PROTO_COMMON!(), u16 code), TP_ARGS!(VCPU_ARGS_COMMON!(), code), TP_STRUCT__entry!(VCPU_FIELD_COMMON!(); __field!(u16, code)), TP_fast_assign!(VCPU_ASSIGN_COMMON!(); __entry.code = code;), VCPU_TP_PRINTK!("intercepted program interruption %04x (%s)", __entry.code, __print_symbolic!(__entry.code, icpt_prog_codes)));
// Trace point for validity intercepts.
TRACE_EVENT!(kvm_s390_intercept_validity, TP_PROTO!(VCPU_PROTO_COMMON!(), u16 viwhy), TP_ARGS!(VCPU_ARGS_COMMON!(), viwhy), TP_STRUCT__entry!(VCPU_FIELD_COMMON!(); __field!(u16, viwhy)), TP_fast_assign!(VCPU_ASSIGN_COMMON!(); __entry.viwhy = viwhy;), VCPU_TP_PRINTK!("got validity intercept %04x", __entry.viwhy));

// Remaining instruction tracepoints retain the kernel TRACE_EVENT interface.
TRACE_EVENT!(kvm_s390_handle_sigp, TP_PROTO!(VCPU_PROTO_COMMON!(), u8 order_code, u16 cpu_addr, u32 parameter), TP_ARGS!(VCPU_ARGS_COMMON!(), order_code, cpu_addr, parameter), TP_STRUCT__entry!(VCPU_FIELD_COMMON!(); __field!(u8, order_code); __field!(u16, cpu_addr); __field!(u32, parameter)), TP_fast_assign!(VCPU_ASSIGN_COMMON!(); __entry.order_code = order_code; __entry.cpu_addr = cpu_addr; __entry.parameter = parameter;), VCPU_TP_PRINTK!("handle sigp order %02x (%s), cpu address %04x, parameter %08x", __entry.order_code, __print_symbolic!(__entry.order_code, sigp_order_codes), __entry.cpu_addr, __entry.parameter));
TRACE_EVENT!(kvm_s390_handle_sigp_pei, TP_PROTO!(VCPU_PROTO_COMMON!(), u8 order_code, u16 cpu_addr), TP_ARGS!(VCPU_ARGS_COMMON!(), order_code, cpu_addr), TP_STRUCT__entry!(VCPU_FIELD_COMMON!(); __field!(u8, order_code); __field!(u16, cpu_addr)), TP_fast_assign!(VCPU_ASSIGN_COMMON!(); __entry.order_code = order_code; __entry.cpu_addr = cpu_addr;), VCPU_TP_PRINTK!("handle sigp pei order %02x (%s), cpu address %04x", __entry.order_code, __print_symbolic!(__entry.order_code, sigp_order_codes), __entry.cpu_addr));
TRACE_EVENT!(kvm_s390_handle_diag, TP_PROTO!(VCPU_PROTO_COMMON!(), u16 code), TP_ARGS!(VCPU_ARGS_COMMON!(), code), TP_STRUCT__entry!(VCPU_FIELD_COMMON!(); __field!(u16, code)), TP_fast_assign!(VCPU_ASSIGN_COMMON!(); __entry.code = code;), VCPU_TP_PRINTK!("handle diagnose call %04x (%s)", __entry.code, __print_symbolic!(__entry.code, diagnose_codes)));
TRACE_EVENT!(kvm_s390_diag_9c, TP_PROTO!(VCPU_PROTO_COMMON!(), c_int target_vcpu, c_int target_cpu, *const c_char result), TP_ARGS!(VCPU_ARGS_COMMON!(), target_vcpu, target_cpu, result), TP_STRUCT__entry!(VCPU_FIELD_COMMON!(); __field!(c_int, target_vcpu); __field!(c_int, target_cpu); __string!(result, result)), TP_fast_assign!(VCPU_ASSIGN_COMMON!(); __entry.target_vcpu = target_vcpu; __entry.target_cpu = target_cpu; __assign_str!(result);), VCPU_TP_PRINTK!("diag=9c target_vcpu=%d target_pcpu=%d result=%s", __entry.target_vcpu, __entry.target_cpu, __get_str!(result)));
TRACE_EVENT!(kvm_s390_handle_lctl, TP_PROTO!(VCPU_PROTO_COMMON!(), c_int g, c_int reg1, c_int reg3, u64 addr), TP_ARGS!(VCPU_ARGS_COMMON!(), g, reg1, reg3, addr), TP_STRUCT__entry!(VCPU_FIELD_COMMON!(); __field!(c_int, g); __field!(c_int, reg1); __field!(c_int, reg3); __field!(u64, addr)), TP_fast_assign!(VCPU_ASSIGN_COMMON!(); __entry.g = g; __entry.reg1 = reg1; __entry.reg3 = reg3; __entry.addr = addr;), VCPU_TP_PRINTK!("%s: loading cr %x-%x from %016llx", if __entry.g != 0 { "lctlg" } else { "lctl" }, __entry.reg1, __entry.reg3, __entry.addr));
TRACE_EVENT!(kvm_s390_handle_stctl, TP_PROTO!(VCPU_PROTO_COMMON!(), c_int g, c_int reg1, c_int reg3, u64 addr), TP_ARGS!(VCPU_ARGS_COMMON!(), g, reg1, reg3, addr), TP_STRUCT__entry!(VCPU_FIELD_COMMON!(); __field!(c_int, g); __field!(c_int, reg1); __field!(c_int, reg3); __field!(u64, addr)), TP_fast_assign!(VCPU_ASSIGN_COMMON!(); __entry.g = g; __entry.reg1 = reg1; __entry.reg3 = reg3; __entry.addr = addr;), VCPU_TP_PRINTK!("%s: storing cr %x-%x to %016llx", if __entry.g != 0 { "stctg" } else { "stctl" }, __entry.reg1, __entry.reg3, __entry.addr));
TRACE_EVENT!(kvm_s390_handle_prefix, TP_PROTO!(VCPU_PROTO_COMMON!(), c_int set, u32 address), TP_ARGS!(VCPU_ARGS_COMMON!(), set, address), TP_STRUCT__entry!(VCPU_FIELD_COMMON!(); __field!(c_int, set); __field!(u32, address)), TP_fast_assign!(VCPU_ASSIGN_COMMON!(); __entry.set = set; __entry.address = address;), VCPU_TP_PRINTK!("%s prefix to %08x", if __entry.set != 0 { "setting" } else { "storing" }, __entry.address));
TRACE_EVENT!(kvm_s390_handle_stap, TP_PROTO!(VCPU_PROTO_COMMON!(), u64 address), TP_ARGS!(VCPU_ARGS_COMMON!(), address), TP_STRUCT__entry!(VCPU_FIELD_COMMON!(); __field!(u64, address)), TP_fast_assign!(VCPU_ASSIGN_COMMON!(); __entry.address = address;), VCPU_TP_PRINTK!("storing cpu address to %016llx", __entry.address));
TRACE_EVENT!(kvm_s390_handle_stfl, TP_PROTO!(VCPU_PROTO_COMMON!(), c_uint facility_list), TP_ARGS!(VCPU_ARGS_COMMON!(), facility_list), TP_STRUCT__entry!(VCPU_FIELD_COMMON!(); __field!(c_uint, facility_list)), TP_fast_assign!(VCPU_ASSIGN_COMMON!(); __entry.facility_list = facility_list;), VCPU_TP_PRINTK!("store facility list value %08x", __entry.facility_list));
TRACE_EVENT!(kvm_s390_handle_stsi, TP_PROTO!(VCPU_PROTO_COMMON!(), c_int fc, c_int sel1, c_int sel2, u64 addr), TP_ARGS!(VCPU_ARGS_COMMON!(), fc, sel1, sel2, addr), TP_STRUCT__entry!(VCPU_FIELD_COMMON!(); __field!(c_int, fc); __field!(c_int, sel1); __field!(c_int, sel2); __field!(u64, addr)), TP_fast_assign!(VCPU_ASSIGN_COMMON!(); __entry.fc = fc; __entry.sel1 = sel1; __entry.sel2 = sel2; __entry.addr = addr;), VCPU_TP_PRINTK!("STSI %d.%d.%d information stored to %016llx", __entry.fc, __entry.sel1, __entry.sel2, __entry.addr));
TRACE_EVENT!(kvm_s390_handle_operexc, TP_PROTO!(VCPU_PROTO_COMMON!(), u16 ipa, u32 ipb), TP_ARGS!(VCPU_ARGS_COMMON!(), ipa, ipb), TP_STRUCT__entry!(VCPU_FIELD_COMMON!(); __field!(u64, instruction)), TP_fast_assign!(VCPU_ASSIGN_COMMON!(); __entry.instruction = ((ipa as u64) << 48) | ((ipb as u64) << 16);), VCPU_TP_PRINTK!("operation exception on instruction %016llx (%s)", __entry.instruction, __print_symbolic!(icpt_insn_decoder(__entry.instruction), icpt_insn_codes)));
TRACE_EVENT!(kvm_s390_handle_sthyi, TP_PROTO!(VCPU_PROTO_COMMON!(), u64 code, u64 addr), TP_ARGS!(VCPU_ARGS_COMMON!(), code, addr), TP_STRUCT__entry!(VCPU_FIELD_COMMON!(); __field!(u64, code); __field!(u64, addr)), TP_fast_assign!(VCPU_ASSIGN_COMMON!(); __entry.code = code; __entry.addr = addr;), VCPU_TP_PRINTK!("STHYI fc: %llu addr: %016llx", __entry.code, __entry.addr));

// The C header includes trace/define_trace.h outside its include guard.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
