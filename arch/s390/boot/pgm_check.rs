// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_void};

// C header dependencies are supplied by the surrounding kernel translation.

extern "C" {
    static mut _stack_start: usize;
    static mut _stack_end: usize;
    static mut __start___ex_table: exception_table_entry;
    static mut __stop___ex_table: exception_table_entry;
    static mut bootdebug: bool;
    static kernel_version: *const c_char;
    static early_command_line: [c_char; 1];
    static __kaslr_offset: usize;
    static __kaslr_offset_phys: usize;

    fn boot_emerg(fmt: *const c_char, ...);
    fn boot_rb_dump();
    fn is_prot_virt_guest() -> bool;
    fn kaslr_enabled() -> bool;
    fn on_stack(info: *const stack_info, sp: usize, size: usize) -> bool;
    fn psw_bits(psw: psw) -> *mut psw_bits;
    fn extable_fixup(x: *const exception_table_entry) -> usize;
}

#[repr(C)]
struct stack_info {
    r#type: usize,
    low: usize,
    high: usize,
}

#[repr(C)]
struct stack_frame {
    back_chain: usize,
    gprs: [usize; 16],
}

#[repr(C)]
struct exception_table_entry {
    insn: i32,
    fixup: i32,
    r#type: u16,
}

#[repr(C)]
struct psw {
    mask: usize,
    addr: usize,
}

#[repr(C)]
struct psw_bits {
    per: u8,
    dat: u8,
    io: u8,
    ext: u8,
    key: u8,
    mcheck: u8,
    wait: u8,
    pstate: u8,
    as_: u8,
    cc: u8,
    pm: u8,
    ri: u8,
    eaba: u8,
}

#[repr(C)]
struct pt_regs {
    gprs: [usize; 16],
    psw: psw,
    int_code: u32,
    last_break: usize,
}

const STACK_TYPE_TASK: usize = 0;
const EX_TYPE_FIXUP: u16 = 1;

pub unsafe fn print_stacktrace(mut sp: usize) {
    let boot_stack = stack_info {
        r#type: STACK_TYPE_TASK,
        low: _stack_start,
        high: _stack_end,
    };
    let mut first = true;

    boot_emerg(b"Call Trace:\n\0".as_ptr() as *const c_char);
    while (sp & 0x7) == 0 && on_stack(
        &boot_stack,
        sp,
        core::mem::size_of::<stack_frame>(),
    ) {
        let sf = sp as *const stack_frame;

        if first {
            boot_emerg(
                b"(sp:%016lx [<%016lx>] %pS)\n\0".as_ptr() as *const c_char,
                sp,
                (*sf).gprs[8],
                (*sf).gprs[8] as *const c_void,
            );
        } else {
            boot_emerg(
                b" sp:%016lx [<%016lx>] %pS\n\0".as_ptr() as *const c_char,
                sp,
                (*sf).gprs[8],
                (*sf).gprs[8] as *const c_void,
            );
        }
        if (*sf).back_chain <= sp {
            break;
        }
        sp = (*sf).back_chain;
        first = false;
    }
}

#[inline]
unsafe fn extable_insn(x: *const exception_table_entry) -> usize {
    (x as *const u8).add(core::mem::offset_of!(exception_table_entry, insn)) as usize
        .wrapping_add((*x).insn as isize as usize)
}

unsafe fn ex_handler(regs: *mut pt_regs) -> bool {
    let mut ex = &raw const __start___ex_table;
    let end = &raw const __stop___ex_table;

    while ex < end {
        if extable_insn(ex) != (*regs).psw.addr {
            ex = ex.add(1);
            continue;
        }
        if (*ex).r#type != EX_TYPE_FIXUP {
            return false;
        }
        (*regs).psw.addr = extable_fixup(ex);
        return true;
    }
    false
}

pub unsafe fn do_pgm_check(regs: *mut pt_regs) {
    let psw = psw_bits((*regs).psw);
    let gpregs = (*regs).gprs.as_ptr();

    if ex_handler(regs) {
        return;
    }
    if bootdebug {
        boot_rb_dump();
    }
    boot_emerg(b"Linux version %s\n\0".as_ptr() as *const c_char, kernel_version);
    if !is_prot_virt_guest() && early_command_line[0] != 0 {
        boot_emerg(b"Kernel command line: %s\n\0".as_ptr() as *const c_char, early_command_line.as_ptr());
    }
    boot_emerg(
        b"Kernel fault: interruption code %04x ilc:%d\n\0".as_ptr() as *const c_char,
        (*regs).int_code & 0xffff,
        (*regs).int_code >> 17,
    );
    if kaslr_enabled() {
        boot_emerg(b"Kernel random base: %lx\n\0".as_ptr() as *const c_char, __kaslr_offset);
        boot_emerg(b"Kernel random base phys: %lx\n\0".as_ptr() as *const c_char, __kaslr_offset_phys);
    }
    boot_emerg(b"PSW : %016lx %016lx (%pS)\n\0".as_ptr() as *const c_char, (*regs).psw.mask, (*regs).psw.addr, (*regs).psw.addr as *const c_void);
    boot_emerg(b"      R:%x T:%x IO:%x EX:%x Key:%x M:%x W:%x P:%x AS:%x CC:%x PM:%x RI:%x EA:%x\n\0".as_ptr() as *const c_char, (*psw).per, (*psw).dat, (*psw).io, (*psw).ext, (*psw).key, (*psw).mcheck, (*psw).wait, (*psw).pstate, (*psw).as_, (*psw).cc, (*psw).pm, (*psw).ri, (*psw).eaba);
    boot_emerg(b"GPRS: %016lx %016lx %016lx %016lx\n\0".as_ptr() as *const c_char, *gpregs.add(0), *gpregs.add(1), *gpregs.add(2), *gpregs.add(3));
    boot_emerg(b"      %016lx %016lx %016lx %016lx\n\0".as_ptr() as *const c_char, *gpregs.add(4), *gpregs.add(5), *gpregs.add(6), *gpregs.add(7));
    boot_emerg(b"      %016lx %016lx %016lx %016lx\n\0".as_ptr() as *const c_char, *gpregs.add(8), *gpregs.add(9), *gpregs.add(10), *gpregs.add(11));
    boot_emerg(b"      %016lx %016lx %016lx %016lx\n\0".as_ptr() as *const c_char, *gpregs.add(12), *gpregs.add(13), *gpregs.add(14), *gpregs.add(15));
    print_stacktrace(*gpregs.add(15));
    boot_emerg(b"Last Breaking-Event-Address:\n\0".as_ptr() as *const c_char);
    boot_emerg(b" [<%016lx>] %pS\n\0".as_ptr() as *const c_char, (*regs).last_break, (*regs).last_break as *const c_void);
    (*psw).io = 0;
    (*psw).ext = 0;
    (*psw).wait = 1;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
