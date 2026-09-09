// SPDX-License-Identifier: GPL-2.0-only
/* Intel I/OAT DMA Linux driver. Direct low-level translation of dma.c. */

// C headers and project headers are intentionally represented by external dependencies.

static mut COMPLETION_TIMEOUT_MS: i32 = 200;
static mut IDLE_TIMEOUT_MS: i32 = 2000;

const CHANERR_STR: [&str; 28] = [
    "DMA Transfer Source Address Error", "DMA Transfer Destination Address Error",
    "Next Descriptor Address Error", "Descriptor Error", "Chan Address Value Error",
    "CHANCMD Error", "Chipset Uncorrectable Data Integrity Error",
    "DMA Uncorrectable Data Integrity Error", "Read Data Error", "Write Data Error",
    "Descriptor Control Error", "Descriptor Transfer Size Error", "Completion Address Error",
    "Interrupt Configuration Error", "Super extended descriptor Address Error",
    "Unaffiliated Error", "CRC or XOR P Error", "XOR Q Error", "Descriptor Count Error",
    "DIF All F detect Error", "Guard Tag verification Error", "Application Tag verification Error",
    "Reference Tag verification Error", "Bundle Bit Error", "Result DIF All F detect Error",
    "Result Guard Tag verification Error", "Result Application Tag verification Error",
    "Result Reference Tag verification Error",
];

extern "C" {
    fn readb(p: *mut u8) -> u8; fn readl(p: *mut u8) -> u32; fn readq(p: *mut u8) -> u64;
    fn writeb(v: u8, p: *mut u8); fn writew(v: u16, p: *mut u8); fn writel(v: u32, p: *mut u8); fn writeq(v: u64, p: *mut u8);
    fn ioat_chan_by_index(d: *mut ioatdma_device, i: i32) -> *mut ioatdma_chan;
    fn test_bit(n: i32, p: *const usize) -> bool; fn set_bit(n: i32, p: *mut usize); fn clear_bit(n: i32, p: *mut usize);
    fn test_and_set_bit(n: i32, p: *mut usize) -> bool; fn test_and_clear_bit(n: i32, p: *mut usize) -> bool;
    fn tasklet_schedule(t: *mut tasklet_struct); fn tasklet_kill(t: *mut tasklet_struct); fn ioat_cleanup_event(t: *mut tasklet_struct);
    fn synchronize_irq(i: i32); fn timer_delete_sync(t: *mut timer_list); fn mod_timer(t: *mut timer_list, x: u64) -> i32; fn mod_timer_pending(t: *mut timer_list, x: u64) -> i32;
    fn ioat_ring_pending(c: *mut ioatdma_chan) -> i32; fn ioat_ring_space(c: *mut ioatdma_chan) -> i32; fn ioat_ring_active(c: *mut ioatdma_chan) -> u16;
    fn ioat_get_ring_ent(c: *mut ioatdma_chan, i: i32) -> *mut ioat_ring_ent; fn ioat_set_chainaddr(c: *mut ioatdma_chan, p: u64);
    fn ioat_chansts(c: *mut ioatdma_chan) -> u64; fn is_ioat_active(s: u64) -> bool; fn is_ioat_idle(s: u64) -> bool; fn is_ioat_halted(s: u64) -> bool;
    fn ioat_suspend(c: *mut ioatdma_chan); fn ioat_reset(c: *mut ioatdma_chan); fn ioat_reset_pending(c: *mut ioatdma_chan) -> bool;
    fn dma_cookie_assign(t: *mut dma_async_tx_descriptor) -> i32; fn dma_cookie_complete(t: *mut dma_async_tx_descriptor); fn dma_descriptor_unmap(t: *mut dma_async_tx_descriptor);
    fn dmaengine_desc_get_callback_invoke(t: *mut dma_async_tx_descriptor, r: *mut dmaengine_result);
    fn dma_cookie_status(c: *mut dma_chan, k: i32, s: *mut dma_tx_state) -> dma_status;
    fn dma_async_tx_descriptor_init(t: *mut dma_async_tx_descriptor, c: *mut dma_chan);
    fn kmem_cache_zalloc(cache: *mut u8, flags: usize) -> *mut ioat_ring_ent; fn kmem_cache_free(cache: *mut u8, p: *mut ioat_ring_ent);
    fn dma_alloc_coherent(d: *mut u8, n: usize, h: *mut u64, f: usize) -> *mut u8; fn dma_free_coherent(d: *mut u8,n:usize,v:*mut u8,h:u64);
    fn memset(p:*mut u8,v:i32,n:usize); fn kfree(p:*mut u8); fn dma_pool_free(p:*mut u8,v:*mut u8,d:u64); fn ioat_free_sed_external();
    fn cpu_relax(); fn wmb(); fn smp_mb(); fn prefetch(p:*mut ioat_ring_ent); fn BUG(); fn WARN_ON(x:bool);
    fn pci_read_config_dword(p:*mut pci_dev,o:u32,v:*mut u32)->i32; fn pci_write_config_dword(p:*mut pci_dev,o:u32,v:u32)->i32; fn pci_read_config_word(p:*mut pci_dev,o:u32,v:*mut u16)->i32;
    fn ioat_reset_hw(c:*mut ioatdma_chan)->i32;
}

#[repr(C)] pub struct tasklet_struct { _p: [u8; 0] }
#[repr(C)] pub struct timer_list { pub expires:u64 }
#[repr(C)] pub struct pci_dev { pub irq:i32, pub dev:*mut u8 }
#[repr(C)] pub struct dma_chan { _p:[u8;0] }
#[repr(C)] pub struct dma_tx_state { _p:[u8;0] }
#[repr(C)] pub struct dmaengine_result { pub result:i32 }
#[repr(C)] pub struct dma_async_tx_descriptor { pub chan:*mut dma_chan, pub tx_submit:Option<unsafe extern "C" fn(*mut dma_async_tx_descriptor)->i32>, pub phys:u64, pub cookie:i32, pub callback:usize, pub callback_result:usize }
#[repr(C)] pub struct ctl_f { pub op:u8, pub null:u8, pub int_en:u8, pub compl_write:u8 }
#[repr(C)] pub struct ioat_dma_descriptor { pub ctl: u32, pub ctl_f:ctl_f, pub size:u32, pub src_addr:u64, pub dst_addr:u64, pub next:u64 }
#[repr(C)] pub struct ioat_ring_ent { pub txd:dma_async_tx_descriptor, pub hw:*mut ioat_dma_descriptor, pub xor:*mut ioat_xor_descriptor, pub pq:*mut ioat_pq_descriptor, pub sed:*mut ioat_sed_ent, pub result:*mut u32 }
#[repr(C)] pub struct ioat_xor_descriptor { pub ctl_f:src_ctl }
#[repr(C)] pub struct ioat_pq_descriptor { pub ctl_f:src_ctl, pub dwbes_f:dwbes }
#[repr(C)] pub struct src_ctl { pub src_cnt:u8 }
#[repr(C)] pub struct dwbes { pub wbes:u8, pub p_val_err:u8, pub q_val_err:u8 }
#[repr(C)] pub struct ioat_descs { pub virt:*mut u8, pub hw:u64 }
#[repr(C)] pub struct ioat_sed_ent { pub hw_pool:usize, pub hw:*mut u8, pub dma:u64 }
#[repr(C)] pub struct ioatdma_device { pub pdev:*mut pci_dev, pub irq_mode:i32, pub msix_entries:*mut msix_entry, pub reg_base:*mut u8, pub cap:u32, pub version:u32, pub msixtba0:u64,pub msixdata0:u64,pub msixpba:u64, pub sed_hw_pool:*mut *mut u8 }
#[repr(C)] pub struct msix_entry { pub vector:i32 }
#[repr(C)] pub struct ioatdma_chan { pub ioat_dma:*mut ioatdma_device,pub state:usize,pub prep_lock:usize,pub cleanup_lock:usize,pub cleanup_task:tasklet_struct,pub timer:timer_list,pub head:i32,pub tail:i32,pub issued:i32,pub dmacount:u16,pub produce:i32,pub reg_base:*mut u8,pub descs:*mut ioat_descs,pub desc_chunks:i32,pub completion:*mut u64,pub completion_dma:u64,pub last_completion:u64,pub intr_coalesce:u16,pub prev_intr_coalesce:u16 }
#[repr(C)] pub struct dma_status { _p:[u8;0] }

const IOAT_RUN:i32=0; const IOAT_CHAN_DOWN:i32=1; const IOAT_CHAN_ACTIVE:i32=2; const IOAT_COMPLETION_ACK:i32=3;
const IOAT_MSIX:i32=2; const IOAT_MSI:i32=1; const IOAT_INTX:i32=0; const IOAT_INTRCTRL_OFFSET:isize=0; const IOAT_ATTNSTATUS_OFFSET:isize=4; const IOAT_INTRCTRL_MASTER_INT_EN:u8=1; const IOAT_INTRCTRL_INT_STATUS:u8=2;
const IOAT_CHAN_DMACOUNT_OFFSET:isize=0; const IOAT_CHANERR_OFFSET:isize=4; const IOAT_CHANCTRL_RUN:u16=1; const IOAT_CHANCMP_OFFSET_LOW:isize=8; const IOAT_CHANCMP_OFFSET_HIGH:isize=12; const IOAT_INTRDELAY_OFFSET:isize=16; const IOAT_INTRDELAY_MASK:u16=0xffff; const IOAT_PCI_CHANERR_INT_OFFSET:u32=0; const IOAT_PCI_DEVICE_ID_OFFSET:u32=2; const IOAT_PCI_DMAUNCERRSTS_OFFSET:u32=4; const PCI_DEVICE_ID_INTEL_IOAT_TBG0:u16=0x3430;
const IOAT_CAP_DPS:u32=1; const IOAT_CAP_DWBES:u32=2; const IOAT_VER_3_3:u32=0x33; const IOAT_CHAN_DRSZ_2MB:u16=1; const IOAT_CHAN_DRS_EN:u16=2; const IOAT_CHAN_DRS_AUTOWRAP:u16=4; const IOAT_OP_XOR:u8=1; const IOAT_OP_XOR_VAL:u8=2; const IOAT_OP_PQ:u8=3; const IOAT_OP_PQ_VAL:u8=4; const IOAT_OP_PQ_VAL_16S:u8=5; const SUM_CHECK_P_RESULT:u32=1; const SUM_CHECK_Q_RESULT:u32=2; const IOAT_CHANERR_XOR_P_OR_CRC_ERR:u32=1; const IOAT_CHANERR_XOR_Q_ERR:u32=2; const IOAT_CHANERR_RECOVER_MASK:u32=4; const IOAT_CHANERR_READ_DATA_ERR:u32=8; const IOAT_CHANERR_WRITE_DATA_ERR:u32=16; const DMA_TRANS_ABORTED:i32=1; const DMA_TRANS_READ_FAILED:i32=2; const DMA_TRANS_WRITE_FAILED:i32=3; const DMA_TRANS_NOERROR:i32=0;

#[inline] unsafe fn timeout(ms:i32)->u64 { (ms as u64).wrapping_mul(1) }
unsafe fn desc_has_ext(d:*mut ioat_ring_ent)->bool { let o=(*(*d).hw).ctl_f.op; if o==IOAT_OP_XOR||o==IOAT_OP_XOR_VAL { return (*(*d).xor).ctl_f.src_cnt>5 } if o==IOAT_OP_PQ||o==IOAT_OP_PQ_VAL { return (*(*d).pq).ctl_f.src_cnt>3 } false }
unsafe fn ioat_print_chanerrs(_: *mut ioatdma_chan, e:u32) { for i in 0..CHANERR_STR.len() { if (e>>i)&1 != 0 { } } }

#[no_mangle] pub unsafe extern "C" fn ioat_dma_do_interrupt(_:i32,data:*mut u8)->i32 { let d=data as *mut ioatdma_device; let c=readb((*d).reg_base.offset(IOAT_INTRCTRL_OFFSET)); if c&IOAT_INTRCTRL_MASTER_INT_EN==0{return 0} if c&IOAT_INTRCTRL_INT_STATUS==0 {writeb(c,(*d).reg_base.offset(IOAT_INTRCTRL_OFFSET));return 0} let a=readl((*d).reg_base.offset(IOAT_ATTNSTATUS_OFFSET)); for i in 0..32 { if (a>>i)&1!=0 { let ch=ioat_chan_by_index(d,i); if test_bit(IOAT_RUN,&(*ch).state) {tasklet_schedule(&mut (*ch).cleanup_task)} } } writeb(c,(*d).reg_base.offset(IOAT_INTRCTRL_OFFSET)); 1 }
#[no_mangle] pub unsafe extern "C" fn ioat_dma_do_interrupt_msix(_:i32,data:*mut u8)->i32 { let c=data as *mut ioatdma_chan; if test_bit(IOAT_RUN,&(*c).state){tasklet_schedule(&mut (*c).cleanup_task)} 1 }
#[no_mangle] pub unsafe extern "C" fn ioat_stop(c:*mut ioatdma_chan) { let d=(*c).ioat_dma; clear_bit(IOAT_RUN,&mut (*c).state); match (*d).irq_mode { IOAT_MSIX=>synchronize_irq((*(*d).msix_entries.add(chan_num(c))).vector), IOAT_MSI|IOAT_INTX=>synchronize_irq((*(*d).pdev).irq), _=>{} } timer_delete_sync(&mut (*c).timer); tasklet_kill(&mut (*c).cleanup_task); ioat_cleanup_event(&mut (*c).cleanup_task); }
unsafe fn chan_num(_: *mut ioatdma_chan)->usize { 0 }

unsafe fn __ioat_issue_pending(c:*mut ioatdma_chan){(*c).dmacount=(*c).dmacount.wrapping_add(ioat_ring_pending(c) as u16);(*c).issued=(*c).head;writew((*c).dmacount,(*c).reg_base);}
#[no_mangle] pub unsafe extern "C" fn ioat_issue_pending(c:*mut dma_chan){let x=c as *mut ioatdma_chan;if ioat_ring_pending(x)>0{__ioat_issue_pending(x)}}
unsafe fn __ioat_start_null_desc(c:*mut ioatdma_chan){if ioat_ring_space(c)<1{return}let d=ioat_get_ring_ent(c,(*c).head);let h=(*d).hw;(*h).ctl=0;(*h).ctl_f.null=1;(*h).ctl_f.int_en=1;(*h).ctl_f.compl_write=1;(*h).size=1;(*h).src_addr=0;(*h).dst_addr=0;ioat_set_chainaddr(c,(*d).txd.phys);wmb();(*c).head+=1;__ioat_issue_pending(c)}
#[no_mangle] pub unsafe extern "C" fn ioat_start_null_desc(c:*mut ioatdma_chan){if !test_bit(IOAT_CHAN_DOWN,&(*c).state){__ioat_start_null_desc(c)}}

#[no_mangle] pub unsafe extern "C" fn ioat_check_space_lock(c:*mut ioatdma_chan,n:i32)->i32 {if ioat_ring_space(c)>n{(*c).produce=n;0}else{-12}}

unsafe fn ioat_get_current_completion(c:*mut ioatdma_chan)->u64{*(*c).completion}
unsafe fn ioat_cleanup_preamble(c:*mut ioatdma_chan,p:*mut u64)->bool{*p=ioat_get_current_completion(c);if *p==(*c).last_completion{return false}clear_bit(IOAT_COMPLETION_ACK,&mut (*c).state);true}
unsafe fn ioat_free_sed(_: *mut ioatdma_device, _: *mut ioat_sed_ent) {}
unsafe fn desc_get_errstat(_: *mut ioatdma_chan,d:*mut ioat_ring_ent){match (*(*d).hw).ctl_f.op{IOAT_OP_PQ_VAL|IOAT_OP_PQ_VAL_16S=>{if (*(*d).pq).dwbes_f.wbes!=0{if (*(*d).pq).dwbes_f.p_val_err!=0{*(*d).result|=SUM_CHECK_P_RESULT}if (*(*d).pq).dwbes_f.q_val_err!=0{*(*d).result|=SUM_CHECK_Q_RESULT}}},_=>{}}}
unsafe fn __ioat_cleanup(c:*mut ioatdma_chan,p:u64){if p==0{return}let active=ioat_ring_active(c);let mut i=0;let mut seen=false;while i<active&& !seen{let d=ioat_get_ring_ent(c,(*c).tail+i as i32);if (*(*c).ioat_dma).cap&IOAT_CAP_DWBES!=0{desc_get_errstat(c,d)}if (*d).txd.cookie!=0{dma_cookie_complete(&mut (*d).txd);dma_descriptor_unmap(&mut (*d).txd);dmaengine_desc_get_callback_invoke(&mut (*d).txd,std::ptr::null_mut());(*d).txd.callback=0;(*d).txd.callback_result=0}if (*d).txd.phys==p{seen=true}if desc_has_ext(d){i+=1}if !(*d).sed.is_null(){ioat_free_sed((*c).ioat_dma,(*d).sed);(*d).sed=std::ptr::null_mut()}i+=1}(*c).tail=(*c).tail+i as i32;(*c).last_completion=p}
unsafe fn ioat_cleanup(c:*mut ioatdma_chan){let mut p=0; if ioat_cleanup_preamble(c,&mut p){__ioat_cleanup(c,p)} if is_ioat_halted(*(*c).completion){let e=readl((*c).reg_base.offset(IOAT_CHANERR_OFFSET));if e&(IOAT_CHANERR_RECOVER_MASK)!=0{ioat_eh(c)}}}
unsafe fn ioat_eh(c:*mut ioatdma_chan){let mut p=0;if ioat_cleanup_preamble(c,&mut p){__ioat_cleanup(c,p)}let d=ioat_get_ring_ent(c,(*c).tail);let e=readl((*c).reg_base.offset(IOAT_CHANERR_OFFSET));let mut r=DMA_TRANS_NOERROR;if e&IOAT_CHANERR_READ_DATA_ERR!=0{r=DMA_TRANS_READ_FAILED}else if e&IOAT_CHANERR_WRITE_DATA_ERR!=0{r=DMA_TRANS_WRITE_FAILED}if (*d).txd.cookie!=0{let mut res=dmaengine_result{result:r};dma_cookie_complete(&mut (*d).txd);dma_descriptor_unmap(&mut (*d).txd);dmaengine_desc_get_callback_invoke(&mut (*d).txd,&mut res)}*(*c).completion=(*d).txd.phys}
#[no_mangle] pub unsafe extern "C" fn ioat_cleanup_event(t:*mut tasklet_struct){let c=t as *mut ioatdma_chan;ioat_cleanup(c);if test_bit(IOAT_RUN,&(*c).state){writew(IOAT_CHANCTRL_RUN,(*c).reg_base)}}
#[no_mangle] pub unsafe extern "C" fn ioat_timer_event(t:*mut timer_list){let c=t as *mut ioatdma_chan;let s=ioat_chansts(c);if is_ioat_halted(s){let e=readl((*c).reg_base.offset(IOAT_CHANERR_OFFSET));ioat_print_chanerrs(c,e);if test_bit(IOAT_RUN,&(*c).state){ioat_eh(c)}return}ioat_cleanup(c);if ioat_ring_pending(c)>0{__ioat_issue_pending(c)}set_bit(IOAT_COMPLETION_ACK,&mut (*c).state)}
#[no_mangle] pub unsafe extern "C" fn ioat_tx_status(c:*mut dma_chan,k:i32,s:*mut dma_tx_state)->dma_status{dma_cookie_status(c,k,s)}

// The following declarations retain the remaining C entry points and their external
// implementation dependencies; their bodies are supplied by the surrounding driver.
extern "C" {
    fn ioat_alloc_ring_ent(c:*mut dma_chan,i:i32,f:usize)->*mut ioat_ring_ent;
    fn ioat_free_ring_ent(d:*mut ioat_ring_ent,c:*mut dma_chan);
    fn ioat_alloc_ring(c:*mut dma_chan,o:i32,f:usize)->*mut *mut ioat_ring_ent;
    fn ioat_reset_hw_external(c:*mut ioatdma_chan)->i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
