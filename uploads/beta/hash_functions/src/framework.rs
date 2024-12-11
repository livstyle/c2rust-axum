use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn alloc_test_set_limit(alloc_count: libc::c_int);
    fn alloc_test_get_allocated() -> size_t;
}
pub type size_t = libc::c_ulong;
pub type UnitTestFunction = Option::<unsafe extern "C" fn() -> ()>;
unsafe extern "C" fn run_test(mut test: UnitTestFunction) {
    alloc_test_set_limit(-(1 as libc::c_int));
    test.expect("non-null function pointer")();
    if alloc_test_get_allocated() == 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"alloc_test_get_allocated() == 0\0" as *const u8 as *const libc::c_char,
            b"framework.c\0" as *const u8 as *const libc::c_char,
            45 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void run_test(UnitTestFunction)\0"))
                .as_ptr(),
        );
    }
    'c_1520: {
        if alloc_test_get_allocated() == 0 as libc::c_int as libc::c_ulong {} else {
            __assert_fail(
                b"alloc_test_get_allocated() == 0\0" as *const u8 as *const libc::c_char,
                b"framework.c\0" as *const u8 as *const libc::c_char,
                45 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void run_test(UnitTestFunction)\0"))
                    .as_ptr(),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn run_tests(mut tests: *mut UnitTestFunction) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (*tests.offset(i as isize)).is_some() {
        run_test(*tests.offset(i as isize));
        i += 1;
        i;
    }
}
