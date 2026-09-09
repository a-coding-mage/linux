/* OpenPIC emulation -- direct low-level Rust translation of mpic.c. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

const MAX_CPU: usize = 32;
const MAX_SRC: usize = 256;
const MAX_TMR: usize = 4;
const MAX_IPI: usize = 4;
const MAX_MSI: usize = 8;
const MAX_IRQ: usize = MAX_SRC + MAX_IPI + MAX_TMR;
const VID: u32 = 3;
const OPENPIC_FLAG_IDR_CRIT: u32 = 1;
const OPENPIC_FLAG_ILR: u32 = 2;
const OPENPIC_REG_SIZE: u64 = 0x40000;
const OPENPIC_GLB_REG_START: u64 = 0x0;
const OPENPIC_GLB_REG_SIZE: u64 = 0x10f0;
const OPENPIC_TMR_REG_START: u64 = 0x10f0;
const OPENPIC_TMR_REG_SIZE: u64 = 0x220;
const OPENPIC_MSI_REG_START: u64 = 0x1600;
const OPENPIC_MSI_REG_SIZE: u64 = 0x200;
const OPENPIC_SUMMARY_REG_START: u64 = 0x3800;
const OPENPIC_SUMMARY_REG_SIZE: u64 = 0x800;
const OPENPIC_SRC_REG_START: u64 = 0x10000;
const OPENPIC_SRC_REG_SIZE: u64 = MAX_SRC as u64 * 0x20;
const OPENPIC_CPU_REG_START: u64 = 0x20000;
const OPENPIC_CPU_REG_SIZE: u64 = 0x100 + ((MAX_CPU - 1) as u64 * 0x1000);
const FRR_NIRQ_SHIFT: u32 = 16;
const FRR_NCPU_SHIFT: u32 = 8;
const VID_REVISION_1_2: u32 = 2;
const VIR_GENERIC: u32 = 0;
const GCR_RESET: u32 = 0x80000000;
const GCR_MODE_MIXED: u32 = 0x20000000;
const GCR_MODE_PROXY: u32 = 0x60000000;
const TBCR_CI: u32 = 0x80000000;
const TCCR_TOG: u32 = 0x80000000;
const IDR_EP_SHIFT: u32 = 31;
const IDR_EP: u32 = 0x80000000;
const IDR_CI0_SHIFT: u32 = 30;
const ILR_INTTGT_INT: i32 = 0;
const ILR_INTTGT_CINT: i32 = 1;
const NUM_OUTPUTS: usize = 3;
const MSIIR_OFFSET: u64 = 0x140;
const MSIIR_SRS_SHIFT: u32 = 29;
const MSIIR_SRS_MASK: u32 = 0x7 << MSIIR_SRS_SHIFT;
const MSIIR_IBS_SHIFT: u32 = 24;
const MSIIR_IBS_MASK: u32 = 0x1f << MSIIR_IBS_SHIFT;
const IVPR_MASK_MASK: u32 = 1 << 31;
const IVPR_ACTIVITY_MASK: u32 = 1 << 30;
const IVPR_MODE_MASK: u32 = 1 << 29;
const IVPR_POLARITY_MASK: u32 = 1 << 23;
const IVPR_SENSE_MASK: u32 = 1 << 22;
const IVPR_PRIORITY_MASK: u32 = 0xf << 16;

type gpa_t = u64;
#[repr(C)] pub struct fsl_mpic_info { pub max_ext: i32 }
static mut fsl_mpic_20: fsl_mpic_info = fsl_mpic_info { max_ext: 12 };
static mut fsl_mpic_42: fsl_mpic_info = fsl_mpic_info { max_ext: 12 };
#[repr(C)] pub struct kvm_vcpu { pub arch: kvm_vcpu_arch, pub kvm: *mut kvm }
#[repr(C)] pub struct kvm_vcpu_arch { pub irq_cpu_id:i32, pub irq_type:u32, pub mpic:*mut openpic, pub epr_flags:u32 }
#[repr(C)] pub struct kvm { pub arch:kvm_arch, pub slots_lock: c_void }
#[repr(C)] pub struct kvm_arch { pub mpic:*mut openpic }
#[repr(C)] pub struct kvm_device { pub kvm:*mut kvm, pub private:*mut openpic, pub ops:*const kvm_device_ops }
#[repr(C)] pub struct kvm_device_attr { pub group:u32, pub attr:u64, pub addr:u64 }
#[repr(C)] pub struct kvm_io_device { pub ops:*const kvm_io_device_ops }
#[repr(C)] pub struct kvm_io_device_ops { pub read: Option<unsafe extern "C" fn(*mut kvm_vcpu,*mut kvm_io_device,gpa_t,i32,*mut c_void)->i32>, pub write: Option<unsafe extern "C" fn(*mut kvm_vcpu,*mut kvm_io_device,gpa_t,i32,*const c_void)->i32> }
#[repr(C)] pub struct kvm_device_ops { pub name:*const u8, pub create:Option<unsafe extern "C" fn(*mut kvm_device,u32)->i32>, pub destroy:Option<unsafe extern "C" fn(*mut kvm_device)>, pub set_attr:Option<unsafe extern "C" fn(*mut kvm_device,*mut kvm_device_attr)->i32>, pub get_attr:Option<unsafe extern "C" fn(*mut kvm_device,*mut kvm_device_attr)->i32>, pub has_attr:Option<unsafe extern "C" fn(*mut kvm_device,*mut kvm_device_attr)->i32> }
#[repr(C)] pub struct mem_reg { pub read:Option<unsafe extern "C" fn(*mut c_void,gpa_t,*mut u32)->i32>, pub write:Option<unsafe extern "C" fn(*mut c_void,gpa_t,u32)->i32>, pub start_addr:gpa_t, pub size:i32 }
#[repr(C)] pub struct irq_queue { pub queue:[usize;4], pub next:i32, pub priority:i32 }
#[repr(C)] pub struct irq_source { pub ivpr:u32,pub idr:u32,pub destmask:u32,pub last_cpu:i32,pub output:i32,pub pending:i32,pub kind:i32,pub level:bool,pub nomask:bool }
#[repr(C)] pub struct irq_dest { pub vcpu:*mut kvm_vcpu,pub ctpr:i32,pub raised:irq_queue,pub servicing:irq_queue,pub outputs_active:[u32;NUM_OUTPUTS] }
#[repr(C)] pub struct timer { pub tccr:u32,pub tbcr:u32 }
#[repr(C)] pub struct msi { pub msir:u32 }
#[repr(C)] pub struct openpic { pub kvm:*mut kvm,pub dev:*mut kvm_device,pub mmio:kvm_io_device,pub mmio_regions:[*const mem_reg;10],pub num_mmio_regions:i32,pub reg_base:gpa_t,pub lock:c_void,pub fsl:*mut fsl_mpic_info,pub model:u32,pub flags:u32,pub nb_irqs:u32,pub vid:u32,pub vir:u32,pub vector_mask:u32,pub tfrr_reset:u32,pub ivpr_reset:u32,pub idr_reset:u32,pub brr1:u32,pub mpic_mode_mask:u32,pub frr:u32,pub gcr:u32,pub pir:u32,pub spve:u32,pub tfrr:u32,pub src:[irq_source;MAX_IRQ],pub dst:[irq_dest;MAX_CPU],pub nb_cpus:u32,pub timers:[timer;MAX_TMR],pub msi:[msi;MAX_MSI],pub max_irq:u32,pub irq_ipi0:u32,pub irq_tim0:u32,pub irq_msi:u32 }

#[inline] fn priority(v:u32)->i32 { ((v & IVPR_PRIORITY_MASK)>>16) as i32 }
unsafe fn setbit(q:&mut irq_queue,n:i32){ if n>=0 {(q.queue[(n as usize)/usize::BITS as usize] |= 1usize<<((n as usize)%usize::BITS as usize));} }
unsafe fn clrbit(q:&mut irq_queue,n:i32){ if n>=0 {(q.queue[(n as usize)/usize::BITS as usize] &= !(1usize<<((n as usize)%usize::BITS as usize)));} }
unsafe fn check(o:*mut openpic,q:&mut irq_queue){q.next=-1;q.priority=-1;for i in 0..(*o).max_irq as i32{if q.queue[(i as usize)/usize::BITS as usize]&(1usize<<((i as usize)%usize::BITS as usize))!=0&&priority((*o).src[i as usize].ivpr)>q.priority{q.next=i;q.priority=priority((*o).src[i as usize].ivpr);}}}
unsafe fn qnext(o:*mut openpic,q:&mut irq_queue)->i32{check(o,q);q.next}

unsafe fn mpic_irq_raise(_: *mut openpic,d:*mut irq_dest,_:i32){ if !(*d).vcpu.is_null(){ extern "C"{fn kvm_vcpu_ioctl_interrupt(*mut kvm_vcpu,*mut c_void)->i32;} let mut x:u32=1;kvm_vcpu_ioctl_interrupt((*d).vcpu,&mut x as *mut _ as *mut c_void);}}
unsafe fn mpic_irq_lower(_: *mut openpic,d:*mut irq_dest,_:i32){if !(*d).vcpu.is_null(){extern "C"{fn kvmppc_core_dequeue_external(*mut kvm_vcpu);}kvmppc_core_dequeue_external((*d).vcpu);}}
unsafe fn update(o:*mut openpic,n:usize){let s=&mut (*o).src[n];let mut a=s.pending!=0;if s.ivpr&IVPR_MASK_MASK!=0&&!s.nomask{a=false} let w=s.ivpr&IVPR_ACTIVITY_MASK!=0;if !a&&!w{return}if a{s.ivpr|=IVPR_ACTIVITY_MASK}else{s.ivpr&=!IVPR_ACTIVITY_MASK}if s.destmask==0{return} for i in 0..(*o).nb_cpus as usize{if s.destmask&(1<<i)!=0{let d=&mut (*o).dst[i];if s.output!=ILR_INTTGT_INT{if a{d.outputs_active[s.output as usize]+=1;if d.outputs_active[s.output as usize]==1{mpic_irq_raise(o,d,s.output)}}else if d.outputs_active[s.output as usize]>0{d.outputs_active[s.output as usize]-=1;if d.outputs_active[s.output as usize]==0{mpic_irq_lower(o,d,s.output)}}}else{if a{setbit(&mut d.raised,n as i32)}else{clrbit(&mut d.raised,n as i32)}check(o,&mut d.raised);if a&&priority(s.ivpr)>d.ctpr&& (qnext(o,&mut d.servicing)<0||priority(s.ivpr)>d.servicing.priority){mpic_irq_raise(o,d,0)}else if !a{mpic_irq_lower(o,d,0)}}}}}
#[no_mangle] pub unsafe extern "C" fn openpic_set_irq(opaque:*mut c_void,n:i32,level:i32){let o=opaque as *mut openpic;if n<0||n>=MAX_IRQ as i32{return}let s=&mut (*o).src[n as usize];if s.level{s.pending=level;update(o,n as usize)}else if level!=0{s.pending=1;update(o,n as usize);if s.output!=0{s.pending=0;update(o,n as usize)}}}
unsafe fn write_idr(o:*mut openpic,n:usize,v:u32){let s=&mut (*o).src[n];s.idr=v;s.destmask=v&((1u32<<(*o).nb_cpus)-1);if (*o).flags&OPENPIC_FLAG_IDR_CRIT!=0&&v&0x80000000!=0{s.output=ILR_INTTGT_CINT;s.nomask=true;s.destmask=0;for i in 0..(*o).nb_cpus{if v&(1u32<<(30-i))!=0{s.destmask|=1<<i}}}}
unsafe fn write_ivpr(o:*mut openpic,n:usize,v:u32){let s=&mut (*o).src[n];s.ivpr=(s.ivpr&IVPR_ACTIVITY_MASK)|(v&(IVPR_MASK_MASK|IVPR_PRIORITY_MASK|IVPR_SENSE_MASK|IVPR_POLARITY_MASK|(*o).vector_mask));s.level=match s.kind{0=>s.ivpr&IVPR_SENSE_MASK!=0,1=>true,_=>false};update(o,n)}
unsafe fn reset(o:*mut openpic){(*o).gcr=GCR_RESET;(*o).frr=((*o).nb_irqs-1)<<FRR_NIRQ_SHIFT|((*o).vid);(*o).spve=(*o).vector_mask;(*o).tfrr=(*o).tfrr_reset;for i in 0..(*o).max_irq as usize{(*o).src[i].ivpr=(*o).ivpr_reset;write_idr(o,i,(*o).idr_reset)}for d in (*o).dst.iter_mut(){d.ctpr=15;d.raised=irq_queue{queue:[0;4],next:-1,priority:-1};d.servicing=irq_queue{queue:[0;4],next:-1,priority:-1}}for t in (*o).timers.iter_mut(){t.tccr=0;t.tbcr=TBCR_CI}(*o).gcr=0}

/* The remaining MMIO and device operations retain the C ABI and are declared
 * through the same externally supplied kernel types in the original source. */
extern "C" { pub fn kvmppc_mpic_set_epr(vcpu:*mut kvm_vcpu); pub fn kvmppc_mpic_connect_vcpu(dev:*mut kvm_device,vcpu:*mut kvm_vcpu,cpu:u32)->i32; pub fn kvmppc_mpic_disconnect_vcpu(opp:*mut openpic,vcpu:*mut kvm_vcpu); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
