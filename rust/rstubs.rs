// SPDX-License-Identifier: GPL-2.0

#![no_std]
#![feature(
    allocator_api,
    alloc_error_handler,
    const_fn,
    const_mut_refs,
    const_panic,
    try_reserve
)]

use kernel::bindings::*;
use kernel::c_types;

#[no_mangle]
unsafe extern "C" fn cdev_init(_arg1: *mut cdev, _arg2: *const file_operations) {
}

#[no_mangle]
unsafe extern "C" fn cdev_add(_arg1: *mut cdev, _arg2: dev_t, _arg3: c_types::c_uint) -> c_types::c_int {
    0
}

#[no_mangle]
unsafe extern "C" fn cdev_del(_arg1: *mut cdev) {
}

// Can't define this in Rust because it is variadic
// #[no_mangle]
// extern "C" fn printk(fmt: *const c_types::c_char, ...) -> c_types::c_int {
//     // The following implementation looks promising but, because it just
//     // prints the format string, all you see is something like "6%s: %.*s:0"
//     // which is not as useful as I had hoped.
//     extern "C" fn klee_print_expr(msg: *const c_types::c_char, _dummy: i32);
//     unsafe { klee_print_expr(msg, 0); }
//     0
// }

#[no_mangle]
unsafe extern "C" fn __init_waitqueue_head(
        _wq_head: *mut wait_queue_head,
        _name: *const c_types::c_char,
        _arg1: *mut lock_class_key,
    ) {
}

#[no_mangle]
unsafe extern "C" fn __wake_up(
        _wq_head: *mut wait_queue_head,
        _mode: c_types::c_uint,
        _nr: c_types::c_int,
        _key: *mut c_types::c_void,
    ) {}

#[no_mangle]
unsafe extern "C" fn prepare_to_wait_exclusive(
        _wq_head: *mut wait_queue_head,
        _wq_entry: *mut wait_queue_entry,
        _state: c_types::c_int,
    ) {
}

#[no_mangle]
unsafe extern "C" fn schedule() {
}

#[no_mangle]
unsafe extern "C" fn finish_wait(_wq_head: *mut wait_queue_head, _wq_entry: *mut wait_queue_entry) {
}

#[no_mangle]
unsafe extern "C" fn __mutex_init(_lock: *mut mutex, _name: *const c_types::c_char, _key: *mut lock_class_key) {
}

#[no_mangle]
unsafe extern "C" fn mutex_lock(_lock: *mut mutex) {
}

#[no_mangle]
unsafe extern "C" fn mutex_unlock(_lock: *mut mutex) {
}

#[no_mangle]
unsafe extern "C" fn add_device_randomness(_arg1: *const c_types::c_void, _arg2: c_types::c_uint) {
}

#[no_mangle]
unsafe extern "C" fn rng_is_initialized() -> bool_ {
    true
}

#[no_mangle]
unsafe extern "C" fn wait_for_random_bytes() -> c_types::c_int {
    0
}

#[no_mangle]
unsafe extern "C" fn get_random_bytes(_buf: *mut c_types::c_void, _nbytes: c_types::c_int) {
}

#[no_mangle]
unsafe extern "C" fn alloc_chrdev_region(
        _arg1: *mut dev_t,
        _arg2: c_types::c_uint,
        _arg3: c_types::c_uint,
        _arg4: *const c_types::c_char,
    ) -> c_types::c_int {
    0
}

#[no_mangle]
unsafe extern "C" fn register_chrdev_region(
        _arg1: dev_t,
        _arg2: c_types::c_uint,
        _arg3: *const c_types::c_char,
    ) -> c_types::c_int {
    0
}

#[no_mangle]
unsafe extern "C" fn unregister_chrdev_region(_arg1: dev_t, _arg2: c_types::c_uint) {
}

#[no_mangle]
unsafe extern "C" fn kernel_param_lock(_mod_: *mut module) {
}

#[no_mangle]
unsafe extern "C" fn kernel_param_unlock(_mod_: *mut module) {
}

#[no_mangle]
unsafe extern "C" fn slab_is_available() -> bool_ {
    true
}

#[no_mangle]
unsafe extern "C" fn vm_insert_page(
        _arg1: *mut vm_area_struct,
        _addr: c_types::c_ulong,
        _arg2: *mut page,
    ) -> c_types::c_int {
    0
}

#[no_mangle]
unsafe extern "C" fn __free_pages(_page: *mut page, _order: c_types::c_uint) {
}


#[no_mangle]
unsafe extern "C" fn register_sysctl(
        _path: *const c_types::c_char,
        _table: *mut ctl_table,
    ) -> *mut ctl_table_header {
    // as far as I can see, this pointer is only used in ::drop()
    // as an argument to unregister_sysctl_table()
    core::ptr::null_mut()
}

#[no_mangle]
unsafe extern "C" fn unregister_sysctl_table(_table: *mut ctl_table_header) {
}

#[no_mangle]
unsafe extern "C" fn misc_register(_misc: *mut miscdevice) -> c_types::c_int {
    0
}

#[no_mangle]
unsafe extern "C" fn misc_deregister(_misc: *mut miscdevice) {
}
