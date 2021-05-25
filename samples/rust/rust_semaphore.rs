// SPDX-License-Identifier: GPL-2.0

//! Rust semaphore sample
//!
//! A counting semaphore that can be used by userspace.
//!
//! The count is incremented by writes to the device. A write of `n` bytes results in an increment
//! of `n`. It is decremented by reads; each read results in the count being decremented by 1. If
//! the count is already zero, a read will block until another write increments it.
//!
//! This can be used in user space from the shell for example  as follows (assuming a node called
//! `semaphore`): `cat semaphore` decrements the count by 1 (waiting for it to become non-zero
//! before decrementing); `echo -n 123 > semaphore` increments the semaphore by 3, potentially
//! unblocking up to 3 blocked readers.

#![no_std]
#![feature(allocator_api, global_asm)]

use alloc::{boxed::Box, sync::Arc};
use core::{
    pin::Pin,
    sync::atomic::{AtomicU64, Ordering},
};
use kernel::{
    c_str, condvar_init, declare_file_operations,
    file::File,
    file_operations::{FileOpener, FileOperations, IoctlCommand, IoctlHandler},
    io_buffer::{IoBufferReader, IoBufferWriter},
    miscdev::Registration,
    mutex_init,
    prelude::*,
    sync::{CondVar, Mutex},
    user_ptr::{UserSlicePtrReader, UserSlicePtrWriter},
    Error,
};

module! {
    type: RustSemaphore,
    name: b"rust_semaphore",
    author: b"Rust for Linux Contributors",
    description: b"Rust semaphore sample",
    license: b"GPL v2",
}

struct SemaphoreInner {
    count: usize,
    max_seen: usize,
}

struct Semaphore {
    changed: CondVar,
    inner: Mutex<SemaphoreInner>,
}

struct FileState {
    read_count: AtomicU64,
    shared: Arc<Semaphore>,
}

impl FileState {
    fn consume(&self) -> Result {
        let mut inner = self.shared.inner.lock();
        while inner.count == 0 {
            if self.shared.changed.wait(&mut inner) {
                return Err(Error::EINTR);
            }
        }
        inner.count -= 1;
        Ok(())
    }
}

impl FileOpener<Arc<Semaphore>> for FileState {
    fn open(shared: &Arc<Semaphore>) -> Result<Box<Self>> {
        Ok(Box::try_new(Self {
            read_count: AtomicU64::new(0),
            shared: shared.clone(),
        })?)
    }
}

impl FileOperations for FileState {
    type Wrapper = Box<Self>;

    declare_file_operations!(read, write, ioctl);

    fn read<T: IoBufferWriter>(&self, _: &File, data: &mut T, offset: u64) -> Result<usize> {
        if data.is_empty() || offset > 0 {
            return Ok(0);
        }
        self.consume()?;
        data.write_slice(&[0u8; 1])?;
        self.read_count.fetch_add(1, Ordering::Relaxed);
        Ok(1)
    }

    fn write<T: IoBufferReader>(&self, _: &File, data: &mut T, _offset: u64) -> Result<usize> {
        {
            let mut inner = self.shared.inner.lock();
            inner.count = inner.count.saturating_add(data.len());
            if inner.count > inner.max_seen {
                inner.max_seen = inner.count;
            }
        }

        self.shared.changed.notify_all();
        Ok(data.len())
    }

    fn ioctl(&self, file: &File, cmd: &mut IoctlCommand) -> Result<i32> {
        cmd.dispatch(self, file)
    }
}

struct RustSemaphore {
    _dev: Pin<Box<Registration<Arc<Semaphore>>>>,
}

impl KernelModule for RustSemaphore {
    fn init() -> Result<Self> {
        pr_info!("Rust semaphore sample (init)\n");

        let sema = Arc::try_new(Semaphore {
            // SAFETY: `condvar_init!` is called below.
            changed: unsafe { CondVar::new() },

            // SAFETY: `mutex_init!` is called below.
            inner: unsafe {
                Mutex::new(SemaphoreInner {
                    count: 0,
                    max_seen: 0,
                })
            },
        })?;

        // SAFETY: `changed` is pinned behind `Arc`.
        condvar_init!(Pin::new_unchecked(&sema.changed), "Semaphore::changed");

        // SAFETY: `inner` is pinned behind `Arc`.
        mutex_init!(Pin::new_unchecked(&sema.inner), "Semaphore::inner");

        Ok(Self {
            _dev: Registration::new_pinned::<FileState>(c_str!("rust_semaphore"), None, sema)?,
        })
    }
}

impl Drop for RustSemaphore {
    fn drop(&mut self) {
        pr_info!("Rust semaphore sample (exit)\n");
    }
}

const IOCTL_GET_READ_COUNT: u32 = 0x80086301;
const IOCTL_SET_READ_COUNT: u32 = 0x40086301;

impl IoctlHandler for FileState {
    fn read(&self, _: &File, cmd: u32, writer: &mut UserSlicePtrWriter) -> Result<i32> {
        match cmd {
            IOCTL_GET_READ_COUNT => {
                writer.write(&self.read_count.load(Ordering::Relaxed))?;
                Ok(0)
            }
            _ => Err(Error::EINVAL),
        }
    }

    fn write(&self, _: &File, cmd: u32, reader: &mut UserSlicePtrReader) -> Result<i32> {
        match cmd {
            IOCTL_SET_READ_COUNT => {
                self.read_count.store(reader.read()?, Ordering::Relaxed);
                Ok(0)
            }
            _ => Err(Error::EINVAL),
        }
    }
}

use alloc::vec::Vec;
use kernel::bindings;
use kernel::{c_types, user_ptr::UserSlicePtr};

fn make_fake_file() -> File {
    // How do we build a file?
    // I think we would need to have the kernel do that - but we don't want to call the kernel code so either
    // - use a null ptr and hope nobody uses it; or
    // - modify file_operations::File implementation to suit our needs
    let fptr: *const bindings::file = core::ptr::null();
    unsafe { File::from_ptr(fptr) } // hack: I had to make this function public to allow this
}

fn make_reader(len: usize) -> UserSlicePtrReader {
    let mut data: Vec<u8> = Vec::with_capacity(len);
    unsafe { UserSlicePtr::new(data.as_mut_ptr() as *mut c_types::c_void, len).reader() }
}

fn make_writer(len: usize) -> UserSlicePtrWriter {
    let mut data: Vec<u8> = Vec::with_capacity(len);
    unsafe { UserSlicePtr::new(data.as_mut_ptr() as *mut c_types::c_void, len).writer() }
}

#[no_mangle]
pub fn test_fileops() -> KernelResult<()>
{
    // 1) Use RustSemaphore::init() to create module state sema
    // 2) Use FileState::open(sema) to get Box<FileState>
    // 3) Test the following operations
    //    - read // should block unless semaphore >= 1
    //    - write // increments semaphore by either 1 or write size (can't figure out which)
    //    - ioctl.read(IOCTL_GET_READ_COUNT)
    //    - ioctl.write(IOCTL_SET_READ_COUNT)
    //    - and all other operations
    let rust_sem = RustSemaphore::init()?;
    pr_info!("Initialized");

    // get a FileState
    let reg: &Pin<Box<Registration<Arc<Semaphore>>>> = &rust_sem._dev;
    let sema: &Arc<Semaphore> = &(*reg).context;
    let file_state: FileState = *FileState::open(&sema)?;
    pr_info!("Got filestate");

    // build a File
    let file = make_fake_file();

    // write some data *before* reading
    pr_info!("Calling write");
    let mut data = make_reader(128); // any size that kmalloc accepts should do here
    let offset: u64 = 0;
    match FileOperations::write(&file_state, &mut data, offset) {
        Err(Error(rc)) => pr_info!("write error: {}", rc),
        Ok(sz) => pr_info!("write {} bytes", sz),
    }
    pr_info!("Called write");

    // read some data (will block if we have not written first)
    pr_info!("Calling read");
    let mut data = make_writer(128); // any size that kmalloc accepts should do here
    match FileOperations::read(&file_state, &file, &mut data, offset) {
        Err(Error(rc)) => pr_info!("read error: {}", rc),
        Ok(sz) => pr_info!("read {} bytes", sz),
    }
    pr_info!("Called read");

    Ok(())
}
