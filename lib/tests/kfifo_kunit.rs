// SPDX-License-Identifier: GPL-2.0
/*
 * KUnit test for the generic kernel FIFO implementation.
 *
 * Copyright (C) 2024 Diego Vieira <diego.daniel.professional@gmail.com>
 */

// Dependency-provided kernel/KUnit symbols are intentionally referenced here.
const KFIFO_SIZE: usize = 32;
const N_ELEMENTS: usize = 5;

unsafe fn kfifo_test_reset_should_clear_the_fifo(test: *mut kunit) {
    let mut my_fifo = DEFINE_KFIFO::<u8, KFIFO_SIZE>();

    kfifo_put(&mut my_fifo, 1);
    kfifo_put(&mut my_fifo, 2);
    kfifo_put(&mut my_fifo, 3);
    KUNIT_EXPECT_EQ(test, kfifo_len(&my_fifo), 3);

    kfifo_reset(&mut my_fifo);

    KUNIT_EXPECT_EQ(test, kfifo_len(&my_fifo), 0);
    KUNIT_EXPECT_TRUE(test, kfifo_is_empty(&my_fifo));
}

unsafe fn kfifo_test_define_should_define_an_empty_fifo(test: *mut kunit) {
    let mut my_fifo = DEFINE_KFIFO::<u8, KFIFO_SIZE>();

    KUNIT_EXPECT_TRUE(test, kfifo_initialized(&my_fifo));
    KUNIT_EXPECT_TRUE(test, kfifo_is_empty(&my_fifo));
    KUNIT_EXPECT_EQ(test, kfifo_len(&my_fifo), 0);
}

unsafe fn kfifo_test_len_should_ret_n_of_stored_elements(test: *mut kunit) {
    let mut buffer1 = [0u8; N_ELEMENTS];

    for i in 0..N_ELEMENTS {
        buffer1[i] = (i + 1) as u8;
    }

    let mut my_fifo = DEFINE_KFIFO::<u8, KFIFO_SIZE>();

    KUNIT_EXPECT_EQ(test, kfifo_len(&my_fifo), 0);

    kfifo_in(&mut my_fifo, buffer1.as_ptr(), N_ELEMENTS);
    KUNIT_EXPECT_EQ(test, kfifo_len(&my_fifo), N_ELEMENTS);

    kfifo_in(&mut my_fifo, buffer1.as_ptr(), N_ELEMENTS);
    KUNIT_EXPECT_EQ(test, kfifo_len(&my_fifo), N_ELEMENTS * 2);

    kfifo_reset(&mut my_fifo);
    KUNIT_EXPECT_EQ(test, kfifo_len(&my_fifo), 0);
}

unsafe fn kfifo_test_put_should_insert_and_get_should_pop(test: *mut kunit) {
    let mut out_data = 0u8;
    let mut processed_elements: i32;
    let elements = [3u8, 5, 11];

    let mut my_fifo = DEFINE_KFIFO::<u8, KFIFO_SIZE>();

    // If the fifo is empty, get returns 0
    processed_elements = kfifo_get(&mut my_fifo, &mut out_data);
    KUNIT_EXPECT_EQ(test, processed_elements, 0);
    KUNIT_EXPECT_EQ(test, out_data, 0);

    for i in 0..3 {
        kfifo_put(&mut my_fifo, elements[i]);
    }

    for i in 0..3 {
        processed_elements = kfifo_get(&mut my_fifo, &mut out_data);
        KUNIT_EXPECT_EQ(test, processed_elements, 1);
        KUNIT_EXPECT_EQ(test, out_data, elements[i]);
    }
}

unsafe fn kfifo_test_in_should_insert_multiple_elements(test: *mut kunit) {
    let in_buffer = [11u8, 25, 65];
    let mut out_data = 0u8;
    let mut processed_elements: i32;

    let mut my_fifo = DEFINE_KFIFO::<u8, KFIFO_SIZE>();

    kfifo_in(&mut my_fifo, in_buffer.as_ptr(), 3);

    for i in 0..3 {
        processed_elements = kfifo_get(&mut my_fifo, &mut out_data);
        KUNIT_EXPECT_EQ(test, processed_elements, 1);
        KUNIT_EXPECT_EQ(test, out_data, in_buffer[i]);
    }
}

unsafe fn kfifo_test_out_should_pop_multiple_elements(test: *mut kunit) {
    let in_buffer = [11u8, 25, 65];
    let mut out_buffer = [0u8; 3];
    let copied_elements: i32;

    let mut my_fifo = DEFINE_KFIFO::<u8, KFIFO_SIZE>();

    for i in 0..3 {
        kfifo_put(&mut my_fifo, in_buffer[i]);
    }

    copied_elements = kfifo_out(&mut my_fifo, out_buffer.as_mut_ptr(), 3);
    KUNIT_EXPECT_EQ(test, copied_elements, 3);

    for i in 0..3 {
        KUNIT_EXPECT_EQ(test, out_buffer[i], in_buffer[i]);
    }
    KUNIT_EXPECT_TRUE(test, kfifo_is_empty(&my_fifo));
}

unsafe fn kfifo_test_dec_init_should_define_an_empty_fifo(test: *mut kunit) {
    let mut my_fifo = DECLARE_KFIFO::<u8, KFIFO_SIZE>();

    INIT_KFIFO(&mut my_fifo);

    // my_fifo is a struct with an inplace buffer
    KUNIT_EXPECT_FALSE(test, __is_kfifo_ptr(&my_fifo));
    KUNIT_EXPECT_TRUE(test, kfifo_initialized(&my_fifo));
}

unsafe fn kfifo_test_define_should_equal_declare_init(test: *mut kunit) {
    // declare a variable my_fifo of type struct kfifo of u8
    let mut my_fifo1 = DECLARE_KFIFO::<u8, KFIFO_SIZE>();
    // initialize the my_fifo variable
    INIT_KFIFO(&mut my_fifo1);

    // DEFINE_KFIFO declares the variable with the initial value
    // essentially the same as calling DECLARE_KFIFO and INIT_KFIFO
    let mut my_fifo2 = DEFINE_KFIFO::<u8, KFIFO_SIZE>();

    // my_fifo1 and my_fifo2 have the same size
    KUNIT_EXPECT_EQ(test, core::mem::size_of_val(&my_fifo1), core::mem::size_of_val(&my_fifo2));
    KUNIT_EXPECT_EQ(test, kfifo_initialized(&my_fifo1), kfifo_initialized(&my_fifo2));
    KUNIT_EXPECT_EQ(test, kfifo_is_empty(&my_fifo1), kfifo_is_empty(&my_fifo2));
}

unsafe fn kfifo_test_alloc_should_initiliaze_a_ptr_fifo(test: *mut kunit) {
    let ret: i32;
    let mut my_fifo = DECLARE_KFIFO_PTR::<u8>();

    INIT_KFIFO(&mut my_fifo);

    // kfifo_initialized returns false signaling the buffer pointer is NULL
    KUNIT_EXPECT_FALSE(test, kfifo_initialized(&my_fifo));

    // kfifo_alloc allocates the buffer
    ret = kfifo_alloc(&mut my_fifo, KFIFO_SIZE, GFP_KERNEL);
    KUNIT_EXPECT_EQ_MSG(test, ret, 0, "Memory allocation should succeed");
    KUNIT_EXPECT_TRUE(test, kfifo_initialized(&my_fifo));

    // kfifo_free frees the buffer
    kfifo_free(&mut my_fifo);
}

unsafe fn kfifo_test_peek_should_not_remove_elements(test: *mut kunit) {
    let mut out_data = 0u8;
    let mut processed_elements: i32;

    let mut my_fifo = DEFINE_KFIFO::<u8, KFIFO_SIZE>();

    // If the fifo is empty, peek returns 0
    processed_elements = kfifo_peek(&mut my_fifo, &mut out_data);
    KUNIT_EXPECT_EQ(test, processed_elements, 0);

    kfifo_put(&mut my_fifo, 3);
    kfifo_put(&mut my_fifo, 5);
    kfifo_put(&mut my_fifo, 11);

    KUNIT_EXPECT_EQ(test, kfifo_len(&my_fifo), 3);

    processed_elements = kfifo_peek(&mut my_fifo, &mut out_data);
    KUNIT_EXPECT_EQ(test, processed_elements, 1);
    KUNIT_EXPECT_EQ(test, out_data, 3);

    KUNIT_EXPECT_EQ(test, kfifo_len(&my_fifo), 3);

    // Using peek doesn't remove the element
    // so the read element and the fifo length
    // remains the same
    processed_elements = kfifo_peek(&mut my_fifo, &mut out_data);
    KUNIT_EXPECT_EQ(test, processed_elements, 1);
    KUNIT_EXPECT_EQ(test, out_data, 3);

    KUNIT_EXPECT_EQ(test, kfifo_len(&my_fifo), 3);
}

// KUNIT_CASE entries, suite registration, and module metadata are supplied by
// the dependency's registration macros and are preserved as external intent.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
