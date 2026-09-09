/* SPDX-License-Identifier: GPL-2.0 */

pub const __VCPU_REGS_RAX: i32 = 0;
pub const __VCPU_REGS_RCX: i32 = 1;
pub const __VCPU_REGS_RDX: i32 = 2;
pub const __VCPU_REGS_RBX: i32 = 3;
pub const __VCPU_REGS_RSP: i32 = 4;
pub const __VCPU_REGS_RBP: i32 = 5;
pub const __VCPU_REGS_RSI: i32 = 6;
pub const __VCPU_REGS_RDI: i32 = 7;

// Assembly-only register-numbering definitions from the C header.
// REG_NUM_INVALID = 100.
//
// R32_NUM(opd, r32) converts the 32-bit register operand r32 into a
// register number and stores it in opd. It is only for !CONFIG_X86_64.
//
// R64_NUM(opd, r64) converts the 64-bit register operand r64 into a
// register number and stores it in opd. Under CONFIG_X86_64, rax..rdi map
// to __VCPU_REGS_RAX..__VCPU_REGS_RDI and r8..r15 map to 8..15; otherwise
// the result is REG_NUM_INVALID.
//
// REG_NUM(reg_num, reg) selects R64_NUM under CONFIG_X86_64 and R32_NUM
// otherwise, then emits an error when the result is REG_NUM_INVALID.
pub const REG_NUM_INVALID: i32 = 100;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
