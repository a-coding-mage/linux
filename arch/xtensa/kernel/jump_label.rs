// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Cadence Design Systems Inc.

use core::ffi::c_void;

// C dependencies supplied by the surrounding kernel translation.
#[repr(C)]
pub struct atomic_t {
    pub counter: i32,
}

#[repr(C)]
pub struct jump_entry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cpumask {
    _private: [u8; 0],
}

#[repr(C)]
pub enum jump_label_type {
    JUMP_LABEL_NOP = 0,
    JUMP_LABEL_JMP = 1,
}

extern "C" {
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn local_flush_icache_range(start: usize, end: usize);
    fn __invalidate_icache_range(addr: usize, size: usize);
    fn atomic_inc_return(v: *mut atomic_t) -> i32;
    fn atomic_inc(v: *mut atomic_t);
    fn atomic_read(v: *const atomic_t) -> i32;
    fn num_online_cpus() -> i32;
    fn cpu_relax();
    fn stop_machine_cpuslocked(
        fn_ptr: unsafe extern "C" fn(*mut c_void) -> i32,
        data: *mut c_void,
        cpus: *const cpumask,
    );
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
    fn jump_entry_target(e: *const jump_entry) -> usize;
    fn jump_entry_code(e: *const jump_entry) -> usize;
    fn bug_on(condition: bool);
}

const J_OFFSET_MASK: u32 = 0x0003ffff;
const J_SIGN_MASK: u32 = !(J_OFFSET_MASK >> 1);

// The C source selects these constants with __XTENSA_EL__ or __XTENSA_EB__.
#[cfg(target_endian = "little")]
const J_INSN: u32 = 0x6;
#[cfg(target_endian = "little")]
const NOP_INSN: u32 = 0x0020f0;
#[cfg(target_endian = "big")]
const J_INSN: u32 = 0x60000000;
#[cfg(target_endian = "big")]
const NOP_INSN: u32 = 0x0f020000;

const JUMP_LABEL_NOP_SIZE: usize = 4;

#[repr(C)]
struct patch {
    cpu_count: atomic_t,
    addr: usize,
    sz: usize,
    data: *const c_void,
}

unsafe fn local_patch_text(addr: usize, data: *const c_void, sz: usize) {
    memcpy(addr as *mut c_void, data, sz);
    local_flush_icache_range(addr, addr + sz);
}

unsafe extern "C" fn patch_text_stop_machine(data: *mut c_void) -> i32 {
    let patch = data as *mut patch;

    if atomic_inc_return(&mut (*patch).cpu_count) == num_online_cpus() {
        local_patch_text((*patch).addr, (*patch).data, (*patch).sz);
        atomic_inc(&mut (*patch).cpu_count);
    } else {
        while atomic_read(&(*patch).cpu_count) <= num_online_cpus() {
            cpu_relax();
        }
        __invalidate_icache_range((*patch).addr, (*patch).sz);
    }
    0
}

unsafe fn patch_text(addr: usize, data: *const c_void, sz: usize) {
    // CONFIG_SMP is a build-time condition in the C source.
    #[cfg(feature = "CONFIG_SMP")]
    {
        let mut patch = patch {
            cpu_count: atomic_t { counter: 0 },
            addr,
            sz,
            data,
        };
        stop_machine_cpuslocked(
            patch_text_stop_machine,
            &mut patch as *mut patch as *mut c_void,
            core::ptr::null(),
        );
    }
    #[cfg(not(feature = "CONFIG_SMP"))]
    {
        let mut flags: usize = 0;

        local_irq_save(&mut flags);
        local_patch_text(addr, data, sz);
        local_irq_restore(flags);
    }
}

pub unsafe fn arch_jump_label_transform(e: *mut jump_entry, type_: jump_label_type) {
    let d = (jump_entry_target(e) - (jump_entry_code(e) + 4)) as u32;
    let insn: u32;

    /* Jump only works within 128K of the J instruction. */
    bug_on(!((d & J_SIGN_MASK) == 0 || (d & J_SIGN_MASK) == J_SIGN_MASK));

    if matches!(type_, jump_label_type::JUMP_LABEL_JMP) {
        #[cfg(target_endian = "little")]
        {
            insn = ((d & J_OFFSET_MASK) << 6) | J_INSN;
        }
        #[cfg(target_endian = "big")]
        {
            insn = ((d & J_OFFSET_MASK) << 8) | J_INSN;
        }
    } else {
        insn = NOP_INSN;
    }

    patch_text(jump_entry_code(e), &insn as *const u32 as *const c_void, JUMP_LABEL_NOP_SIZE);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
