# SPDX-License-Identifier: GPL-2.0-only
"""Private module workloads over original C and genuine selected bindings."""
from pathlib import Path
import re

EXPORTS = ('__rb_erase_color', 'rb_insert_color', 'rb_erase', 'rb_erase_linked',
           '__rb_insert_augmented', 'rb_next', 'rb_prev', 'rb_replace_node',
           'rb_replace_node_rcu', 'rb_next_postorder', 'rb_first_postorder')
GPL_ONLY = {'rb_erase_linked'}
STEPS = 150000
DESCRIPTION = 'Independent selected rbtree differential and nullable ABI'
SIGNATURES = {
    **{name:('void', [('struct rb_node *','node'),('struct rb_root *','root')], '()',
              ['*mut bindings::rb_node','*mut bindings::rb_root'])
       for name in ('rb_insert_color','rb_erase')},
    **{name:('void', [('struct rb_node *','node'),('struct rb_root *','root'),
                     ('void (*{name})(struct rb_node *, struct rb_node *)','rotate')], '()',
              ['*mut bindings::rb_node','*mut bindings::rb_root','bindings::RbAugmentRotate'])
       for name in ('__rb_insert_augmented','__rb_erase_color')},
    'rb_erase_linked':('bool',[('struct rb_node_linked *','node'),('struct rb_root_linked *','root')],
                     'bool',['*mut bindings::rb_node_linked','*mut bindings::rb_root_linked']),
    **{name:('struct rb_node *',[('const struct rb_node *','node')], '*mut bindings::rb_node',
             ['*const bindings::rb_node']) for name in ('rb_next','rb_prev','rb_next_postorder')},
    'rb_first_postorder':('struct rb_node *',[('const struct rb_root *','root')], '*mut bindings::rb_node',
                          ['*const bindings::rb_root']),
    **{name:('void',[('struct rb_node *','victim'),('struct rb_node *','new'),('struct rb_root *','root')],
             '()',['*mut bindings::rb_node','*mut bindings::rb_node','*mut bindings::rb_root'])
       for name in ('rb_replace_node','rb_replace_node_rcu')},
}


def marker(caller):
    if caller not in ('c','rust'): raise ValueError('unknown rbtree caller')
    return f'LUPOS_RBTREE_{caller.upper()}_ABI_OK steps={STEPS} exports=11'.encode()


def wrappers(caller):
    result=[]
    for name in EXPORTS:
        ret,args,rret,rargs=SIGNATURES[name]
        values=', '.join(value for _,value in args)
        if caller=='c':
            declaration=', '.join(typ.format(name=value) if '{name}' in typ else typ+' '+value for typ,value in args)
            types=', '.join(typ.format(name='') for typ,_ in args)
            result.append(f'static noinline {ret} rbtree_call_{name}({declaration}) {{\n'
                f' {ret} (*volatile selected)({types}) = {name}; return selected({values});\n}}')
        else:
            declaration=', '.join(value+': '+typ for (_,value),typ in zip(args,rargs))
            types=', '.join(rargs)
            result.append(f'unsafe extern "C" {{ fn oracle_{name}({declaration}) -> {rret}; }}\n'
                '/// Invoke the actual public binding with its original C KCFI type.\n'
                '///\n/// # Safety\n/// Arguments satisfy the original operation contract.\n'
                '#[no_mangle]\n#[inline(never)]\n'
                f'pub unsafe extern "C" fn rbtree_call_{name}({declaration}) -> {rret} {{\n'
                f' let selected: unsafe extern "C" fn({types}) -> {rret} = bindings::{name};\n'
                f' unsafe {{ core::ptr::read_volatile(&selected)({values}) }}\n}}')
    return '\n'.join(result)


def reference_source(root):
    # The whole original source, not a production replacement or guessed API.
    return ('#include <linux/export.h>\n#undef EXPORT_SYMBOL\n#undef EXPORT_SYMBOL_GPL\n'
            '#define EXPORT_SYMBOL(x)\n#define EXPORT_SYMBOL_GPL(x)\n'+
            ''.join('#define '+name+' oracle_'+name+'\n' for name in EXPORTS)+
            '#include "'+str(root/'lib/rbtree.c')+'"\n'+
            (root/'scripts/tests/rbtree_native/oracle_tail.c').read_text())


def c_source(root):
    source=(root/'scripts/tests/rbtree_native/driver.c').read_text()
    if source.count('step < 50000')!=1 or source.count('exercise(false, true)')!=1:
        raise ValueError('original rbtree differential workload changed')
    source=source[source.index('#define N 127'):source.index('#if defined(__x86_64__)')]
    # The standalone fixture's Rust-header-only test remains in the native
    # checker. This C runtime caller owns the same full public differential,
    # NULL/partial-init/self-alias domains and original C header composition.
    start=source.index('/* Same key/max layout as the Rust header fixture')
    end=source.index('static int verify_node',start)
    source=source[:start]+source[end:]
    source=source.replace('    err = rust_inline_checks(); if (err) return err;\n','')
    source=source.replace('    err = named_callback_tests(); if (err) return err;\n','')
    source=source.replace('#include "domain_tail.c"',(root/'scripts/tests/rbtree_native/domain_tail.c').read_text())
    source=re.sub(r'\brust_(edge|add_linked|erase_aug|link_rcu|tags)\b',r'rbtree_header_\1',source)
    for name in EXPORTS:
        source=re.sub(r'\b'+name+r'\b','rbtree_call_'+name,source)
    bridge=r'''
struct rb_node *rbtree_header_edge(const struct rb_root *r, bool last) { return last ? rb_last(r) : rb_first(r); }
bool rbtree_header_add_linked(struct rb_node_linked *n, struct rb_root_linked *r,
                            bool (*less)(struct rb_node *, const struct rb_node *)) { return rb_add_linked(n,r,less); }
void rbtree_header_erase_aug(struct rb_node *n, struct rb_root *r, const struct rb_augment_callbacks *cb) { rb_erase_augmented(n,r,cb); }
void rbtree_header_link_rcu(struct rb_node *n, struct rb_node *p, struct rb_node **link) { rb_link_node_rcu(n,p,link); }
struct rb_node *rbtree_header_tags(struct rb_node *n, struct rb_node *p) { struct rb_node *old=rb_parent(n); rb_set_parent(n,p); return old; }
'''
    return ('#include <linux/module.h>\n#include <linux/rbtree_augmented.h>\n'+wrappers('c')+'\n'+source+bridge+
        '\nstatic int __init rbtree_abi_init(void) { int error=test_main();\n'
        ' if(error) { pr_err("LUPOS_RBTREE_FAIL line=%d\\n",error); return -EINVAL; }\n'
        ' pr_info("'+marker('c').decode()+'\\n"); return 0; }\n'
        'static void __exit rbtree_abi_exit(void) {}\nmodule_init(rbtree_abi_init);\nmodule_exit(rbtree_abi_exit);\n'
        'MODULE_LICENSE("GPL");\nMODULE_DESCRIPTION("'+DESCRIPTION+'");\n')


def header_bridge(root):
    return ('/// Only translated headers; no private tree-provider implementation.\npub mod headers {\n'
            '/// Original public header algorithms.\n#[path="'+str(root/'include/linux/rbtree_header.rs')+'"] pub mod declarations;\n'
            '/// Original augmentation header algorithms.\n#[path="'+str(root/'include/linux/rbtree_augmented_header.rs')+'"] pub mod augmented;\n}\n')


RUST_WORKLOAD = r'''
use bindings::{rb_node, rb_node_linked, rb_root, rb_root_linked, rb_augment_callbacks, RbAugmentRotate};
use core::ptr::{addr_of_mut, null_mut};
use headers::{declarations as h, augmented as a};
unsafe extern "C" {
    fn oracle_edge(root: *const rb_root, last: bool) -> *mut rb_node;
    fn oracle_add_linked(node: *mut rb_node_linked, root: *mut rb_root_linked,
        less: Option<unsafe extern "C" fn(*mut rb_node,*const rb_node)->bool>) -> bool;
    fn oracle_erase_aug(node: *mut rb_node, root: *mut rb_root, cb: *const rb_augment_callbacks);
}
const N: usize = 127;
const SLOTS: usize = 2*N;
#[repr(C)]
struct Item { link: rb_node_linked, id: i32, key: i32, sum: u32 }
struct State {
    nodes: [[Item; SLOTS]; 2], trees: [rb_root_linked; 2], active: [i32; N],
    calls: [usize; 2], trace: [[u32; 8192]; 2], side: usize, rng: u32, digest: usize,
}
// SAFETY: All fields accept zero; only the serialized module init accesses this
// private storage. No reference or tree node escapes init into the kernel.
static mut STATE: State = unsafe { core::mem::zeroed() };
fn state() -> *mut State { addr_of_mut!(STATE) }
unsafe fn node(s: usize, slot: usize) -> *mut rb_node {
    unsafe { addr_of_mut!((*state()).nodes[s][slot].link.node) }
}
unsafe fn item(n: *const rb_node) -> *mut Item { n.cast_mut().cast() }
unsafe fn id(n: *const rb_node) -> u32 { unsafe { if n.is_null() {0} else {(*item(n)).id as u32+1} } }
unsafe fn sum(n: *const rb_node) -> u32 { unsafe { if n.is_null() {0} else {(*item(n)).sum} } }
unsafe fn random_word() -> u32 {
    unsafe { let s=state(); (*s).rng ^= (*s).rng << 13; (*s).rng ^= (*s).rng >> 17;
        (*s).rng ^= (*s).rng << 5; (*s).rng }
}
unsafe fn record(kind:u32, x:*mut rb_node, y:*mut rb_node) {
    unsafe {
        let s=state(); let side=(*s).side; let n=(*s).calls[side];
        assert!(n+7<8192);
        let values=[kind,id(x),id(y),if x.is_null(){0}else{id((*x).rb_left)},
            if x.is_null(){0}else{id((*x).rb_right)},
            if x.is_null(){0}else{((*x).__rb_parent_color&3) as u32},
            if y.is_null(){0}else{((*y).__rb_parent_color&3) as u32}];
        for (i,value) in values.into_iter().enumerate() { (*s).trace[side][n+i]=value; }
        (*s).calls[side]+=7;
    }
}
unsafe extern "C" fn propagate(mut n:*mut rb_node, stop:*mut rb_node) {
    unsafe { record(1,n,stop); while n!=stop {
        let value=1+sum((*n).rb_left)+sum((*n).rb_right);
        if (*item(n)).sum==value {break;} (*item(n)).sum=value; n=h::rb_parent(n);
    } }
}
unsafe extern "C" fn copy_cb(old:*mut rb_node,new:*mut rb_node) {
    unsafe { record(2,old,new); (*item(new)).sum=(*item(old)).sum; }
}
unsafe extern "C" fn rotate(old:*mut rb_node,new:*mut rb_node) {
    unsafe { record(3,old,new); (*item(new)).sum=(*item(old)).sum;
        (*item(old)).sum=1+sum((*old).rb_left)+sum((*old).rb_right); }
}
unsafe extern "C" fn less(x:*mut rb_node,y:*const rb_node)->bool {
    unsafe { (*item(x)).key<(*item(y)).key }
}
static CALLBACKS:rb_augment_callbacks=rb_augment_callbacks{
    propagate:Some(propagate),copy:Some(copy_cb),rotate:Some(rotate)};
macro_rules! check { ($value:expr) => { if !$value { return line!() as i32; } }; }
unsafe fn verify_node(n:*mut rb_node,parent:*mut rb_node,lo:i32,hi:i32,aug:bool)->i32 {
    unsafe {
        if n.is_null() {return 1;}
        if h::rb_parent(n)!=parent || (*item(n)).key<=lo || (*item(n)).key>=hi {return -1;}
        if a::rb_is_red(n) && ((!(*n).rb_left.is_null() && a::rb_is_red((*n).rb_left)) ||
            (!(*n).rb_right.is_null() && a::rb_is_red((*n).rb_right))) {return -1;}
        if aug && (*item(n)).sum!=1+sum((*n).rb_left)+sum((*n).rb_right) {return -1;}
        let left=verify_node((*n).rb_left,n,lo,(*item(n)).key,aug);
        let right=verify_node((*n).rb_right,n,(*item(n)).key,hi,aug);
        if left<0 || left!=right {return -1;} left+i32::from(a::rb_is_black(n))
    }
}
unsafe fn verify(aug:bool,linked:bool)->i32 {
    unsafe {
        let s=state(); check!(id((*s).trees[0].rb_root.rb_node)==id((*s).trees[1].rb_root.rb_node));
        check!((*s).calls[0]==(*s).calls[1]);
        for i in 0..(*s).calls[0] {check!((*s).trace[0][i]==(*s).trace[1][i]);}
        for i in 0..N { let slot=(*s).active[i]; if slot<0 {continue;}
            let x=node(0,slot as usize); let y=node(1,slot as usize);
            check!(id((*x).rb_left)==id((*y).rb_left)); check!(id((*x).rb_right)==id((*y).rb_right));
            check!(id(h::rb_parent(x))==id(h::rb_parent(y)));
            check!((*x).__rb_parent_color&3==(*y).__rb_parent_color&3);
            if aug {check!((*item(x)).sum==(*item(y)).sum);}
            if linked {check!(id((*item(x)).link.prev.cast())==id((*item(y)).link.prev.cast()));
                check!(id((*item(x)).link.next.cast())==id((*item(y)).link.next.cast()));}
        }
        let root=addr_of_mut!((*s).trees[1].rb_root);
        check!((*root).rb_node.is_null() || a::rb_is_black((*root).rb_node));
        check!(verify_node((*root).rb_node,null_mut(),-1,N as i32,aug)>0);
        let original=addr_of_mut!((*s).trees[0].rb_root);
        let mut x=oracle_edge(original,false); let mut y=h::rb_first(root);
        let mut count=0; let mut previous=-1;
        while !x.is_null() || !y.is_null() {
            check!(id(x)==id(y)); check!((*item(y)).key>previous); previous=(*item(y)).key;
            x=oracle_rb_next(x); y=rbtree_call_rb_next(y); count+=1; check!(count<=N);
        }
        x=oracle_edge(original,true); y=h::rb_last(root); let mut i=0;
        while !x.is_null() || !y.is_null() {check!(id(x)==id(y)); x=oracle_rb_prev(x);
            y=rbtree_call_rb_prev(y); i+=1; check!(i<=N);}
        check!(i==count);
        x=oracle_rb_first_postorder(original); y=rbtree_call_rb_first_postorder(root); i=0;
        while !x.is_null() || !y.is_null() {check!(id(x)==id(y));
            (*s).digest=(*s).digest.wrapping_mul(33).wrapping_add(id(y) as usize);
            x=oracle_rb_next_postorder(x); y=rbtree_call_rb_next_postorder(y); i+=1; check!(i<=N);}
        check!(i==count);
        if linked {
            check!(id((*s).trees[0].rb_leftmost.cast())==id((*s).trees[1].rb_leftmost.cast()));
            let mut p=(*s).trees[1].rb_leftmost; let mut last=null_mut(); i=0;
            while !p.is_null() {check!((*p).prev==last); last=p; p=(*p).next; i+=1; check!(i<=N);}
            check!(i==count);
        }
        0
    }
}
unsafe fn insert(side:usize,slot:usize,aug:bool,linked:bool) {
    unsafe {
        let s=state(); let n=node(side,slot); let root=addr_of_mut!((*s).trees[side].rb_root);
        (*s).side=side;
        if linked {let n=n.cast(); let r=addr_of_mut!((*s).trees[side]);
            if side==1 {h::rb_add_linked(n,r,Some(less));}else{oracle_add_linked(n,r,Some(less));} return;}
        let mut link=addr_of_mut!((*root).rb_node); let mut parent=null_mut();
        while !(*link).is_null() {parent=*link; if aug {(*item(parent)).sum+=1;}
            link=if less(n,parent) {addr_of_mut!((*parent).rb_left)}else{addr_of_mut!((*parent).rb_right)};}
        (*item(n)).sum=1; h::rb_link_node_rcu(n,parent,link);
        if aug {let cb=RbAugmentRotate::from_option(Some(rotate));
            if side==1 {rbtree_call___rb_insert_augmented(n,root,cb);}else{oracle___rb_insert_augmented(n,root,cb);}}
        else if side==1 {rbtree_call_rb_insert_color(n,root);}else{oracle_rb_insert_color(n,root);}
    }
}
unsafe fn exercise(aug:bool,linked:bool)->i32 {
    unsafe {
        let s=state(); for i in 0..N {(*s).active[i]=-1;}
        for side in 0..2 {
            (*s).trees[side]=core::mem::zeroed();
            for slot in 0..SLOTS {let item=item(node(side,slot)); *item=core::mem::zeroed();
                (*item).id=slot as i32; (*item).key=(slot%N) as i32; h::rb_clear_linked_node(addr_of_mut!((*item).link));}
        }
        for _ in 0..50000 {
            let key=random_word() as usize%N; let slot=(*s).active[key]; let action=random_word();
            (*s).calls=[0;2];
            if slot<0 {(*s).active[key]=key as i32; for side in 0..2 {insert(side,key,aug,linked);}}
            else if action&3==0 && !linked {
                let slot=slot as usize; let new_slot=if slot<N {slot+N}else{slot-N};
                for side in 0..2 {let old=node(side,slot); let new=node(side,new_slot);
                    (*item(new)).sum=(*item(old)).sum; let root=addr_of_mut!((*s).trees[side].rb_root);
                    if action&4!=0 {if side==1 {rbtree_call_rb_replace_node_rcu(old,new,root);}else{oracle_rb_replace_node_rcu(old,new,root);}}
                    else if side==1 {rbtree_call_rb_replace_node(old,new,root);}else{oracle_rb_replace_node(old,new,root);}
                } (*s).active[key]=new_slot as i32;
            } else {
                for side in 0..2 {let n=node(side,slot as usize); (*s).side=side;
                    let root=addr_of_mut!((*s).trees[side]);
                    if linked {
                        let more=if side==1 {rbtree_call_rb_erase_linked(n.cast(),root)}else{oracle_rb_erase_linked(n.cast(),root)};
                        check!(more==!(*root).rb_leftmost.is_null()); check!(h::rb_empty_node(n));
                        check!((*item(n)).link.prev.is_null() && (*item(n)).link.next.is_null());
                    } else if aug {if side==1 {a::rb_erase_augmented(n,addr_of_mut!((*root).rb_root),&CALLBACKS);}
                        else {oracle_erase_aug(n,addr_of_mut!((*root).rb_root),&CALLBACKS);}}
                    else if side==1 {rbtree_call_rb_erase(n,addr_of_mut!((*root).rb_root));}
                    else {oracle_rb_erase(n,addr_of_mut!((*root).rb_root));}
                } (*s).active[key]=-1;
            }
            let error=verify(aug,linked); if error!=0 {return error;}
        } 0
    }
}
unsafe fn domains()->i32 {
    unsafe {
        let mut n=core::mem::MaybeUninit::<rb_node>::uninit(); let n=n.as_mut_ptr();
        h::rb_clear_node(n); check!(oracle_rb_next(n).is_null() && rbtree_call_rb_next(n).is_null());
        check!(oracle_rb_prev(n).is_null() && rbtree_call_rb_prev(n).is_null());
        (*n).__rb_parent_color=0; oracle___rb_insert_augmented(n,null_mut(),RbAugmentRotate::from_option(None));
        check!((*n).__rb_parent_color==1); (*n).__rb_parent_color=0;
        rbtree_call___rb_insert_augmented(n,null_mut(),RbAugmentRotate::from_option(None));
        check!((*n).__rb_parent_color==1); (*n).__rb_parent_color=0;
        rbtree_call_rb_insert_color(n,null_mut()); check!((*n).__rb_parent_color==1);
        check!(oracle_rb_next_postorder(n).is_null() && rbtree_call_rb_next_postorder(n).is_null());
        for side in 0..2 {
            let mut p:rb_node=core::mem::zeroed(); let mut sibling:rb_node=core::mem::zeroed();
            p.rb_right=&mut sibling; sibling.__rb_parent_color=&mut p as *mut rb_node as usize|1;
            if side==1 {rbtree_call___rb_erase_color(&mut p,null_mut(),RbAugmentRotate::from_option(None));}
            else {oracle___rb_erase_color(&mut p,null_mut(),RbAugmentRotate::from_option(None));}
            check!(a::rb_is_black(&p) && a::rb_is_red(&sibling));
            *n=core::mem::zeroed(); (*n).__rb_parent_color=1; let mut root=rb_root{rb_node:n};
            if side==1 {rbtree_call_rb_replace_node(n,n,&mut root); rbtree_call_rb_replace_node_rcu(n,n,&mut root);}
            else {oracle_rb_replace_node(n,n,&mut root);oracle_rb_replace_node_rcu(n,n,&mut root);}
            check!(root.rb_node==n && (*n).__rb_parent_color==1);
            // Nonroot replacement never accesses root and fully initializes new.
            let mut parent:rb_node=core::mem::zeroed(); parent.rb_left=n;
            (*n).__rb_parent_color=&mut parent as *mut rb_node as usize|1;
            let mut new=core::mem::MaybeUninit::<rb_node>::uninit();
            if side==1 {rbtree_call_rb_replace_node(n,new.as_mut_ptr(),null_mut());}
            else {oracle_rb_replace_node(n,new.as_mut_ptr(),null_mut());}
            check!(parent.rb_left==new.as_mut_ptr() && h::rb_parent(new.as_ptr())==&mut parent);
            // The leaf erase path reads only propagate; copy/rotate stay uninitialized.
            *n=core::mem::zeroed(); (*n).__rb_parent_color=1; root.rb_node=n;
            let mut cb=core::mem::MaybeUninit::<rb_augment_callbacks>::uninit();
            addr_of_mut!((*cb.as_mut_ptr()).propagate).write(Some(domain_noop));
            if side==1 {a::rb_erase_augmented(n,&mut root,cb.as_ptr());}
            else {oracle_erase_aug(n,&mut root,cb.as_ptr());}
            check!(root.rb_node.is_null());
        }
        check!(rbtree_call_rb_next_postorder(null_mut()).is_null());
        let root=rb_root{rb_node:null_mut()}; check!(rbtree_call_rb_first_postorder(&root).is_null());
        0
    }
}
unsafe extern "C" fn domain_noop(_a:*mut rb_node,_b:*mut rb_node) {}
/// Rust owns the full state machine, invariants, callback trace and comparisons.
///
/// # Safety
/// Called only by serialized private fixture initialization.
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn rbtree_rust_exercise()->ffi::c_int {
    unsafe {
        (*state()).rng=0x9328175; (*state()).digest=0;
        let error=domains(); if error!=0 {return error;}
        for (aug,linked) in [(false,false),(true,false),(false,true)] {
            let error=exercise(aug,linked); if error!=0 {return error;}
        } 0
    }
}
/// Run the independent workload before publishing success.
#[no_mangle]
#[link_section=".init.text"]
pub extern "C" fn init_module()->ffi::c_int {
    // SAFETY: init owns the static fixture state and retains no resources.
    let error=unsafe {rbtree_rust_exercise()};
    if error!=0 {return -22;}
    // SAFETY: Static NUL-terminated printk format with no arguments.
    unsafe {bindings::_printk(c"\x016@MARKER@\n".as_ptr().cast());} 0
}
/// All fixture storage is private and no references escape initialization.
#[no_mangle]
#[link_section=".exit.text"]
pub extern "C" fn cleanup_module() {}
#[used]
#[link_section=".init.data"]
static ADDRESSABLE_INIT_MODULE:extern "C" fn()->ffi::c_int=init_module;
#[used]
#[link_section=".exit.data"]
static ADDRESSABLE_CLEANUP_MODULE:extern "C" fn()=cleanup_module;
const INFO:&str="license=GPL\0description=Independent selected rbtree differential and nullable ABI\0";
#[used]
#[link_section=".modinfo"]
static MODINFO:[u8;INFO.len()]={let mut bytes=[0;INFO.len()];let mut i=0;
    while i<bytes.len(){bytes[i]=INFO.as_bytes()[i];i+=1;}bytes};
#[used]
static __IS_RUST_MODULE:()=();
'''


def caller_source(root,caller):
    marker(caller)
    if caller=='c': return c_source(root)
    return ('//! Independent Rust rbtree state machine over genuine bindings.\n'
            'use kernel::{bindings,ffi};\n'+header_bridge(root)+wrappers('rust')+'\n'+
            RUST_WORKLOAD.replace('@MARKER@',marker(caller).decode()))


def init_source(root):
    """Reuse audited PID1; require original rbtree test's exact EAGAIN result."""
    source=(root/'scripts/tests/boot_init.rs').read_text()
    needle='    let mut loaded_paths = Vec::new();'
    if source.count(needle)!=1: raise ValueError('boot PID1 module insertion point changed')
    hook='''    // Original rbtree_test executes checks and deliberately returns -EAGAIN.
    // A second attempt proves that the rejected init left no loaded instance.
    for attempt in 0..2 {
        let result = load_module(&fs::read("/original-rbtree-test.ko").unwrap());
        assert_eq!(result.unwrap_err().raw_os_error(), Some(11), "expected original rbtree EAGAIN");
        write_record(&mut log, &format!("LUPOS_RBTREE_ORIGINAL_EAGAIN_OK {attempt}")).unwrap();
    }
'''
    return source.replace(needle,hook+needle)
