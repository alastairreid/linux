// SPDX-License-Identifier: GPL-2.0

//! Rust character device sample

#![no_std]
#![feature(allocator_api, global_asm)]

use alloc::boxed::Box;
use core::pin::Pin;
use kernel::prelude::*;
use kernel::{c_str, chrdev, file_operations::FileOperations};

use kernel::{c_types, user_ptr::UserSlicePtr, file_operations::File};
use alloc::vec::Vec;
use kernel::bindings;
use kernel::Error;

module! {
    type: RustChrdev,
    name: b"rust_chrdev",
    author: b"Rust for Linux Contributors",
    description: b"Rust character device sample",
    license: b"GPL v2",
}

#[derive(Default)]
struct RustFile;

impl FileOperations for RustFile {
    kernel::declare_file_operations!();
}

struct RustChrdev {
    _dev: Pin<Box<chrdev::Registration<2>>>,
}

impl KernelModule for RustChrdev {
    fn init() -> Result<Self> {
        pr_info!("Rust character device sample (init)\n");

        let mut chrdev_reg =
            chrdev::Registration::new_pinned(c_str!("rust_chrdev"), 0, &THIS_MODULE)?;

        // Register the same kind of device twice, we're just demonstrating
        // that you can use multiple minors. There are two minors in this case
        // because its type is `chrdev::Registration<2>`
        chrdev_reg.as_mut().register::<RustFile>()?;
        chrdev_reg.as_mut().register::<RustFile>()?;

        Ok(RustChrdev { _dev: chrdev_reg })
    }
}

impl Drop for RustChrdev {
    fn drop(&mut self) {
        pr_info!("Rust character device sample (exit)\n");
    }
}

#[no_mangle]
pub fn test_fileops() -> KernelResult<()> {
    // it is not clear to me how this is being called automatically
    // let dev = RustChrdev::init()?;

    let ctx = ();
    let f: Box<RustFile> = RustFile::open(&ctx)?;

    // note: we expect the following to fail because the above code does
    // not implement any FileOperations so we should get Error::EINVAL

    let len: usize = 128; // any size that kmalloc accepts should do here
    let mut data: Vec<u8> = Vec::with_capacity(len);
    let buf: *mut u8 = data.as_mut_ptr();

    // how do we build a file?
    // I think we would need to have the kernel do that - but we don't want
    // to call the kernel code so either
    // - use a null ptr and hope nobody uses it
    // or
    // - modify file_operations::File implementation to suit our needs
    let fptr: *const bindings::file = core::ptr::null();
    let file: File = unsafe { File::from_ptr(fptr) }; // hack: I had to make this function public to allow this
    let mut data = unsafe { UserSlicePtr::new(buf as *mut c_types::c_void, len).writer() };
    let offset: u64 = 0;
    match f.read(&file, &mut data, offset) {
        Err(Error(rc)) => pr_info!("read error: {}", rc),
        Ok(sz) => pr_info!("read {} bytes", sz),
    }

    Ok(())
}
