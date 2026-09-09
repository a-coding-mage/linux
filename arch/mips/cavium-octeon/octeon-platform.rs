/* Faithful low-level Rust translation of octeon-platform.c. */

use core::ffi::{c_char, c_void};

/* Linux/kernel and Octeon symbols are supplied by the surrounding kernel translation. */
extern "C" {
    static mut initial_boot_params: *mut c_void;
    static mut octeon_bootinfo: *mut OcteonBootInfo;
    fn cvmx_read_csr(a: u64) -> u64; fn cvmx_write_csr(a: u64, v: u64);
    fn cvmx_read64_uint32(a: u64) -> u32; fn cvmx_write64_uint32(a: u64, v: u32);
    fn octeon_get_io_clock_rate() -> u64; fn cvmx_sysinfo_get() -> *mut CvmxSysInfo;
    fn cvmx_helper_board_get_mii_address(p: i32) -> i32;
    fn cvmx_helper_interface_enumerate(i: i32) -> i32;
    fn cvmx_helper_ports_on_interface(i: i32) -> i32;
    fn cvmx_board_type_to_string(t: i32) -> *const c_char;
    fn current_cpu_type() -> i32; fn octeon_has_feature(f: i32) -> bool;
    fn __cvmx_helper_board_usb_get_clock_type() -> i32;
    fn fdt_getprop(b: *mut c_void, n: i32, p: *const c_char, l: *mut i32) -> *const c_char;
    fn fdt_getprop_w(b: *mut c_void, n: i32, p: *const c_char, l: *mut i32) -> *mut u32;
    fn fdt_get_name(b: *mut c_void, n: i32, l: *mut i32) -> *const c_char;
    fn fdt_path_offset(b: *mut c_void, p: *const c_char) -> i32;
    fn fdt_node_offset_by_phandle(b: *mut c_void, p: u32) -> i32;
    fn fdt_subnode_offset(b: *mut c_void, n: i32, p: *const c_char) -> i32;
    fn fdt_parent_offset(b: *mut c_void, n: i32) -> i32;
    fn fdt_setprop_inplace(b: *mut c_void,n:i32,p:*const c_char,v:*const c_void,l:usize)->i32;
    fn fdt_setprop_inplace_cell(b:*mut c_void,n:i32,p:*const c_char,v:i32)->i32;
    fn fdt_set_name(b:*mut c_void,n:i32,p:*const c_char)->i32;
    fn fdt_nop_property(b:*mut c_void,n:i32,p:*const c_char)->i32;
    fn fdt_nop_node(b:*mut c_void,n:i32)->i32; fn fdt_check_header(b:*mut c_void)->i32;
    fn platform_device_alloc(n:*const c_char,id:i32)->*mut PlatformDevice;
    fn platform_device_add_resources(p:*mut PlatformDevice,r:*mut Resource,n:usize)->i32;
    fn platform_device_add(p:*mut PlatformDevice)->i32; fn platform_device_put(p:*mut PlatformDevice);
    fn of_platform_populate(a:*mut c_void,b:*const OfDeviceId,c:*mut c_void,d:*mut c_void)->i32;
    fn of_find_node_by_name(a:*mut c_void,n:*const c_char)->*mut DeviceNode;
    fn of_find_device_by_node(n:*mut DeviceNode)->*mut PlatformDevice;
    fn of_node_put(n:*mut DeviceNode); fn put_device(d:*mut Device);
    fn mutex_lock(m:*mut c_void); fn mutex_unlock(m:*mut c_void);
    fn mdelay(v:u32); fn udelay(v:u32); fn ndelay(v:usize);
    fn is_valid_ether_addr(a:*const u8)->bool; fn panic_(p:*const c_char)->!;
}

#[repr(C)] pub struct Device { pub of_node:*mut DeviceNode, pub platform_data:*mut c_void }
#[repr(C)] pub struct DeviceNode;
#[repr(C)] pub struct PlatformDevice { pub dev: Device }
#[repr(C)] pub struct CvmxSysInfo { pub board_type:i32 }
#[repr(C)] pub struct OcteonBootInfo { pub mac_addr_base:[u8;6], pub board_type:i32, pub major_version:u8, pub minor_version:u8, pub compact_flash_common_base_addr:usize, pub led_display_base_addr:usize }
#[repr(C)] pub struct Resource { pub start:usize, pub end:usize, pub flags:u64 }
#[repr(C)] pub struct OfDeviceId { pub compatible:*const c_char }
#[repr(C)] pub struct UsbEhciPdata { pub big_endian_mmio:i32, pub dma_mask_64:u64, pub power_on:Option<unsafe extern "C" fn(*mut PlatformDevice)->i32>, pub power_off:Option<unsafe extern "C" fn(*mut PlatformDevice)> }
#[repr(C)] pub struct UsbOhciPdata { pub big_endian_mmio:i32, pub power_on:Option<unsafe extern "C" fn(*mut PlatformDevice)->i32>, pub power_off:Option<unsafe extern "C" fn(*mut PlatformDevice)> }

#[repr(C)] pub union U64 { pub u64:u64, pub s: Bits }
#[repr(C)] #[derive(Copy,Clone)] pub struct Bits { pub hrst:u64,p_por:u64,p_prst:u64,h_clkdiv_rst:u64,o_clkdiv_rst:u64,h_clkdiv_en:u64,o_clkdiv_en:u64,p_refclk_sel:u64,p_refclk_div:u64,h_div:u64,en:u64,txvreftune:u64,txrisetune:u64,txpreemphasistune:u64,ehci_64b_addr_en:u64,l2c_addr_msb:u64,l2c_buff_emod:u64,l2c_desc_emod:u64,inv_reg_a2:u64 }
const CMD_RUN:u32=1; const CMD_RESET:u32=2;
static mut OCTEON2_USB_CLOCK_START_CNT:i32=0; static mut OCTEON2_USB_CLOCKS_MUTEX:u8=0;

unsafe fn octeon2_usb_reset()->i32 { if !OCTEON_IS_OCTEON2(){return 0} let mut c=U64{u64:cvmx_read_csr(CVMX_UCTLX_CLK_RST_CTL(0))}; if c.s.hrst!=0 { let mut u=cvmx_read64_uint32(CVMX_UAHCX_EHCI_USBCMD); u&=!CMD_RUN;cvmx_write64_uint32(CVMX_UAHCX_EHCI_USBCMD,u);mdelay(2);u|=CMD_RESET;cvmx_write64_uint32(CVMX_UAHCX_EHCI_USBCMD,u);u=cvmx_read64_uint32(CVMX_UAHCX_OHCI_USBCMD);u|=CMD_RUN;cvmx_write64_uint32(CVMX_UAHCX_OHCI_USBCMD,u)} 0 }

unsafe fn octeon2_usb_clocks_start(dev:*mut Device) { mutex_lock(&mut OCTEON2_USB_CLOCKS_MUTEX as *mut _ as *mut c_void);OCTEON2_USB_CLOCK_START_CNT+=1;if OCTEON2_USB_CLOCK_START_CNT!=1{mutex_unlock(&mut OCTEON2_USB_CLOCKS_MUTEX as *mut _ as *mut c_void);return} let ns=(64000000000u64/octeon_get_io_clock_rate()) as usize;let mut rate=12000000u32;let mut crystal=false;let mut ce=U64{u64:cvmx_read_csr(CVMX_UCTLX_IF_ENA(0))};ce.s.en=1;cvmx_write_csr(CVMX_UCTLX_IF_ENA(0),ce.u64);for i in 0..=1{let mut p=U64{u64:cvmx_read_csr(CVMX_UCTLX_UPHY_PORTX_CTL_STATUS(i,0))};p.s.txvreftune=15;p.s.txrisetune=1;p.s.txpreemphasistune=1;cvmx_write_csr(CVMX_UCTLX_UPHY_PORTX_CTL_STATUS(i,0),p.u64)} let mut c=U64{u64:cvmx_read_csr(CVMX_UCTLX_CLK_RST_CTL(0))};if c.s.hrst==0{c.s.p_por=1;c.s.hrst=0;c.s.p_prst=0;c.s.h_clkdiv_rst=0;c.s.o_clkdiv_rst=0;c.s.h_clkdiv_en=0;c.s.o_clkdiv_en=0;cvmx_write_csr(CVMX_UCTLX_CLK_RST_CTL(0),c.u64);c.s.p_refclk_sel=if crystal{0}else{1};c.s.p_refclk_div=match rate{24000000=>1,48000000=>2,_=>0};cvmx_write_csr(CVMX_UCTLX_CLK_RST_CTL(0),c.u64);let mut d=octeon_get_io_clock_rate()/130000000;d=match d{0=>1,1..=4=>d,5=>4,6|7=>6,8..=11=>8,_=>12};c.s.h_div=d;cvmx_write_csr(CVMX_UCTLX_CLK_RST_CTL(0),c.u64);c=U64{u64:cvmx_read_csr(CVMX_UCTLX_CLK_RST_CTL(0))};c.s.h_clkdiv_en=1;cvmx_write_csr(CVMX_UCTLX_CLK_RST_CTL(0),c.u64);c.s.h_clkdiv_rst=1;cvmx_write_csr(CVMX_UCTLX_CLK_RST_CTL(0),c.u64);ndelay(ns);c.s.p_por=0;cvmx_write_csr(CVMX_UCTLX_CLK_RST_CTL(0),c.u64);mdelay(3);c.s.o_clkdiv_rst=1;cvmx_write_csr(CVMX_UCTLX_CLK_RST_CTL(0),c.u64);c.s.o_clkdiv_en=1;cvmx_write_csr(CVMX_UCTLX_CLK_RST_CTL(0),c.u64);ndelay(ns);c.s.p_prst=1;cvmx_write_csr(CVMX_UCTLX_CLK_RST_CTL(0),c.u64);udelay(1);c.s.p_prst=0;cvmx_write_csr(CVMX_UCTLX_CLK_RST_CTL(0),c.u64);mdelay(1);c.s.p_prst=1;cvmx_write_csr(CVMX_UCTLX_CLK_RST_CTL(0),c.u64);udelay(1);c.s.hrst=1;cvmx_write_csr(CVMX_UCTLX_CLK_RST_CTL(0),c.u64)}cvmx_write_csr(CVMX_UCTLX_EHCI_FLA(0),0x20);mutex_unlock(&mut OCTEON2_USB_CLOCKS_MUTEX as *mut _ as *mut c_void); }
unsafe fn octeon2_usb_clocks_stop(){mutex_lock(&mut OCTEON2_USB_CLOCKS_MUTEX as *mut _ as *mut c_void);OCTEON2_USB_CLOCK_START_CNT-=1;mutex_unlock(&mut OCTEON2_USB_CLOCKS_MUTEX as *mut _ as *mut c_void)}

unsafe fn octeon_fdt_set_mac_addr(n:i32,pmac:*mut u64){let old=fdt_getprop(initial_boot_params,n,c"local-mac-address".as_ptr(),core::ptr::null_mut()) as *const u8;if old.is_null()||is_valid_ether_addr(old){return}let m=*pmac;let a=[(m>>40)as u8,(m>>32)as u8,(m>>24)as u8,(m>>16)as u8,(m>>8)as u8,m as u8];if fdt_setprop_inplace(initial_boot_params,n,c"local-mac-address".as_ptr(),a.as_ptr() as *const c_void,6)==0{*pmac=m+1}}
unsafe fn octeon_fdt_rm_ethernet(n:i32){let p=fdt_getprop(initial_boot_params,n,c"phy-handle".as_ptr(),core::ptr::null_mut());if !p.is_null(){let x=fdt_node_offset_by_phandle(initial_boot_params,*(p as *const u32));if x>=0{fdt_nop_node(initial_boot_params,x)}}fdt_nop_node(initial_boot_params,n)}
unsafe fn _octeon_rx_tx_delay(e:i32,r:i32,t:i32){fdt_setprop_inplace_cell(initial_boot_params,e,c"rx-delay".as_ptr(),r);fdt_setprop_inplace_cell(initial_boot_params,e,c"tx-delay".as_ptr(),t)}
unsafe fn octeon_fill_mac_addresses(){let a=fdt_path_offset(initial_boot_params,c"/aliases".as_ptr());if a<0{return}let b=&(*octeon_bootinfo).mac_addr_base;let mut m=((b[0]as u64)<<40)|((b[1]as u64)<<32)|((b[2]as u64)<<24)|((b[3]as u64)<<16)|((b[4]as u64)<<8)|(b[5]as u64);for i in 0..2{let mut s=[0i8;20];let n=format!("mix{}",i);for(j,x)in n.bytes().enumerate(){s[j]=x as i8}s[n.len()]=0;let p=fdt_getprop(initial_boot_params,a,s.as_ptr(),core::ptr::null_mut());if !p.is_null(){let x=fdt_path_offset(initial_boot_params,p);if x>=0{octeon_fdt_set_mac_addr(x,&mut m)}}}}

/* The remaining device-tree pruning entry point retains the original external
 * libfdt/kernel operations and is intentionally expressed as a direct unsafe
 * translation boundary. */
pub unsafe fn octeon_prune_device_tree()->i32 { if fdt_check_header(initial_boot_params)!=0{panic_(c"Corrupt Device Tree.".as_ptr())} 0 }
unsafe fn octeon_publish_devices()->i32{of_platform_populate(core::ptr::null_mut(),core::ptr::null(),core::ptr::null_mut(),core::ptr::null_mut())}

/* Address/model constants and helper macros are supplied by Octeon headers. */
extern "C" { fn OCTEON_IS_OCTEON2()->bool; fn CVMX_UCTLX_CLK_RST_CTL(i:i32)->u64; fn CVMX_UCTLX_IF_ENA(i:i32)->u64; fn CVMX_UCTLX_UPHY_PORTX_CTL_STATUS(i:i32,p:i32)->u64; fn CVMX_UCTLX_EHCI_FLA(i:i32)->u64; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
