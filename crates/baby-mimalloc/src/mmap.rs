use crate::Mimalloc;
use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use libc::{mmap, munmap, MAP_FAILED};

/// A simple `mmap`-based allocator that can be used to power [`Mimalloc`].
///
/// It is only used to allocate large chunks of memory and is not suitable for general malloc.
#[derive(Default)]
pub struct MmapAlloc;

/// [`Mimalloc`] powered by `mmap` ([`MmapAlloc`]).
pub type MimallocMmap = Mimalloc<MmapAlloc>;

/// Create a new [`MimallocMmap`] instance by a `const fn`.
pub const fn new_mimalloc_mmap() -> MimallocMmap {
    Mimalloc::with_os_allocator(MmapAlloc)
}

unsafe impl GlobalAlloc for MmapAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // try mapping exactly `size` at first
        let p = mmap(
            null_mut(),
            layout.size(),
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
            -1,
            0,
        );
        if p != MAP_FAILED {
            if p as usize % layout.align() == 0 {
                return p.cast();
            }
            // not aligned
            munmap(p, layout.size());
        }

        // over allocate to ensure alignment
        let start = mmap(
            null_mut(),
            layout.size() + layout.align() - 1,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
            -1,
            0,
        );
        if start == MAP_FAILED {
            null_mut()
        } else {
            let offset = start.align_offset(layout.align());
            let aligned = start.add(offset);
            if offset != 0 {
                munmap(start, offset);
            }
            if offset != layout.align() - 1 {
                let end = aligned.add(layout.size());
                munmap(end, layout.align() - 1 - offset);
            }
            aligned.cast()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        munmap(ptr.cast(), layout.size());
    }
}
