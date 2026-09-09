// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2000, 2001, 2002, 2003 Broadcom Corporation
 */

// Linux and SiByte headers provide the types, constants, macros, and
// functions referenced below.

#[cfg(CONFIG_SIBYTE_HAS_LDT)]
extern "C" {
    static mut ldt_eoi_space: usize;
}

extern "C" {
    static mut sb1250_irq_owner: [core::ffi::c_int; SB1250_NR_IRQS as usize];
}

extern "C" {
    fn raw_spin_lock_irqsave(lock: *mut RawSpinLock, flags: *mut usize);
    fn raw_spin_unlock_irqrestore(lock: *mut RawSpinLock, flags: usize);
    fn ____raw_readq(addr: usize) -> u64;
    fn ____raw_writeq(value: u64, addr: usize);
    fn __raw_readq(addr: usize) -> u64;
    fn __raw_writeq(value: u64, addr: usize);
    fn cpu_logical_map(cpu: core::ffi::c_int) -> core::ffi::c_int;
    fn cpumask_first_and(mask: *const CpuMask, online: *const CpuMask) -> core::ffi::c_int;
    fn smp_processor_id() -> core::ffi::c_uint;
    fn irq_set_chip_and_handler(irq: core::ffi::c_uint, chip: *mut IrqChip, handler: unsafe extern "C" fn());
    fn do_IRQ(irq: core::ffi::c_int);
    fn fls64(value: u64) -> core::ffi::c_int;
    fn read_c0_cause() -> core::ffi::c_uint;
    fn read_c0_status() -> core::ffi::c_uint;
    fn change_c0_status(mask: core::ffi::c_uint, value: core::ffi::c_uint);
    fn sb1250_mailbox_interrupt();
    fn spurious_interrupt();
    fn handle_level_irq();
}

#[repr(C)]
pub struct RawSpinLock;
#[repr(C)]
pub struct CpuMask;
#[repr(C)]
pub struct IrqData { pub irq: core::ffi::c_uint }
#[repr(C)]
pub struct IrqChip {
    pub name: *const core::ffi::c_char,
    pub irq_mask_ack: Option<unsafe extern "C" fn(*mut IrqData)>,
    pub irq_unmask: Option<unsafe extern "C" fn(*mut IrqData)>,
    pub irq_mask: Option<unsafe extern "C" fn(*mut IrqData)>,
    pub irq_set_affinity: Option<unsafe extern "C" fn(*mut IrqData, *const CpuMask, bool) -> core::ffi::c_int>,
}

static mut sb1250_imr_lock: RawSpinLock = RawSpinLock;

pub unsafe extern "C" fn sb1250_mask_irq(cpu: core::ffi::c_int, irq: core::ffi::c_int) {
    let mut flags = 0usize;
    raw_spin_lock_irqsave(&mut sb1250_imr_lock, &mut flags);
    let mut cur_ints = ____raw_readq(IOADDR(A_IMR_MAPPER(cpu) + R_IMR_INTERRUPT_MASK));
    cur_ints |= 1u64 << irq;
    ____raw_writeq(cur_ints, IOADDR(A_IMR_MAPPER(cpu) + R_IMR_INTERRUPT_MASK));
    raw_spin_unlock_irqrestore(&mut sb1250_imr_lock, flags);
}

pub unsafe extern "C" fn sb1250_unmask_irq(cpu: core::ffi::c_int, irq: core::ffi::c_int) {
    let mut flags = 0usize;
    raw_spin_lock_irqsave(&mut sb1250_imr_lock, &mut flags);
    let mut cur_ints = ____raw_readq(IOADDR(A_IMR_MAPPER(cpu) + R_IMR_INTERRUPT_MASK));
    cur_ints &= !(1u64 << irq);
    ____raw_writeq(cur_ints, IOADDR(A_IMR_MAPPER(cpu) + R_IMR_INTERRUPT_MASK));
    raw_spin_unlock_irqrestore(&mut sb1250_imr_lock, flags);
}

#[cfg(CONFIG_SMP)]
unsafe extern "C" fn sb1250_set_affinity(d: *mut IrqData, mask: *const CpuMask, _force: bool) -> core::ffi::c_int {
    let i = cpumask_first_and(mask, cpu_online_mask);
    let cpu = cpu_logical_map(i);
    let mut flags = 0usize;
    raw_spin_lock_irqsave(&mut sb1250_imr_lock, &mut flags);
    let irq = (*d).irq as usize;
    let old_cpu = sb1250_irq_owner[irq];
    let mut cur_ints = ____raw_readq(IOADDR(A_IMR_MAPPER(old_cpu) + R_IMR_INTERRUPT_MASK));
    let int_on = (cur_ints & (1u64 << irq)) == 0;
    if int_on {
        cur_ints |= 1u64 << irq;
        ____raw_writeq(cur_ints, IOADDR(A_IMR_MAPPER(old_cpu) + R_IMR_INTERRUPT_MASK));
    }
    sb1250_irq_owner[irq] = cpu;
    if int_on {
        cur_ints = ____raw_readq(IOADDR(A_IMR_MAPPER(cpu) + R_IMR_INTERRUPT_MASK));
        cur_ints &= !(1u64 << irq);
        ____raw_writeq(cur_ints, IOADDR(A_IMR_MAPPER(cpu) + R_IMR_INTERRUPT_MASK));
    }
    raw_spin_unlock_irqrestore(&mut sb1250_imr_lock, flags);
    0
}

unsafe extern "C" fn disable_sb1250_irq(d: *mut IrqData) { sb1250_mask_irq(sb1250_irq_owner[(*d).irq as usize], (*d).irq as i32); }
unsafe extern "C" fn enable_sb1250_irq(d: *mut IrqData) { sb1250_unmask_irq(sb1250_irq_owner[(*d).irq as usize], (*d).irq as i32); }

unsafe extern "C" fn ack_sb1250_irq(d: *mut IrqData) {
    let irq = (*d).irq;
    #[cfg(CONFIG_SIBYTE_HAS_LDT)] {
        let pending = __raw_readq(IOADDR(A_IMR_REGISTER(sb1250_irq_owner[irq as usize], R_IMR_LDT_INTERRUPT))) & (1u64 << irq);
        if pending != 0 {
            for i in 0..NR_CPUS {
                let cpu = if cfg!(CONFIG_SMP) { cpu_logical_map(i) } else { i };
                __raw_writeq(pending, IOADDR(A_IMR_REGISTER(cpu, R_IMR_LDT_INTERRUPT_CLR)));
            }
            *((ldt_eoi_space + ((irq as usize) << 16) + (7 << 2)) as *mut u32) = 0;
        }
    }
    sb1250_mask_irq(sb1250_irq_owner[irq as usize], irq as i32);
}

static mut sb1250_irq_type: IrqChip = IrqChip {
    name: b"SB1250-IMR\0".as_ptr() as *const _, irq_mask_ack: Some(ack_sb1250_irq),
    irq_unmask: Some(enable_sb1250_irq), irq_mask: Some(disable_sb1250_irq),
    irq_set_affinity: None,
};

pub unsafe extern "C" fn init_sb1250_irqs() {
    for i in 0..SB1250_NR_IRQS { irq_set_chip_and_handler(i as u32, &mut sb1250_irq_type, handle_level_irq); sb1250_irq_owner[i as usize] = 0; }
}

const IMR_IP2_VAL: u64 = K_INT_MAP_I0;
const IMR_IP3_VAL: u64 = K_INT_MAP_I1;

pub unsafe extern "C" fn arch_init_irq() {
    let imask = STATUSF_IP4 | STATUSF_IP3 | STATUSF_IP2 | STATUSF_IP1 | STATUSF_IP0;
    for i in 0..SB1250_NR_IRQS { __raw_writeq(IMR_IP2_VAL, IOADDR(A_IMR_REGISTER(0, R_IMR_INTERRUPT_MAP_BASE) + (i << 3))); __raw_writeq(IMR_IP2_VAL, IOADDR(A_IMR_REGISTER(1, R_IMR_INTERRUPT_MAP_BASE) + (i << 3))); }
    init_sb1250_irqs();
    __raw_writeq(IMR_IP3_VAL, IOADDR(A_IMR_REGISTER(0, R_IMR_INTERRUPT_MAP_BASE) + (K_INT_MBOX_0 << 3)));
    __raw_writeq(IMR_IP3_VAL, IOADDR(A_IMR_REGISTER(1, R_IMR_INTERRUPT_MAP_BASE) + (K_INT_MBOX_0 << 3)));
    __raw_writeq(0xffffffffffffffff, IOADDR(A_IMR_REGISTER(0, R_IMR_MAILBOX_CLR_CPU)));
    __raw_writeq(0xffffffffffffffff, IOADDR(A_IMR_REGISTER(1, R_IMR_MAILBOX_CLR_CPU)));
    let tmp = !0u64 ^ (1u64 << K_INT_MBOX_0);
    __raw_writeq(tmp, IOADDR(A_IMR_REGISTER(0, R_IMR_INTERRUPT_MASK))); __raw_writeq(tmp, IOADDR(A_IMR_REGISTER(1, R_IMR_INTERRUPT_MASK)));
    change_c0_status(ST0_IM, imask);
}

unsafe extern "C" fn dispatch_ip2() { let cpu = smp_processor_id(); let mask = __raw_readq(IOADDR(A_IMR_REGISTER(cpu, R_IMR_INTERRUPT_STATUS_BASE))); if mask != 0 { do_IRQ(fls64(mask) - 1); } }

pub unsafe extern "C" fn plat_irq_dispatch() {
    let cpu = smp_processor_id();
    let pending = read_c0_cause() & read_c0_status() & ST0_IM;
    if pending & CAUSEF_IP7 != 0 { do_IRQ(MIPS_CPU_IRQ_BASE + 7); }
    else if pending & CAUSEF_IP4 != 0 { do_IRQ(K_INT_TIMER_0 + cpu); }
    #[cfg(CONFIG_SMP)]
    else if pending & CAUSEF_IP3 != 0 { sb1250_mailbox_interrupt(); }
    else if pending & CAUSEF_IP2 != 0 { dispatch_ip2(); }
    else { spurious_interrupt(); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
