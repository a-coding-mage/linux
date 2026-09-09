/* Rust translation of mips/alchemy/common/dbdma.c. */
/* Kernel headers and symbols are supplied by the surrounding platform. */

use core::ffi::c_void;

/* The following types, constants, macros, and functions originate in the
 * Linux Alchemy DBDMA headers and are intentionally left as dependencies. */

const DBDEV_TAB_SIZE: usize = 64;

static mut dbdma_initialized: i32 = 0;
static mut dbdma_gptr: *mut dbdma_global_t = KSEG1ADDR(AU1550_DBDMA_CONF_PHYS_ADDR) as *mut dbdma_global_t;
static mut dbdev_tab: *mut dbdev_tab_t = core::ptr::null_mut();
static mut chan_tab_ptr: [*mut chan_tab_t; NUM_DBDMA_CHANS] = [core::ptr::null_mut(); NUM_DBDMA_CHANS];
static mut au1xxx_dbdma_spin_lock: DEFINE_SPINLOCK_TYPE = DEFINE_SPINLOCK_INIT;

/* I couldn't find a macro that did this... */
#[inline]
unsafe fn align_addr(x: u32, a: u32) -> u32 { (x.wrapping_add(a.wrapping_sub(1))) & !(a.wrapping_sub(1)) }

static mut au1550_dbdev_tab: [dbdev_tab_t; 32] = [
    dbdev!(AU1550_DSCR_CMD0_UART0_TX, DEV_FLAGS_OUT, 0, 8, 0x11100004), dbdev!(AU1550_DSCR_CMD0_UART0_RX, DEV_FLAGS_IN, 0, 8, 0x11100000),
    dbdev!(AU1550_DSCR_CMD0_UART3_TX, DEV_FLAGS_OUT, 0, 8, 0x11400004), dbdev!(AU1550_DSCR_CMD0_UART3_RX, DEV_FLAGS_IN, 0, 8, 0x11400000),
    dbdev!(AU1550_DSCR_CMD0_DMA_REQ0, 0, 0, 0, 0), dbdev!(AU1550_DSCR_CMD0_DMA_REQ1, 0, 0, 0, 0), dbdev!(AU1550_DSCR_CMD0_DMA_REQ2, 0, 0, 0, 0), dbdev!(AU1550_DSCR_CMD0_DMA_REQ3, 0, 0, 0, 0),
    dbdev!(AU1550_DSCR_CMD0_USBDEV_RX0, DEV_FLAGS_IN, 4, 8, 0x10200000), dbdev!(AU1550_DSCR_CMD0_USBDEV_TX0, DEV_FLAGS_OUT, 4, 8, 0x10200004), dbdev!(AU1550_DSCR_CMD0_USBDEV_TX1, DEV_FLAGS_OUT, 4, 8, 0x10200008), dbdev!(AU1550_DSCR_CMD0_USBDEV_TX2, DEV_FLAGS_OUT, 4, 8, 0x1020000c),
    dbdev!(AU1550_DSCR_CMD0_USBDEV_RX3, DEV_FLAGS_IN, 4, 8, 0x10200010), dbdev!(AU1550_DSCR_CMD0_USBDEV_RX4, DEV_FLAGS_IN, 4, 8, 0x10200014),
    dbdev!(AU1550_DSCR_CMD0_PSC0_TX, DEV_FLAGS_OUT, 0, 0, 0x11a0001c), dbdev!(AU1550_DSCR_CMD0_PSC0_RX, DEV_FLAGS_IN, 0, 0, 0x11a0001c), dbdev!(AU1550_DSCR_CMD0_PSC1_TX, DEV_FLAGS_OUT, 0, 0, 0x11b0001c), dbdev!(AU1550_DSCR_CMD0_PSC1_RX, DEV_FLAGS_IN, 0, 0, 0x11b0001c), dbdev!(AU1550_DSCR_CMD0_PSC2_TX, DEV_FLAGS_OUT, 0, 0, 0x10a0001c), dbdev!(AU1550_DSCR_CMD0_PSC2_RX, DEV_FLAGS_IN, 0, 0, 0x10a0001c), dbdev!(AU1550_DSCR_CMD0_PSC3_TX, DEV_FLAGS_OUT, 0, 0, 0x10b0001c), dbdev!(AU1550_DSCR_CMD0_PSC3_RX, DEV_FLAGS_IN, 0, 0, 0x10b0001c),
    dbdev!(AU1550_DSCR_CMD0_PCI_WRITE, 0, 0, 0, 0), dbdev!(AU1550_DSCR_CMD0_NAND_FLASH, 0, 0, 0, 0), dbdev!(AU1550_DSCR_CMD0_MAC0_RX, DEV_FLAGS_IN, 0, 0, 0), dbdev!(AU1550_DSCR_CMD0_MAC0_TX, DEV_FLAGS_OUT, 0, 0, 0), dbdev!(AU1550_DSCR_CMD0_MAC1_RX, DEV_FLAGS_IN, 0, 0, 0), dbdev!(AU1550_DSCR_CMD0_MAC1_TX, DEV_FLAGS_OUT, 0, 0, 0), dbdev!(DSCR_CMD0_THROTTLE, DEV_FLAGS_ANYUSE, 0, 0, 0), dbdev!(DSCR_CMD0_ALWAYS, DEV_FLAGS_ANYUSE, 0, 0, 0),
];

unsafe fn find_dbdev_id(id: u32) -> *mut dbdev_tab_t {
    for i in 0..DBDEV_TAB_SIZE { let p = dbdev_tab.add(i); if (*p).dev_id == id { return p; } }
    core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn au1xxx_ddma_get_nextptr_virt(dp: *mut au1x_ddma_desc_t) -> *mut c_void {
    phys_to_virt(DSCR_GET_NXTPTR((*dp).dscr_nxtptr))
}

#[no_mangle]
pub unsafe extern "C" fn au1xxx_ddma_add_device(dev: *mut dbdev_tab_t) -> u32 {
    static mut new_id: u16 = 0x1000;
    let p = find_dbdev_id(!0);
    if p.is_null() { return 0; }
    core::ptr::copy_nonoverlapping(dev, p, 1);
    (*p).dev_id = DSCR_DEV2CUSTOM_ID(new_id as u32, (*dev).dev_id); new_id = new_id.wrapping_add(1); (*p).dev_id
}

#[no_mangle]
pub unsafe extern "C" fn au1xxx_ddma_del_device(devid: u32) { let p = find_dbdev_id(devid); if !p.is_null() { core::ptr::write_bytes(p, 0, 1); (*p).dev_id = !0; } }

#[no_mangle]
pub unsafe extern "C" fn au1xxx_dbdma_chan_alloc(srcid: u32, destid: u32, callback: Option<unsafe extern "C" fn(i32, *mut c_void)>, callparam: *mut c_void) -> u32 {
    if dbdma_initialized == 0 { return 0; }
    let stp = find_dbdev_id(srcid); let dtp = find_dbdev_id(destid); if stp.is_null() || dtp.is_null() { return 0; }
    if ((*stp).dev_flags & DEV_FLAGS_INUSE) != 0 && ((*stp).dev_flags & DEV_FLAGS_ANYUSE) == 0 { return 0; }
    if ((*dtp).dev_flags & DEV_FLAGS_INUSE) != 0 && ((*dtp).dev_flags & DEV_FLAGS_ANYUSE) == 0 { return 0; }
    (*stp).dev_flags |= DEV_FLAGS_INUSE; (*dtp).dev_flags |= DEV_FLAGS_INUSE;
    for i in 0..NUM_DBDMA_CHANS { if chan_tab_ptr[i].is_null() {
        let ctp = kmalloc_obj::<chan_tab_t>(GFP_ATOMIC); if ctp.is_null() { break; } chan_tab_ptr[i] = ctp;
        core::ptr::write_bytes(ctp, 0, 1); (*ctp).chan_index = i as i32; let dcp = KSEG1ADDR(AU1550_DBDMA_PHYS_ADDR).wrapping_add(0x100 * i as u32);
        (*ctp).chan_ptr = dcp as *mut au1x_dma_chan_t; (*ctp).chan_src = stp; (*ctp).chan_dest = dtp; (*ctp).chan_callback = callback; (*ctp).chan_callparam = callparam;
        let mut cfg = 0; if (*stp).dev_intlevel != 0 { cfg |= DDMA_CFG_SED; } if (*stp).dev_intpolarity != 0 { cfg |= DDMA_CFG_SP; } if (*dtp).dev_intlevel != 0 { cfg |= DDMA_CFG_DED; } if (*dtp).dev_intpolarity != 0 { cfg |= DDMA_CFG_DP; } if ((*stp).dev_flags | (*dtp).dev_flags) & DEV_FLAGS_SYNC != 0 { cfg |= DDMA_CFG_SYNC; } (*ctp).chan_ptr.as_mut().unwrap().ddma_cfg = cfg;
        wmb(); return (&mut chan_tab_ptr[i] as *mut *mut chan_tab_t) as u32;
    }}
    (*stp).dev_flags &= !DEV_FLAGS_INUSE; (*dtp).dev_flags &= !DEV_FLAGS_INUSE; 0
}

#[no_mangle]
pub unsafe extern "C" fn au1xxx_dbdma_set_devwidth(chanid: u32, bits: i32) -> u32 { let ctp = *(chanid as *mut *mut chan_tab_t); let mut rv=0; if (*(*ctp).chan_src).dev_flags & DEV_FLAGS_IN != 0 { rv=(*(*ctp).chan_src).dev_devwidth; (*(*ctp).chan_src).dev_devwidth=bits; } if (*(*ctp).chan_dest).dev_flags & DEV_FLAGS_OUT != 0 { rv=(*(*ctp).chan_dest).dev_devwidth; (*(*ctp).chan_dest).dev_devwidth=bits; } rv as u32 }

/* Descriptor construction and ring operations retain the original register
 * encodings and pointer choreography. */
#[no_mangle]
pub unsafe extern "C" fn au1xxx_dbdma_ring_alloc(chanid:u32, entries:i32)->u32 { let ctp=*(chanid as *mut *mut chan_tab_t); let stp=(*ctp).chan_src; let dtp=(*ctp).chan_dest; let base=kmalloc_objs::<au1x_ddma_desc_t>(entries as usize,GFP_KERNEL|GFP_DMA) as *mut au1x_ddma_desc_t; if base.is_null(){return 0;} (*ctp).chan_desc_base=base; let mut p=base; let mut cmd0=DSCR_CMD0_SID((*stp).dev_id)|DSCR_CMD0_DID((*dtp).dev_id)|DSCR_CMD0_IE|DSCR_CMD0_CV|DSCR_CMD0_ST(DSCR_CMD0_ST_NOCHANGE); if ((*stp).dev_flags&DEV_FLAGS_IN)!=0 {cmd0|=DSCR_CMD0_SN;} if ((*dtp).dev_flags&DEV_FLAGS_OUT)!=0 {cmd0|=DSCR_CMD0_DN;} for _ in 0..entries {(*p).dscr_cmd0=cmd0;(*p).dscr_cmd1=0;(*p).dscr_source0=(*stp).dev_physaddr;(*p).dscr_dest0=(*dtp).dev_physaddr;(*p).dscr_source1=0;(*p).dscr_dest1=0;(*p).dscr_stat=0;(*p).sw_context=0;(*p).sw_status=0;(*p).dscr_nxtptr=DSCR_NXTPTR(virt_to_phys(p.add(1)));p=p.add(1);} p=p.sub(1);(*p).dscr_nxtptr=DSCR_NXTPTR(virt_to_phys(base));(*ctp).get_ptr=base;(*ctp).put_ptr=base;(*ctp).cur_ptr=base;base as u32 }
/* The source also defines the SoC-specific AU1200/AU1300 device tables and
 * suspend/resume syscore registration; those declarations remain external
 * platform data in this isolated translation. */

#[no_mangle]
pub unsafe extern "C" fn au1xxx_dbdma_put_source(chanid:u32,buf:dma_addr_t,nbytes:i32,flags:u32)->u32 { let c=*(chanid as *mut *mut chan_tab_t); let p=(*c).put_ptr; if (*p).dscr_cmd0&DSCR_CMD0_V!=0{return 0;} (*p).dscr_source0=buf as u32;(*p).dscr_cmd1=nbytes as u32;if flags&DDMA_FLAGS_IE!=0{(*p).dscr_cmd0|=DSCR_CMD0_IE;}if flags&DDMA_FLAGS_NOIE!=0{(*p).dscr_cmd0&=!DSCR_CMD0_IE;}(*p).dscr_cmd0|=DSCR_CMD0_V;wmb();(*c).chan_ptr.as_mut().unwrap().ddma_dbell=0;(*c).put_ptr=phys_to_virt(DSCR_GET_NXTPTR((*p).dscr_nxtptr)) as *mut au1x_ddma_desc_t;nbytes as u32 }
#[no_mangle]
pub unsafe extern "C" fn au1xxx_dbdma_put_dest(chanid:u32,buf:dma_addr_t,nbytes:i32,flags:u32)->u32 { let c=*(chanid as *mut *mut chan_tab_t);let p=(*c).put_ptr;if (*p).dscr_cmd0&DSCR_CMD0_V!=0{return 0;}if flags&DDMA_FLAGS_IE!=0{(*p).dscr_cmd0|=DSCR_CMD0_IE;}if flags&DDMA_FLAGS_NOIE!=0{(*p).dscr_cmd0&=!DSCR_CMD0_IE;}(*p).dscr_dest0=buf as u32;(*p).dscr_cmd1=nbytes as u32;(*p).dscr_cmd0|=DSCR_CMD0_V;wmb();(*c).chan_ptr.as_mut().unwrap().ddma_dbell=0;(*c).put_ptr=phys_to_virt(DSCR_GET_NXTPTR((*p).dscr_nxtptr)) as *mut au1x_ddma_desc_t;nbytes as u32 }
#[no_mangle]
pub unsafe extern "C" fn au1xxx_dbdma_get_dest(chanid:u32,buf:*mut *mut c_void,nbytes:*mut i32)->u32 {let c=*(chanid as *mut *mut chan_tab_t);let p=(*c).get_ptr;if (*p).dscr_cmd0&DSCR_CMD0_V!=0{return 0;}*buf=phys_to_virt((*p).dscr_dest0);*nbytes=(*p).dscr_cmd1 as i32;let r=(*p).dscr_stat;(*c).get_ptr=phys_to_virt(DSCR_GET_NXTPTR((*p).dscr_nxtptr)) as *mut au1x_ddma_desc_t;r}
#[no_mangle] pub unsafe extern "C" fn au1xxx_dbdma_stop(id:u32){let c=*(id as *mut *mut chan_tab_t);let cp=&mut *(*c).chan_ptr;cp.ddma_cfg&=!DDMA_CFG_EN;wmb();for _ in 0..101{if cp.ddma_stat&DDMA_STAT_H!=0{break;}udelay(1);}cp.ddma_stat|=DDMA_STAT_DB|DDMA_STAT_V;wmb();}
#[no_mangle] pub unsafe extern "C" fn au1xxx_dbdma_start(id:u32){let c=*(id as *mut *mut chan_tab_t);let cp=&mut *(*c).chan_ptr;cp.ddma_desptr=virt_to_phys((*c).cur_ptr);cp.ddma_cfg|=DDMA_CFG_EN;wmb();cp.ddma_dbell=0;wmb();}
#[no_mangle] pub unsafe extern "C" fn au1xxx_dbdma_reset(id:u32){let c=*(id as *mut *mut chan_tab_t);au1xxx_dbdma_stop(id);(*c).get_ptr=(*c).chan_desc_base;(*c).put_ptr=(*c).chan_desc_base;(*c).cur_ptr=(*c).chan_desc_base;let first=(*c).chan_desc_base;let mut p=first;loop{(*p).dscr_cmd0&=!DSCR_CMD0_V;(*p).sw_status=0;p=phys_to_virt(DSCR_GET_NXTPTR((*p).dscr_nxtptr)) as *mut au1x_ddma_desc_t;if p==first{break;}}}
#[no_mangle] pub unsafe extern "C" fn au1xxx_get_dma_residue(id:u32)->u32{let c=*(id as *mut *mut chan_tab_t);let r=(*(*c).chan_ptr).ddma_bytecnt;wmb();r}
#[no_mangle] pub unsafe extern "C" fn au1xxx_dbdma_chan_free(id:u32){let c=*(id as *mut *mut chan_tab_t);let s=(*c).chan_src;let d=(*c).chan_dest;au1xxx_dbdma_stop(id);kfree((*c).cdb_membase as *mut c_void);(*s).dev_flags&=!DEV_FLAGS_INUSE;(*d).dev_flags&=!DEV_FLAGS_INUSE;chan_tab_ptr[(*c).chan_index as usize]=core::ptr::null_mut();kfree(c as *mut c_void);}
#[no_mangle] pub unsafe extern "C" fn au1xxx_dbdma_put_dscr(id:u32,ds:*mut au1x_ddma_desc_t)->u32{let c=*(id as *mut *mut chan_tab_t);let p=(*c).put_ptr;if (*p).dscr_cmd0&DSCR_CMD0_V!=0{return 0;}(*p).dscr_dest0=(*ds).dscr_dest0;(*p).dscr_source0=(*ds).dscr_source0;(*p).dscr_dest1=(*ds).dscr_dest1;(*p).dscr_source1=(*ds).dscr_source1;(*p).dscr_cmd1=(*ds).dscr_cmd1;(*p).dscr_cmd0=((*p).dscr_cmd0&!DSCR_CMD0_IE)|(*ds).dscr_cmd0|DSCR_CMD0_V;(*c).chan_ptr.as_mut().unwrap().ddma_dbell=0;(*c).put_ptr=phys_to_virt(DSCR_GET_NXTPTR((*p).dscr_nxtptr)) as *mut au1x_ddma_desc_t;(*ds).dscr_cmd1}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
