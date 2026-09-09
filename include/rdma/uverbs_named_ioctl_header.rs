/* SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB */
/*
 * Copyright (c) 2018, Mellanox Technologies inc.  All rights reserved.
 */

// C header dependency: <rdma/uverbs_ioctl.h>
// UVERBS_MODULE_NAME must be defined by the including translation unit.

// The following macros preserve the source header's declaration interface.
// Identifier concatenation is retained as macro intent for the eventual
// dependency/integration layer.
macro_rules! _UVERBS_PASTE { ($x:ident, $y:ident) => { concat_idents!($x, $y) }; }
macro_rules! _UVERBS_NAME { ($x:ident, $y:ident) => { _UVERBS_PASTE!($x, $y) }; }
macro_rules! UVERBS_METHOD { ($id:ident) => { _UVERBS_NAME!(UVERBS_MODULE_NAME, _method_$id) }; }
macro_rules! UVERBS_HANDLER { ($id:ident) => { _UVERBS_NAME!(UVERBS_MODULE_NAME, _handler_$id) }; }
macro_rules! UVERBS_OBJECT { ($id:ident) => { _UVERBS_NAME!(UVERBS_MODULE_NAME, _object_$id) }; }

macro_rules! UVERBS_METHOD_ATTRS { ($method_id:ident) => { _method_attrs_$method_id }; }
macro_rules! UVERBS_OBJECT_METHODS { ($object_id:ident) => { _UVERBS_NAME!(_object_methods_$object_id, __LINE__) }; }

macro_rules! DECLARE_UVERBS_NAMED_METHOD {
    ($method_id:expr, $( $attrs:expr ),* $(,)?) => {
        static UVERBS_METHOD_ATTRS!($method_id): &'static [&'static uverbs_attr_def] = &[ $( $attrs ),* ];
        static UVERBS_METHOD!($method_id): uverbs_method_def = uverbs_method_def {
            id: $method_id,
            handler: UVERBS_HANDLER!($method_id),
            num_attrs: UVERBS_METHOD_ATTRS!($method_id).len(),
            attrs: &UVERBS_METHOD_ATTRS!($method_id),
        };
    };
}

/* Create a standard destroy method using the default handler. The handle_attr
 * argument must be the attribute specifying the handle to destroy, the
 * default handler does not support any other attributes.
 */
macro_rules! DECLARE_UVERBS_NAMED_METHOD_DESTROY {
    ($method_id:expr, $handle_attr:expr) => {
        static UVERBS_METHOD_ATTRS!($method_id): &'static [&'static uverbs_attr_def] = &[ $handle_attr ];
        static UVERBS_METHOD!($method_id): uverbs_method_def = uverbs_method_def {
            id: $method_id,
            handler: uverbs_destroy_def_handler,
            num_attrs: UVERBS_METHOD_ATTRS!($method_id).len(),
            attrs: &UVERBS_METHOD_ATTRS!($method_id),
        };
    };
}

macro_rules! DECLARE_UVERBS_NAMED_OBJECT {
    ($object_id:expr, $type_attrs:expr, $( $methods:expr ),* $(,)?) => {
        static UVERBS_OBJECT_METHODS!($object_id): &'static [&'static uverbs_method_def] = &[ $( $methods ),* ];
        static UVERBS_OBJECT!($object_id): uverbs_object_def = uverbs_object_def {
            id: $object_id,
            type_attrs: &$type_attrs,
            num_methods: UVERBS_OBJECT_METHODS!($object_id).len(),
            methods: &UVERBS_OBJECT_METHODS!($object_id),
        };
    };
}

/* Declare global methods. These still have a unique object_id because we
 * identify all uapi methods with a (object,method) tuple. However, they have
 * no type pointer.
 */
macro_rules! DECLARE_UVERBS_GLOBAL_METHODS {
    ($object_id:expr, $( $methods:expr ),* $(,)?) => {
        static UVERBS_OBJECT_METHODS!($object_id): &'static [&'static uverbs_method_def] = &[ $( $methods ),* ];
        static UVERBS_OBJECT!($object_id): uverbs_object_def = uverbs_object_def {
            id: $object_id,
            num_methods: UVERBS_OBJECT_METHODS!($object_id).len(),
            methods: &UVERBS_OBJECT_METHODS!($object_id),
        };
    };
}

/* Used by drivers to declare a complete parsing tree for new methods. */
macro_rules! ADD_UVERBS_METHODS {
    ($name:ident, $object_id:expr, $( $methods:expr ),* $(,)?) => {
        static UVERBS_OBJECT_METHODS!($object_id): &'static [&'static uverbs_method_def] = &[ $( $methods ),* ];
        static $name: uverbs_object_def = uverbs_object_def {
            id: $object_id,
            num_methods: UVERBS_OBJECT_METHODS!($object_id).len(),
            methods: &UVERBS_OBJECT_METHODS!($object_id),
        };
    };
}

/* Used by drivers to declare a complete parsing tree for a single method that
 * differs only in having additional driver specific attributes.
 */
macro_rules! ADD_UVERBS_ATTRIBUTES_SIMPLE {
    ($name:ident, $object_id:expr, $method_id:expr, $( $attrs:expr ),* $(,)?) => {
        static UVERBS_METHOD_ATTRS!($method_id): &'static [&'static uverbs_attr_def] = &[ $( $attrs ),* ];
        static UVERBS_METHOD!($method_id): uverbs_method_def = uverbs_method_def {
            id: $method_id,
            num_attrs: UVERBS_METHOD_ATTRS!($method_id).len(),
            attrs: &UVERBS_METHOD_ATTRS!($method_id),
        };
        ADD_UVERBS_METHODS!($name, $object_id, &UVERBS_METHOD!($method_id));
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
