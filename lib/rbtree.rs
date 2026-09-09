// SPDX-License-Identifier: GPL-2.0-or-later
/* Red Black Trees; direct translation of rbtree.c. */

use core::ffi::c_void;

#[repr(C)]
pub struct rb_node { pub __rb_parent_color: usize, pub rb_right: *mut rb_node, pub rb_left: *mut rb_node }
#[repr(C)]
pub struct rb_root { pub rb_node: *mut rb_node }
#[repr(C)]
pub struct rb_node_linked { pub node: rb_node, pub next: *mut rb_node_linked, pub prev: *mut rb_node_linked }
#[repr(C)]
pub struct rb_root_linked { pub rb_root: rb_root, pub rb_leftmost: *mut rb_node_linked }
#[repr(C)]
pub struct rb_augment_callbacks { pub propagate: Option<unsafe extern "C" fn(*mut rb_node,*mut rb_node)>, pub copy: Option<unsafe extern "C" fn(*mut rb_node,*mut rb_node)>, pub rotate: Option<unsafe extern "C" fn(*mut rb_node,*mut rb_node)> }

pub const RB_RED: usize = 0;
pub const RB_BLACK: usize = 1;

extern "C" {
    fn rb_parent(node: *const rb_node) -> *mut rb_node;
    fn rb_is_black(node: *const rb_node) -> bool;
    fn rb_is_red(node: *const rb_node) -> bool;
    fn rb_set_parent_color(node: *mut rb_node, parent: *mut rb_node, color: usize);
    fn rb_set_parent(node: *mut rb_node, parent: *mut rb_node);
    fn __rb_change_child(old: *mut rb_node, new: *mut rb_node, parent: *mut rb_node, root: *mut rb_root);
    fn __rb_change_child_rcu(old: *mut rb_node, new: *mut rb_node, parent: *mut rb_node, root: *mut rb_root);
    fn __rb_erase_augmented(node: *mut rb_node, root: *mut rb_root, c: *const rb_augment_callbacks) -> *mut rb_node;
    fn RB_EMPTY_NODE(node: *const rb_node) -> bool;
    fn RB_CLEAR_LINKED_NODE(node: *mut rb_node_linked);
}

#[inline] unsafe fn rb_set_black(rb: *mut rb_node) { (*rb).__rb_parent_color = (*rb).__rb_parent_color.wrapping_add(RB_BLACK); }
#[inline] unsafe fn rb_red_parent(red: *mut rb_node) -> *mut rb_node { (*red).__rb_parent_color as *mut rb_node }

#[inline] unsafe fn __rb_rotate_set_parents(old: *mut rb_node, new: *mut rb_node, root: *mut rb_root, color: usize) {
    let parent = rb_parent(old); (*new).__rb_parent_color = (*old).__rb_parent_color;
    rb_set_parent_color(old, new, color); __rb_change_child(old, new, parent, root);
}

unsafe fn __rb_insert(mut node: *mut rb_node, root: *mut rb_root, augment_rotate: Option<unsafe extern "C" fn(*mut rb_node,*mut rb_node)>) {
    let mut parent = rb_red_parent(node);
    loop {
        if parent.is_null() { rb_set_parent_color(node, core::ptr::null_mut(), RB_BLACK); break; }
        if rb_is_black(parent) { break; }
        let gparent = rb_red_parent(parent); let mut tmp = (*gparent).rb_right;
        if parent != tmp {
            if !tmp.is_null() && rb_is_red(tmp) {
                rb_set_parent_color(tmp,gparent,RB_BLACK); rb_set_parent_color(parent,gparent,RB_BLACK);
                node=gparent; parent=rb_parent(node); rb_set_parent_color(node,parent,RB_RED); continue;
            }
            tmp=(*parent).rb_right;
            if node==tmp { tmp=(*node).rb_left; (*parent).rb_right=tmp; (*node).rb_left=parent; if !tmp.is_null(){rb_set_parent_color(tmp,parent,RB_BLACK)} rb_set_parent_color(parent,node,RB_RED); if let Some(f)=augment_rotate{f(parent,node)} parent=node; tmp=(*node).rb_right; }
            (*gparent).rb_left=tmp; (*parent).rb_right=gparent; if !tmp.is_null(){rb_set_parent_color(tmp,gparent,RB_BLACK)} __rb_rotate_set_parents(gparent,parent,root,RB_RED); if let Some(f)=augment_rotate{f(gparent,parent)} break;
        } else {
            tmp=(*gparent).rb_left;
            if !tmp.is_null() && rb_is_red(tmp) { rb_set_parent_color(tmp,gparent,RB_BLACK); rb_set_parent_color(parent,gparent,RB_BLACK); node=gparent; parent=rb_parent(node); rb_set_parent_color(node,parent,RB_RED); continue; }
            tmp=(*parent).rb_left;
            if node==tmp { tmp=(*node).rb_right; (*parent).rb_left=tmp; (*node).rb_right=parent; if !tmp.is_null(){rb_set_parent_color(tmp,parent,RB_BLACK)} rb_set_parent_color(parent,node,RB_RED); if let Some(f)=augment_rotate{f(parent,node)} parent=node; tmp=(*node).rb_left; }
            (*gparent).rb_right=tmp; (*parent).rb_left=gparent; if !tmp.is_null(){rb_set_parent_color(tmp,gparent,RB_BLACK)} __rb_rotate_set_parents(gparent,parent,root,RB_RED); if let Some(f)=augment_rotate{f(gparent,parent)} break;
        }
    }
}

unsafe fn ____rb_erase_color(mut parent: *mut rb_node, root: *mut rb_root, augment_rotate: Option<unsafe extern "C" fn(*mut rb_node,*mut rb_node)>) {
    let mut node: *mut rb_node=core::ptr::null_mut();
    while !parent.is_null() { let mut sibling=(*parent).rb_right;
        if node!=sibling { if rb_is_red(sibling) { let tmp1=(*sibling).rb_left; (*parent).rb_right=tmp1; (*sibling).rb_left=parent; rb_set_parent_color(tmp1,parent,RB_BLACK); __rb_rotate_set_parents(parent,sibling,root,RB_RED); if let Some(f)=augment_rotate{f(parent,sibling)} sibling=tmp1; }
            let mut tmp1=(*sibling).rb_right; if tmp1.is_null()||rb_is_black(tmp1) { let mut tmp2=(*sibling).rb_left; if tmp2.is_null()||rb_is_black(tmp2) { rb_set_parent_color(sibling,parent,RB_RED); if rb_is_red(parent){rb_set_black(parent)}else{node=parent;parent=rb_parent(node);if !parent.is_null(){continue}} break; }
                tmp1=(*tmp2).rb_right; (*sibling).rb_left=tmp1; (*tmp2).rb_right=sibling; (*parent).rb_right=tmp2; if !tmp1.is_null(){rb_set_parent_color(tmp1,sibling,RB_BLACK)} if let Some(f)=augment_rotate{f(sibling,tmp2)} sibling=tmp2; }
            let tmp2=(*sibling).rb_left; (*parent).rb_right=tmp2; (*sibling).rb_left=parent; rb_set_parent_color(tmp1,sibling,RB_BLACK); if !tmp2.is_null(){rb_set_parent(tmp2,parent)} __rb_rotate_set_parents(parent,sibling,root,RB_BLACK); if let Some(f)=augment_rotate{f(parent,sibling)} break;
        } else { sibling=(*parent).rb_left; if rb_is_red(sibling){let tmp1=(*sibling).rb_right;(*parent).rb_left=tmp1;(*sibling).rb_right=parent;rb_set_parent_color(tmp1,parent,RB_BLACK);__rb_rotate_set_parents(parent,sibling,root,RB_RED);if let Some(f)=augment_rotate{f(parent,sibling)}sibling=tmp1}
            let mut tmp1=(*sibling).rb_left;if tmp1.is_null()||rb_is_black(tmp1){let mut tmp2=(*sibling).rb_right;if tmp2.is_null()||rb_is_black(tmp2){rb_set_parent_color(sibling,parent,RB_RED);if rb_is_red(parent){rb_set_black(parent)}else{node=parent;parent=rb_parent(node);if !parent.is_null(){continue}}break}tmp1=(*tmp2).rb_left;(*sibling).rb_right=tmp1;(*tmp2).rb_left=sibling;(*parent).rb_left=tmp2;if !tmp1.is_null(){rb_set_parent_color(tmp1,sibling,RB_BLACK)}if let Some(f)=augment_rotate{f(sibling,tmp2)}sibling=tmp2}let tmp2=(*sibling).rb_right;(*parent).rb_left=tmp2;(*sibling).rb_right=parent;rb_set_parent_color(tmp1,sibling,RB_BLACK);if !tmp2.is_null(){rb_set_parent(tmp2,parent)}__rb_rotate_set_parents(parent,sibling,root,RB_BLACK);if let Some(f)=augment_rotate{f(parent,sibling)}break;
        }
    }
}

#[no_mangle] pub unsafe extern "C" fn __rb_erase_color(parent:*mut rb_node,root:*mut rb_root,augment_rotate:Option<unsafe extern "C" fn(*mut rb_node,*mut rb_node)>){____rb_erase_color(parent,root,augment_rotate)}
unsafe extern "C" fn dummy_propagate(_: *mut rb_node,_:*mut rb_node){}
unsafe extern "C" fn dummy_copy(_: *mut rb_node,_:*mut rb_node){}
unsafe extern "C" fn dummy_rotate(_: *mut rb_node,_:*mut rb_node){}
static DUMMY_CALLBACKS: rb_augment_callbacks=rb_augment_callbacks{propagate:Some(dummy_propagate),copy:Some(dummy_copy),rotate:Some(dummy_rotate)};

#[no_mangle] pub unsafe extern "C" fn rb_insert_color(node:*mut rb_node,root:*mut rb_root){__rb_insert(node,root,Some(dummy_rotate))}
#[no_mangle] pub unsafe extern "C" fn rb_erase(node:*mut rb_node,root:*mut rb_root){let rebalance=__rb_erase_augmented(node,root,&DUMMY_CALLBACKS);if !rebalance.is_null(){____rb_erase_color(rebalance,root,Some(dummy_rotate))}}
#[no_mangle] pub unsafe extern "C" fn rb_erase_linked(node:*mut rb_node_linked,root:*mut rb_root_linked)->bool{if !(*node).prev{(*root).rb_leftmost=(*node).next}else{(*(*node).prev).next=(*node).next}if !(*node).next.is_null(){(*(*node).next).prev=(*node).prev}rb_erase(&mut (*node).node,&mut (*root).rb_root);RB_CLEAR_LINKED_NODE(node);!(*root).rb_leftmost.is_null()}
#[no_mangle] pub unsafe extern "C" fn __rb_insert_augmented(node:*mut rb_node,root:*mut rb_root,rotate:Option<unsafe extern "C" fn(*mut rb_node,*mut rb_node)>){__rb_insert(node,root,rotate)}

#[no_mangle] pub unsafe extern "C" fn rb_next(mut node:*const rb_node)->*mut rb_node{if RB_EMPTY_NODE(node){return core::ptr::null_mut()}if !(*node).rb_right.is_null(){let mut n=(*node).rb_right;while !(*n).rb_left.is_null(){n=(*n).rb_left}return n}let mut parent;while {parent=rb_parent(node);!parent.is_null()&&node==(*parent).rb_right}{node=parent}parent}
#[no_mangle] pub unsafe extern "C" fn rb_prev(mut node:*const rb_node)->*mut rb_node{if RB_EMPTY_NODE(node){return core::ptr::null_mut()}if !(*node).rb_left.is_null(){let mut n=(*node).rb_left;while !(*n).rb_right.is_null(){n=(*n).rb_right}return n}let mut parent;while{parent=rb_parent(node);!parent.is_null()&&node==(*parent).rb_left}{node=parent}parent}
#[no_mangle] pub unsafe extern "C" fn rb_replace_node(victim:*mut rb_node,new:*mut rb_node,root:*mut rb_root){let parent=rb_parent(victim);*new=*victim;if !(*victim).rb_left.is_null(){rb_set_parent((*victim).rb_left,new)}if !(*victim).rb_right.is_null(){rb_set_parent((*victim).rb_right,new)}__rb_change_child(victim,new,parent,root)}
#[no_mangle] pub unsafe extern "C" fn rb_replace_node_rcu(victim:*mut rb_node,new:*mut rb_node,root:*mut rb_root){let parent=rb_parent(victim);*new=*victim;if !(*victim).rb_left.is_null(){rb_set_parent((*victim).rb_left,new)}if !(*victim).rb_right.is_null(){rb_set_parent((*victim).rb_right,new)}__rb_change_child_rcu(victim,new,parent,root)}
unsafe fn rb_left_deepest_node(mut node:*const rb_node)->*mut rb_node{loop{if !(*node).rb_left.is_null(){node=(*node).rb_left}else if !(*node).rb_right.is_null(){node=(*node).rb_right}else{return node as *mut rb_node}}}
#[no_mangle] pub unsafe extern "C" fn rb_next_postorder(node:*const rb_node)->*mut rb_node{if node.is_null(){return core::ptr::null_mut()}let parent=rb_parent(node);if !parent.is_null()&&node==(*parent).rb_left&&!(*parent).rb_right.is_null(){rb_left_deepest_node((*parent).rb_right)}else{parent}}
#[no_mangle] pub unsafe extern "C" fn rb_first_postorder(root:*const rb_root)->*mut rb_node{if (*root).rb_node.is_null(){core::ptr::null_mut()}else{rb_left_deepest_node((*root).rb_node)}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
