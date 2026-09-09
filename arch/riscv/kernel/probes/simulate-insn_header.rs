/* SPDX-License-Identifier: GPL-2.0+ */

// C header guard: _RISCV_KERNEL_PROBES_SIMULATE_INSN_H
// Dependency supplied by <asm/insn.h>.

/*
 * The C preprocessor token-pastes `name` into riscv_insn_is_##name and
 * simulate_##name. Rust macro callers provide those two resolved names.
 */
macro_rules! RISCV_INSN_REJECTED {
    ($is_name:path, $code:expr) => {{
        if $is_name($code) {
            return INSN_REJECTED;
        }
    }};
}

macro_rules! RISCV_INSN_SET_SIMULATE {
    ($is_name:path, $handler:path, $code:expr, $api:expr) => {{
        if $is_name($code) {
            $api.handler = $handler;
            return INSN_GOOD_NO_SLOT;
        }
    }};
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

extern "C" {
    pub fn simulate_auipc(opcode: u32, addr: ::core::ffi::c_ulong,
                          regs: *mut pt_regs) -> bool;
    pub fn simulate_branch(opcode: u32, addr: ::core::ffi::c_ulong,
                           regs: *mut pt_regs) -> bool;
    pub fn simulate_jal(opcode: u32, addr: ::core::ffi::c_ulong,
                        regs: *mut pt_regs) -> bool;
    pub fn simulate_jalr(opcode: u32, addr: ::core::ffi::c_ulong,
                         regs: *mut pt_regs) -> bool;
    pub fn simulate_c_j(opcode: u32, addr: ::core::ffi::c_ulong,
                        regs: *mut pt_regs) -> bool;
    pub fn simulate_c_jal(opcode: u32, addr: ::core::ffi::c_ulong,
                          regs: *mut pt_regs) -> bool;
    pub fn simulate_c_jr(opcode: u32, addr: ::core::ffi::c_ulong,
                         regs: *mut pt_regs) -> bool;
    pub fn simulate_c_jalr(opcode: u32, addr: ::core::ffi::c_ulong,
                           regs: *mut pt_regs) -> bool;
    pub fn simulate_c_bnez(opcode: u32, addr: ::core::ffi::c_ulong,
                           regs: *mut pt_regs) -> bool;
    pub fn simulate_c_beqz(opcode: u32, addr: ::core::ffi::c_ulong,
                           regs: *mut pt_regs) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
