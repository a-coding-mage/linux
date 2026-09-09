/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2008 Maxime Bizon <mbizon@freebox.fr>
 * Copyright (C) 2014 Kevin Cernekee <cernekee@gmail.com>
 */

// Linux kernel headers supplied by the surrounding translation unit.

const RELO_NORMAL_VEC: usize = 1 << 18;
const REG_BCM6328_OTP: *mut core::ffi::c_void = 0x1000062c as *mut core::ffi::c_void;
const BCM6328_TP1_DISABLED: u32 = 1 << 9;

extern "C" {
    static mut bmips_cbr_addr: *mut core::ffi::c_void;
    static mut bmips_rac_flush_disable: bool;
    static mut ebase: usize;
    static mut board_ebase_setup: Option<unsafe extern "C" fn()>;
    static mut bmips_smp_enabled: i32;
    static mut cfe_seal: u32;
    static mut fw_arg0: u32;
    static mut fw_arg1: u32;
    static mut fw_arg2: u32;
    static mut fw_arg3: u32;
    static mut bmips_booted_mask: core::ffi::c_void;
    static mut mips_hpt_frequency: u32;
    static mut ioport_resource: IoportResource;

    fn BMIPS_GET_CBR() -> *mut u8;
    fn BMIPS_RELO_VECTOR_CONTROL_1() -> usize;
    fn __raw_writel(value: usize, addr: *mut u8);
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize);
    fn __sync();
    fn set_c0_cause(value: u32);
    fn cpumask_set_cpu(cpu: u32, mask: *mut core::ffi::c_void);
    fn __raw_readl(addr: *const core::ffi::c_void) -> u32;
    fn read_c0_brcm_cmt_local() -> u32;
    fn cfe_init(arg0: u32, arg2: u32);
    fn bmips_cpu_setup();
    fn register_bmips_smp_ops();
    fn of_find_node_by_name(parent: *mut core::ffi::c_void, name: *const u8) -> *mut DeviceNode;
    fn panic(msg: *const u8) -> !;
    fn of_property_read_u32(np: *mut DeviceNode, name: *const u8, value: *mut u32) -> i32;
    fn of_node_put(np: *mut DeviceNode);
    fn set_io_port_base(base: usize);
    fn phys_to_virt(addr: u32) -> *mut core::ffi::c_void;
    fn get_fdt() -> *mut core::ffi::c_void;
    fn cfe_die(msg: *const u8) -> !;
    fn __dt_setup_arch(dtb: *mut core::ffi::c_void);
    fn of_flat_dt_is_compatible(root: *mut core::ffi::c_void, compatible: *const u8) -> bool;
    fn of_get_flat_dt_root() -> *mut core::ffi::c_void;
    fn unflatten_and_copy_device_tree();
    fn of_get_available_child_count(np: *mut DeviceNode) -> usize;
    fn memblock_start_of_DRAM() -> usize;
    fn memblock_end_of_DRAM() -> usize;
    fn of_clk_init(data: *const core::ffi::c_void);
}

#[repr(C)]
struct IoportResource { start: usize, end: usize }

#[repr(C)]
struct DeviceNode;

const C_SW0: u32 = 1 << 8;
const CFE_EPTSEAL: u32 = 0x43464531;
const VMLINUX_LOAD_ADDRESS: usize = 0;

#[repr(C)]
struct BmipsQuirk {
    compatible: *const u8,
    quirk_fn: Option<unsafe extern "C" fn()>,
}

unsafe extern "C" fn kbase_setup() {
    __raw_writel((VMLINUX_LOAD_ADDRESS & 0xfff00000) | RELO_NORMAL_VEC,
                 BMIPS_GET_CBR().add(BMIPS_RELO_VECTOR_CONTROL_1()));
    ebase = VMLINUX_LOAD_ADDRESS & 0xfff00000;
}

unsafe extern "C" fn bcm3384_viper_quirks() {
    board_ebase_setup = Some(kbase_setup);
    bmips_smp_enabled = 0;
}

unsafe extern "C" fn bcm63xx_fixup_cpu1() {
    memcpy(0xa0000200 as *mut _, &bmips_smp_movevec as *const _ as *const _, 0x20);
    __sync();
    set_c0_cause(C_SW0);
    cpumask_set_cpu(1, &mut bmips_booted_mask as *mut _ as *mut _);
}

unsafe extern "C" fn bcm6328_quirks() {
    if __raw_readl(REG_BCM6328_OTP) & BCM6328_TP1_DISABLED != 0 {
        bmips_smp_enabled = 0;
    } else {
        bcm63xx_fixup_cpu1();
    }
}

unsafe extern "C" fn bcm6358_quirks() {
    bmips_smp_enabled = 0;
    bmips_rac_flush_disable = (read_c0_brcm_cmt_local() & (1 << 31) != 0) || bmips_cbr_addr != core::ptr::null_mut();
}

unsafe extern "C" fn bcm6368_quirks() { bcm63xx_fixup_cpu1(); }

static BMIPS_QUIRK_LIST: &[BmipsQuirk] = &[
    BmipsQuirk { compatible: b"brcm,bcm3368\0".as_ptr(), quirk_fn: Some(bcm6358_quirks) },
    BmipsQuirk { compatible: b"brcm,bcm3384-viper\0".as_ptr(), quirk_fn: Some(bcm3384_viper_quirks) },
    BmipsQuirk { compatible: b"brcm,bcm33843-viper\0".as_ptr(), quirk_fn: Some(bcm3384_viper_quirks) },
    BmipsQuirk { compatible: b"brcm,bcm6328\0".as_ptr(), quirk_fn: Some(bcm6328_quirks) },
    BmipsQuirk { compatible: b"brcm,bcm6358\0".as_ptr(), quirk_fn: Some(bcm6358_quirks) },
    BmipsQuirk { compatible: b"brcm,bcm6362\0".as_ptr(), quirk_fn: Some(bcm6368_quirks) },
    BmipsQuirk { compatible: b"brcm,bcm6368\0".as_ptr(), quirk_fn: Some(bcm6368_quirks) },
    BmipsQuirk { compatible: b"brcm,bcm63168\0".as_ptr(), quirk_fn: Some(bcm6368_quirks) },
    BmipsQuirk { compatible: b"brcm,bcm63268\0".as_ptr(), quirk_fn: Some(bcm6368_quirks) },
    BmipsQuirk { compatible: core::ptr::null(), quirk_fn: None },
];

unsafe extern "C" fn bmips_init_cfe() {
    cfe_seal = fw_arg3;
    if cfe_seal != CFE_EPTSEAL { return; }
    cfe_init(fw_arg0, fw_arg2);
}

pub unsafe extern "C" fn prom_init() {
    bmips_cbr_addr = BMIPS_GET_CBR() as *mut _;
    bmips_init_cfe();
    bmips_cpu_setup();
    register_bmips_smp_ops();
}

pub unsafe extern "C" fn get_system_type() -> *const u8 { b"Generic BMIPS kernel\0".as_ptr() }

pub unsafe extern "C" fn plat_time_init() {
    let np = of_find_node_by_name(core::ptr::null_mut(), b"cpus\0".as_ptr());
    if np.is_null() { panic(b"missing 'cpus' DT node\0".as_ptr()); }
    let mut freq = 0;
    if of_property_read_u32(np, b"mips-hpt-frequency\0".as_ptr(), &mut freq) < 0 { panic(b"missing 'mips-hpt-frequency' property\0".as_ptr()); }
    of_node_put(np);
    mips_hpt_frequency = freq;
}

pub unsafe extern "C" fn plat_mem_setup() {
    set_io_port_base(0);
    ioport_resource.start = 0;
    ioport_resource.end = usize::MAX;
    let dtb = if fw_arg0 == 0 && fw_arg1 == 0xffff_ffff { phys_to_virt(fw_arg2) } else { get_fdt() };
    if dtb.is_null() { cfe_die(b"no dtb found\0".as_ptr()); }
    __dt_setup_arch(dtb);
    for q in BMIPS_QUIRK_LIST {
        if q.quirk_fn.is_none() { break; }
        if of_flat_dt_is_compatible(of_get_flat_dt_root(), q.compatible) { (q.quirk_fn.unwrap())(); }
    }
}

pub unsafe extern "C" fn device_tree_init() {
    unflatten_and_copy_device_tree();
    let np = of_find_node_by_name(core::ptr::null_mut(), b"cpus\0".as_ptr());
    if np.is_null() { return; }
    if of_get_available_child_count(np) <= 1 { bmips_smp_enabled = 0; }
    let mut addr = 0;
    if of_property_read_u32(np, b"brcm,bmips-cbr-reg\0".as_ptr(), &mut addr) != 0 { of_node_put(np); return; }
    if (addr as usize) >= memblock_start_of_DRAM() && (addr as usize) < memblock_end_of_DRAM() { of_node_put(np); return; }
    bmips_cbr_addr = addr as usize as *mut _;
    bmips_rac_flush_disable = false;
    of_node_put(np);
}

unsafe extern "C" fn plat_dev_init() -> i32 { of_clk_init(core::ptr::null()); 0 }

// arch_initcall(plat_dev_init);

extern "C" { static bmips_smp_movevec: u8; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
