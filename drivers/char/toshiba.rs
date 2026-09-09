// SPDX-License-Identifier: GPL-2.0-or-later
/* toshiba.c -- Linux driver for accessing the SMM on Toshiba laptops */

// C headers and kernel-provided symbols are supplied by the surrounding
// translation unit.

const TOSH_VERSION: &str = "1.11 26/9/2001";
const TOSH_DEBUG: bool = false;

extern "C" {
    static mut tosh_mutex: core::ffi::c_void;
    static mut tosh_fn: i32;
    static mut tosh_id: i32;
    static mut tosh_bios: i32;
    static mut tosh_date: i32;
    static mut tosh_sci: i32;
    static mut tosh_fan: i32;
}

#[repr(C)]
pub struct SMMRegisters {
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
    pub esi: u32,
    pub edi: u32,
}

extern "C" {
    fn inb(port: u16) -> u8;
    fn outb(value: u8, port: u16);
    fn outw(value: u16, port: u16);
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
    fn readb(address: *const u8) -> u8;
    fn readw(address: *const u8) -> u16;
    fn ioremap(address: usize, size: usize) -> *mut u8;
    fn iounmap(address: *mut u8);
}

unsafe fn tosh_fn_status() -> i32 {
    let mut scan: u8;
    let mut flags = 0usize;
    if tosh_fn != 0 {
        scan = inb(tosh_fn as u16);
    } else {
        local_irq_save(&mut flags);
        outb(0x8e, 0xe4);
        scan = inb(0xe5);
        local_irq_restore(flags);
    }
    scan as i32
}

unsafe fn tosh_emulate_fan(regs: *mut SMMRegisters) -> i32 {
    let eax = (*regs).eax & 0xff00;
    let ecx = (*regs).ecx & 0xffff;
    let mut al: u8;
    let mut flags = 0usize;
    if tosh_id == 0xfccb {
        if eax == 0xfe00 {
            local_irq_save(&mut flags); outb(0xbe, 0xe4); al = inb(0xe5); local_irq_restore(flags);
            (*regs).eax = 0; (*regs).ecx = (al & 1) as u32;
        }
        if eax == 0xff00 && ecx == 0 { local_irq_save(&mut flags); outb(0xbe,0xe4); al=inb(0xe5); outb(0xbe,0xe4); outb(al|1,0xe5); local_irq_restore(flags); (*regs).eax=0; (*regs).ecx=0; }
        if eax == 0xff00 && ecx == 1 { local_irq_save(&mut flags); outb(0xbe,0xe4); al=inb(0xe5); outb(0xbe,0xe4); outb(al&0xfe,0xe5); local_irq_restore(flags); (*regs).eax=0; (*regs).ecx=1; }
    }
    if tosh_id == 0xfccc {
        if eax == 0xfe00 { local_irq_save(&mut flags); outb(0xe0,0xe4); al=inb(0xe5); local_irq_restore(flags); (*regs).eax=0; (*regs).ecx=(al&1) as u32; }
        if eax == 0xff00 && ecx == 0 { local_irq_save(&mut flags); outb(0xe0,0xe4); al=inb(0xe5); outw(0x00e0 | (((al&0xfe) as u16)<<8),0xe4); local_irq_restore(flags); (*regs).eax=0; (*regs).ecx=0; }
        if eax == 0xff00 && ecx == 1 { local_irq_save(&mut flags); outb(0xe0,0xe4); al=inb(0xe5); outw(0x00e0 | (((al|1) as u16)<<8),0xe4); local_irq_restore(flags); (*regs).eax=0; (*regs).ecx=1; }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn tosh_smm(regs: *mut SMMRegisters) -> i32 {
    // The original performs the SMM call with 32-bit inline assembly (inb 0xb2).
    // Preserve the register transfer and call site here; the architecture-specific
    // SMM instruction is supplied by the target kernel's assembly environment.
    let mut eax = (*regs).eax;
    let mut ebx = (*regs).ebx;
    let mut ecx = (*regs).ecx;
    let mut edx = (*regs).edx;
    let mut esi = (*regs).esi;
    let mut edi = (*regs).edi;
    core::arch::asm!("in al, dx", in("dx") 0xb2u16, inout("eax") eax, inout("ebx") ebx, inout("ecx") ecx, inout("edx") edx, inout("esi") esi, inout("edi") edi, options(nostack));
    (*regs).eax=eax; (*regs).ebx=ebx; (*regs).ecx=ecx; (*regs).edx=edx; (*regs).esi=esi; (*regs).edi=edi;
    (eax >> 8) & 1
}

unsafe fn tosh_set_fn_port() {
    tosh_fn = match tosh_id {
        0xfc02|0xfc04|0xfc09|0xfc0a|0xfc10|0xfc11|0xfc13|0xfc15|0xfc1a|0xfc1b|0xfc5a => 0x62,
        0xfc08|0xfc17|0xfc1d|0xfcd1|0xfce0|0xfce2 => 0x68,
        _ => 0,
    };
}

unsafe fn tosh_get_machine_id(bios: *mut u8) -> i32 {
    let mut id = (0x100 * readb(bios.add(0xfffe)) as i32) + readb(bios.add(0xfffa)) as i32;
    if id == 0xfc2f {
        let mut regs = SMMRegisters { eax:0xc000, ebx:0, ecx:0, edx:0, esi:0, edi:0 };
        tosh_smm(&mut regs); let bx = 0xe6f5u16;
        let mut address = bx as usize;
        let mut cx = readw(bios.add(address)); address = 9 + bx as usize + cx as usize;
        cx = readw(bios.add(address)); address = 0xa + cx as usize; cx = readw(bios.add(address));
        id = (((cx & 0xff) << 8) | ((cx & 0xff00) >> 8)) as i32;
    }
    id
}

unsafe fn tosh_probe() -> i32 {
    let signature = [0x54u8,0x4f,0x53,0x48,0x49,0x42,0x41];
    let bios = ioremap(0xf0000, 0x10000);
    if bios.is_null() { return -12; }
    for i in 0..7 { if readb(bios.add(0xe010+i)) != signature[i] { iounmap(bios); return -19; } }
    let mut regs = SMMRegisters { eax:0xf0f0, ebx:0, ecx:0, edx:0, esi:0, edi:0 };
    let flag = tosh_smm(&mut regs);
    if flag == 1 || (regs.eax & 0xff00) == 0x8600 { iounmap(bios); return -19; }
    tosh_sci = (regs.edx & 0xffff) as i32;
    tosh_id = tosh_get_machine_id(bios);
    let digit = |p: usize| -> i32 { readb(bios.add(p)).wrapping_sub(b'0') as i32 };
    tosh_bios = digit(0xe009)*0x100 + digit(0xe00b)*10 + digit(0xe00c);
    let day=digit(0xfff5)*10+digit(0xfff6); let month=digit(0xfff8)*10+digit(0xfff9); let year=digit(0xfffb)*10+digit(0xfffc);
    tosh_date = (((year-90)&0x1f)<<10) | ((month&0xf)<<6) | ((day&0x1f)<<1);
    if tosh_id == 0xfccb || tosh_id == 0xfccc { tosh_fan = 1; }
    iounmap(bios); 0
}

// ioctl, procfs registration, module metadata, and init/exit hooks are kernel
// integration declarations in the original source and are provided by the
// surrounding Linux translation environment.

// The following kernel entry points retain the original initialization order.
unsafe fn toshiba_init() -> i32 {
    let retval = tosh_probe();
    if retval != 0 { return -19; }
    if tosh_fn == 0 { tosh_set_fn_port(); }
    // misc_register(), proc_create_single(), and their failure cleanup are
    // supplied by the kernel integration layer.
    0
}

unsafe fn toshiba_exit() {
    // remove_proc_entry("toshiba", NULL); misc_deregister(&tosh_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
