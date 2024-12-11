#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
use ::binomial_heap::*;
use binomial_heap::src::binomial_heap::*;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn alloc_test_set_limit(alloc_count: libc::c_int);
    fn run_tests(tests_0: *mut UnitTestFunction);
    fn binomial_heap_new(
        heap_type: BinomialHeapType,
        compare_func: BinomialHeapCompareFunc,
    ) -> *mut BinomialHeap;
    fn binomial_heap_free(heap: *mut BinomialHeap);
    fn binomial_heap_insert(
        heap: *mut BinomialHeap,
        value: BinomialHeapValue,
    ) -> libc::c_int;
    fn binomial_heap_pop(heap: *mut BinomialHeap) -> BinomialHeapValue;
    fn binomial_heap_num_entries(heap: *mut BinomialHeap) -> libc::c_uint;
    fn int_compare(
        location1: *mut libc::c_void,
        location2: *mut libc::c_void,
    ) -> libc::c_int;
}
pub type UnitTestFunction = Option::<unsafe extern "C" fn() -> ()>;
pub type BinomialHeapType = libc::c_uint;
pub const BINOMIAL_HEAP_TYPE_MAX: BinomialHeapType = 1;
pub const BINOMIAL_HEAP_TYPE_MIN: BinomialHeapType = 0;
pub type BinomialHeapValue = *mut libc::c_void;
pub type BinomialHeapCompareFunc = Option::<
    unsafe extern "C" fn(BinomialHeapValue, BinomialHeapValue) -> libc::c_int,
>;
pub type BinomialHeap = _BinomialHeap;
#[no_mangle]
pub static mut test_array: [libc::c_int; 10000] = [0; 10000];
#[no_mangle]
pub unsafe extern "C" fn test_binomial_heap_new_free() {
    let mut heap: *mut BinomialHeap = 0 as *mut BinomialHeap;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 10000 as libc::c_int {
        heap = binomial_heap_new(
            BINOMIAL_HEAP_TYPE_MIN,
            Some(
                int_compare
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
        );
        binomial_heap_free(heap);
        i += 1;
        i;
    }
    alloc_test_set_limit(0 as libc::c_int);
    if (binomial_heap_new(
        BINOMIAL_HEAP_TYPE_MIN,
        Some(
            int_compare
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    ))
        .is_null()
    {} else {
        __assert_fail(
            b"binomial_heap_new(BINOMIAL_HEAP_TYPE_MIN, int_compare) == NULL\0"
                as *const u8 as *const libc::c_char,
            b"test-binomial-heap.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void test_binomial_heap_new_free(void)\0"))
                .as_ptr(),
        );
    }
    'c_1137: {
        if (binomial_heap_new(
            BINOMIAL_HEAP_TYPE_MIN,
            Some(
                int_compare
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
        ))
            .is_null()
        {} else {
            __assert_fail(
                b"binomial_heap_new(BINOMIAL_HEAP_TYPE_MIN, int_compare) == NULL\0"
                    as *const u8 as *const libc::c_char,
                b"test-binomial-heap.c\0" as *const u8 as *const libc::c_char,
                48 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"void test_binomial_heap_new_free(void)\0"))
                    .as_ptr(),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn test_binomial_heap_insert() {
    let mut heap: *mut BinomialHeap = 0 as *mut BinomialHeap;
    let mut i: libc::c_int = 0;
    heap = binomial_heap_new(
        BINOMIAL_HEAP_TYPE_MIN,
        Some(
            int_compare
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    i = 0 as libc::c_int;
    while i < 10000 as libc::c_int {
        test_array[i as usize] = i;
        if binomial_heap_insert(
            heap,
            &mut *test_array.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                as BinomialHeapValue,
        ) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binomial_heap_insert(heap, &test_array[i]) != 0\0" as *const u8
                    as *const libc::c_char,
                b"test-binomial-heap.c\0" as *const u8 as *const libc::c_char,
                60 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 37],
                    &[libc::c_char; 37],
                >(b"void test_binomial_heap_insert(void)\0"))
                    .as_ptr(),
            );
        }
        'c_1356: {
            if binomial_heap_insert(
                heap,
                &mut *test_array.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                    as BinomialHeapValue,
            ) != 0 as libc::c_int
            {} else {
                __assert_fail(
                    b"binomial_heap_insert(heap, &test_array[i]) != 0\0" as *const u8
                        as *const libc::c_char,
                    b"test-binomial-heap.c\0" as *const u8 as *const libc::c_char,
                    60 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 37],
                        &[libc::c_char; 37],
                    >(b"void test_binomial_heap_insert(void)\0"))
                        .as_ptr(),
                );
            }
        };
        i += 1;
        i;
    }
    if binomial_heap_num_entries(heap) == 10000 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"binomial_heap_num_entries(heap) == NUM_TEST_VALUES\0" as *const u8
                as *const libc::c_char,
            b"test-binomial-heap.c\0" as *const u8 as *const libc::c_char,
            62 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"void test_binomial_heap_insert(void)\0"))
                .as_ptr(),
        );
    }
    'c_1304: {
        if binomial_heap_num_entries(heap) == 10000 as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"binomial_heap_num_entries(heap) == NUM_TEST_VALUES\0" as *const u8
                    as *const libc::c_char,
                b"test-binomial-heap.c\0" as *const u8 as *const libc::c_char,
                62 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 37],
                    &[libc::c_char; 37],
                >(b"void test_binomial_heap_insert(void)\0"))
                    .as_ptr(),
            );
        }
    };
    alloc_test_set_limit(0 as libc::c_int);
    if binomial_heap_insert(heap, &mut i as *mut libc::c_int as BinomialHeapValue)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"binomial_heap_insert(heap, &i) == 0\0" as *const u8 as *const libc::c_char,
            b"test-binomial-heap.c\0" as *const u8 as *const libc::c_char,
            67 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"void test_binomial_heap_insert(void)\0"))
                .as_ptr(),
        );
    }
    'c_1248: {
        if binomial_heap_insert(heap, &mut i as *mut libc::c_int as BinomialHeapValue)
            == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binomial_heap_insert(heap, &i) == 0\0" as *const u8
                    as *const libc::c_char,
                b"test-binomial-heap.c\0" as *const u8 as *const libc::c_char,
                67 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 37],
                    &[libc::c_char; 37],
                >(b"void test_binomial_heap_insert(void)\0"))
                    .as_ptr(),
            );
        }
    };
    binomial_heap_free(heap);
}
#[no_mangle]
pub unsafe extern "C" fn test_min_heap() {
    let mut heap: *mut BinomialHeap = 0 as *mut BinomialHeap;
    let mut val: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    heap = binomial_heap_new(
        BINOMIAL_HEAP_TYPE_MIN,
        Some(
            int_compare
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    i = 0 as libc::c_int;
    while i < 10000 as libc::c_int {
        test_array[i as usize] = i;
        if binomial_heap_insert(
            heap,
            &mut *test_array.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                as BinomialHeapValue,
        ) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binomial_heap_insert(heap, &test_array[i]) != 0\0" as *const u8
                    as *const libc::c_char,
                b"test-binomial-heap.c\0" as *const u8 as *const libc::c_char,
                84 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 25],
                    &[libc::c_char; 25],
                >(b"void test_min_heap(void)\0"))
                    .as_ptr(),
            );
        }
        'c_1593: {
            if binomial_heap_insert(
                heap,
                &mut *test_array.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                    as BinomialHeapValue,
            ) != 0 as libc::c_int
            {} else {
                __assert_fail(
                    b"binomial_heap_insert(heap, &test_array[i]) != 0\0" as *const u8
                        as *const libc::c_char,
                    b"test-binomial-heap.c\0" as *const u8 as *const libc::c_char,
                    84 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 25],
                        &[libc::c_char; 25],
                    >(b"void test_min_heap(void)\0"))
                        .as_ptr(),
                );
            }
        };
        i += 1;
        i;
    }
    i = -(1 as libc::c_int);
    while binomial_heap_num_entries(heap) > 0 as libc::c_int as libc::c_uint {
        val = binomial_heap_pop(heap) as *mut libc::c_int;
        if *val == i + 1 as libc::c_int {} else {
            __assert_fail(
                b"*val == i + 1\0" as *const u8 as *const libc::c_char,
                b"test-binomial-heap.c\0" as *const u8 as *const libc::c_char,
                93 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 25],
                    &[libc::c_char; 25],
                >(b"void test_min_heap(void)\0"))
                    .as_ptr(),
            );
        }
        'c_1521: {
            if *val == i + 1 as libc::c_int {} else {
                __assert_fail(
                    b"*val == i + 1\0" as *const u8 as *const libc::c_char,
                    b"test-binomial-heap.c\0" as *const u8 as *const libc::c_char,
                    93 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 25],
                        &[libc::c_char; 25],
                    >(b"void test_min_heap(void)\0"))
                        .as_ptr(),
                );
            }
        };
        i = *val;
    }
    val = binomial_heap_pop(heap) as *mut libc::c_int;
    if val.is_null() {} else {
        __assert_fail(
            b"val == NULL\0" as *const u8 as *const libc::c_char,
            b"test-binomial-heap.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"void test_min_heap(void)\0"))
                .as_ptr(),
        );
    }
    'c_1455: {
        if val.is_null() {} else {
            __assert_fail(
                b"val == NULL\0" as *const u8 as *const libc::c_char,
                b"test-binomial-heap.c\0" as *const u8 as *const libc::c_char,
                100 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 25],
                    &[libc::c_char; 25],
                >(b"void test_min_heap(void)\0"))
                    .as_ptr(),
            );
        }
    };
    binomial_heap_free(heap);
}
#[no_mangle]
pub unsafe extern "C" fn test_max_heap() {
    let mut heap: *mut BinomialHeap = 0 as *mut BinomialHeap;
    let mut val: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    heap = binomial_heap_new(
        BINOMIAL_HEAP_TYPE_MAX,
        Some(
            int_compare
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    i = 0 as libc::c_int;
    while i < 10000 as libc::c_int {
        test_array[i as usize] = i;
        if binomial_heap_insert(
            heap,
            &mut *test_array.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                as BinomialHeapValue,
        ) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binomial_heap_insert(heap, &test_array[i]) != 0\0" as *const u8
                    as *const libc::c_char,
                b"test-binomial-heap.c\0" as *const u8 as *const libc::c_char,
                117 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 25],
                    &[libc::c_char; 25],
                >(b"void test_max_heap(void)\0"))
                    .as_ptr(),
            );
        }
        'c_1824: {
            if binomial_heap_insert(
                heap,
                &mut *test_array.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                    as BinomialHeapValue,
            ) != 0 as libc::c_int
            {} else {
                __assert_fail(
                    b"binomial_heap_insert(heap, &test_array[i]) != 0\0" as *const u8
                        as *const libc::c_char,
                    b"test-binomial-heap.c\0" as *const u8 as *const libc::c_char,
                    117 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 25],
                        &[libc::c_char; 25],
                    >(b"void test_max_heap(void)\0"))
                        .as_ptr(),
                );
            }
        };
        i += 1;
        i;
    }
    i = 10000 as libc::c_int;
    while binomial_heap_num_entries(heap) > 0 as libc::c_int as libc::c_uint {
        val = binomial_heap_pop(heap) as *mut libc::c_int;
        if *val == i - 1 as libc::c_int {} else {
            __assert_fail(
                b"*val == i - 1\0" as *const u8 as *const libc::c_char,
                b"test-binomial-heap.c\0" as *const u8 as *const libc::c_char,
                126 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 25],
                    &[libc::c_char; 25],
                >(b"void test_max_heap(void)\0"))
                    .as_ptr(),
            );
        }
        'c_1754: {
            if *val == i - 1 as libc::c_int {} else {
                __assert_fail(
                    b"*val == i - 1\0" as *const u8 as *const libc::c_char,
                    b"test-binomial-heap.c\0" as *const u8 as *const libc::c_char,
                    126 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 25],
                        &[libc::c_char; 25],
                    >(b"void test_max_heap(void)\0"))
                        .as_ptr(),
                );
            }
        };
        i = *val;
    }
    val = binomial_heap_pop(heap) as *mut libc::c_int;
    if val.is_null() {} else {
        __assert_fail(
            b"val == NULL\0" as *const u8 as *const libc::c_char,
            b"test-binomial-heap.c\0" as *const u8 as *const libc::c_char,
            133 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"void test_max_heap(void)\0"))
                .as_ptr(),
        );
    }
    'c_1691: {
        if val.is_null() {} else {
            __assert_fail(
                b"val == NULL\0" as *const u8 as *const libc::c_char,
                b"test-binomial-heap.c\0" as *const u8 as *const libc::c_char,
                133 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 25],
                    &[libc::c_char; 25],
                >(b"void test_max_heap(void)\0"))
                    .as_ptr(),
            );
        }
    };
    binomial_heap_free(heap);
}
unsafe extern "C" fn generate_heap() -> *mut BinomialHeap {
    let mut heap: *mut BinomialHeap = 0 as *mut BinomialHeap;
    let mut i: libc::c_int = 0;
    heap = binomial_heap_new(
        BINOMIAL_HEAP_TYPE_MIN,
        Some(
            int_compare
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    i = 0 as libc::c_int;
    while i < 10000 as libc::c_int {
        test_array[i as usize] = i;
        if i != 10000 as libc::c_int / 2 as libc::c_int {
            if binomial_heap_insert(
                heap,
                &mut *test_array.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                    as BinomialHeapValue,
            ) != 0 as libc::c_int
            {} else {
                __assert_fail(
                    b"binomial_heap_insert(heap, &test_array[i]) != 0\0" as *const u8
                        as *const libc::c_char,
                    b"test-binomial-heap.c\0" as *const u8 as *const libc::c_char,
                    153 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 34],
                        &[libc::c_char; 34],
                    >(b"BinomialHeap *generate_heap(void)\0"))
                        .as_ptr(),
                );
            }
            'c_1924: {
                if binomial_heap_insert(
                    heap,
                    &mut *test_array.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                        as BinomialHeapValue,
                ) != 0 as libc::c_int
                {} else {
                    __assert_fail(
                        b"binomial_heap_insert(heap, &test_array[i]) != 0\0" as *const u8
                            as *const libc::c_char,
                        b"test-binomial-heap.c\0" as *const u8 as *const libc::c_char,
                        153 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 34],
                            &[libc::c_char; 34],
                        >(b"BinomialHeap *generate_heap(void)\0"))
                            .as_ptr(),
                    );
                }
            };
        }
        i += 1;
        i;
    }
    return heap;
}
unsafe extern "C" fn verify_heap(mut heap: *mut BinomialHeap) {
    let mut num_vals: libc::c_uint = 0;
    let mut val: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    num_vals = binomial_heap_num_entries(heap);
    if num_vals == (10000 as libc::c_int - 1 as libc::c_int) as libc::c_uint {} else {
        __assert_fail(
            b"num_vals == NUM_TEST_VALUES - 1\0" as *const u8 as *const libc::c_char,
            b"test-binomial-heap.c\0" as *const u8 as *const libc::c_char,
            170 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void verify_heap(BinomialHeap *)\0"))
                .as_ptr(),
        );
    }
    'c_2149: {
        if num_vals == (10000 as libc::c_int - 1 as libc::c_int) as libc::c_uint
        {} else {
            __assert_fail(
                b"num_vals == NUM_TEST_VALUES - 1\0" as *const u8 as *const libc::c_char,
                b"test-binomial-heap.c\0" as *const u8 as *const libc::c_char,
                170 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void verify_heap(BinomialHeap *)\0"))
                    .as_ptr(),
            );
        }
    };
    i = 0 as libc::c_int;
    while i < 10000 as libc::c_int {
        if !(i == 10000 as libc::c_int / 2 as libc::c_int) {
            val = binomial_heap_pop(heap) as *mut libc::c_int;
            if *val == i {} else {
                __assert_fail(
                    b"*val == i\0" as *const u8 as *const libc::c_char,
                    b"test-binomial-heap.c\0" as *const u8 as *const libc::c_char,
                    180 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 33],
                        &[libc::c_char; 33],
                    >(b"void verify_heap(BinomialHeap *)\0"))
                        .as_ptr(),
                );
            }
            'c_2080: {
                if *val == i {} else {
                    __assert_fail(
                        b"*val == i\0" as *const u8 as *const libc::c_char,
                        b"test-binomial-heap.c\0" as *const u8 as *const libc::c_char,
                        180 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 33],
                            &[libc::c_char; 33],
                        >(b"void verify_heap(BinomialHeap *)\0"))
                            .as_ptr(),
                    );
                }
            };
            num_vals = num_vals.wrapping_sub(1);
            num_vals;
            if binomial_heap_num_entries(heap) == num_vals {} else {
                __assert_fail(
                    b"binomial_heap_num_entries(heap) == num_vals\0" as *const u8
                        as *const libc::c_char,
                    b"test-binomial-heap.c\0" as *const u8 as *const libc::c_char,
                    185 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 33],
                        &[libc::c_char; 33],
                    >(b"void verify_heap(BinomialHeap *)\0"))
                        .as_ptr(),
                );
            }
            'c_2031: {
                if binomial_heap_num_entries(heap) == num_vals {} else {
                    __assert_fail(
                        b"binomial_heap_num_entries(heap) == num_vals\0" as *const u8
                            as *const libc::c_char,
                        b"test-binomial-heap.c\0" as *const u8 as *const libc::c_char,
                        185 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 33],
                            &[libc::c_char; 33],
                        >(b"void verify_heap(BinomialHeap *)\0"))
                            .as_ptr(),
                    );
                }
            };
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn test_insert_out_of_memory() {
    let mut heap: *mut BinomialHeap = 0 as *mut BinomialHeap;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        heap = generate_heap();
        alloc_test_set_limit(i);
        test_array[(10000 as libc::c_int / 2 as libc::c_int)
            as usize] = 10000 as libc::c_int / 2 as libc::c_int;
        if binomial_heap_insert(
            heap,
            &mut *test_array
                .as_mut_ptr()
                .offset((10000 as libc::c_int / 2 as libc::c_int) as isize)
                as *mut libc::c_int as BinomialHeapValue,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binomial_heap_insert(heap, &test_array[TEST_VALUE]) == 0\0"
                    as *const u8 as *const libc::c_char,
                b"test-binomial-heap.c\0" as *const u8 as *const libc::c_char,
                207 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 37],
                    &[libc::c_char; 37],
                >(b"void test_insert_out_of_memory(void)\0"))
                    .as_ptr(),
            );
        }
        'c_2231: {
            if binomial_heap_insert(
                heap,
                &mut *test_array
                    .as_mut_ptr()
                    .offset((10000 as libc::c_int / 2 as libc::c_int) as isize)
                    as *mut libc::c_int as BinomialHeapValue,
            ) == 0 as libc::c_int
            {} else {
                __assert_fail(
                    b"binomial_heap_insert(heap, &test_array[TEST_VALUE]) == 0\0"
                        as *const u8 as *const libc::c_char,
                    b"test-binomial-heap.c\0" as *const u8 as *const libc::c_char,
                    207 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 37],
                        &[libc::c_char; 37],
                    >(b"void test_insert_out_of_memory(void)\0"))
                        .as_ptr(),
                );
            }
        };
        alloc_test_set_limit(-(1 as libc::c_int));
        verify_heap(heap);
        binomial_heap_free(heap);
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn test_pop_out_of_memory() {
    let mut heap: *mut BinomialHeap = 0 as *mut BinomialHeap;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        heap = generate_heap();
        alloc_test_set_limit(i);
        if (binomial_heap_pop(heap)).is_null() {} else {
            __assert_fail(
                b"binomial_heap_pop(heap) == NULL\0" as *const u8 as *const libc::c_char,
                b"test-binomial-heap.c\0" as *const u8 as *const libc::c_char,
                235 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_pop_out_of_memory(void)\0"))
                    .as_ptr(),
            );
        }
        'c_2348: {
            if (binomial_heap_pop(heap)).is_null() {} else {
                __assert_fail(
                    b"binomial_heap_pop(heap) == NULL\0" as *const u8
                        as *const libc::c_char,
                    b"test-binomial-heap.c\0" as *const u8 as *const libc::c_char,
                    235 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 34],
                        &[libc::c_char; 34],
                    >(b"void test_pop_out_of_memory(void)\0"))
                        .as_ptr(),
                );
            }
        };
        alloc_test_set_limit(-(1 as libc::c_int));
        binomial_heap_free(heap);
        i += 1;
        i;
    }
}
static mut tests: [UnitTestFunction; 7] = unsafe {
    [
        Some(test_binomial_heap_new_free as unsafe extern "C" fn() -> ()),
        Some(test_binomial_heap_insert as unsafe extern "C" fn() -> ()),
        Some(test_min_heap as unsafe extern "C" fn() -> ()),
        Some(test_max_heap as unsafe extern "C" fn() -> ()),
        Some(test_insert_out_of_memory as unsafe extern "C" fn() -> ()),
        Some(test_pop_out_of_memory as unsafe extern "C" fn() -> ()),
        None,
    ]
};
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    run_tests(tests.as_mut_ptr());
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
