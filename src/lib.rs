#![no_std]

#[macro_export]
macro_rules! init {
    () => {
        extern "C" {
            fn __init_tls(auxv: core::ffi::c_long);
        }
        unsafe {
            __init_tls(0);
        }
    };
}

#[macro_export]
macro_rules! use_brk_via_growing_elf_end {
    () => {
        $crate::use_brk_via_growing_elf_end!(0x300000);
    };
    ($max_value:expr) => {
        #[no_mangle]
        pub static __ckb_hijack_brk_max: core::ffi::c_long = $max_value;
    };
}

#[macro_export]
macro_rules! use_brk_via_bss_value {
    () => {
        $crate::use_brk_via_bss_value!(307200);
    };
    ($length:expr) => {
        #[no_mangle]
        pub static __ckb_hijack_brk_buffer: [u8; $length + 8191] = [0u8; $length + 8191];

        #[repr(C)]
        struct __SyscallContext {
            n: core::ffi::c_long,
            a: core::ffi::c_long,
            b: core::ffi::c_long,
            c: core::ffi::c_long,
            d: core::ffi::c_long,
            e: core::ffi::c_long,
            f: core::ffi::c_long,
            processed: *mut core::ffi::c_int,
        }

        #[no_mangle]
        pub extern "C" fn __ckb_hijack_brk(c: *mut core::ffi::c_void) -> core::ffi::c_long {
            let context = unsafe { &mut *(c as *mut __SyscallContext) };

            if context.n != 214 {
                return -1;
            }

            unsafe {
                *context.processed = 1;
            }

            let brk_min = __ckb_hijack_brk_buffer.as_ptr() as core::ffi::c_long;
            let brk_max = brk_min + __ckb_hijack_brk_buffer.len() as core::ffi::c_long;
            if context.a == 0 {
                brk_min
            } else if (context.a >= brk_min) && (context.a <= brk_max) {
                context.a
            } else {
                -1
            }
        }
    };
}

#[macro_export]
macro_rules! malloc_as_alloc {
    () => {
        extern "C" {
            fn malloc(size: core::ffi::c_long) -> *mut core::ffi::c_void;
            fn free(ptr: *mut core::ffi::c_void);
        }

        struct MallocAllocator;

        unsafe impl core::alloc::GlobalAlloc for MallocAllocator {
            unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
                malloc(layout.size() as core::ffi::c_long) as *mut u8
            }

            unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
                free(ptr as *mut core::ffi::c_void)
            }
        }

        #[global_allocator]
        static ALLOC: MallocAllocator = MallocAllocator;
    };
}

#[macro_export]
macro_rules! disable_brk_and_mmap {
    () => {
        #[no_mangle]
        pub extern "C" fn __ckb_hijack_brk(_p: *mut core::ffi::c_void) -> core::ffi::c_long {
            -1
        }

        #[no_mangle]
        pub extern "C" fn __ckb_hijack_mmap(_p: *mut core::ffi::c_void) -> core::ffi::c_long {
            -1
        }
    };
}

#[macro_export]
macro_rules! disable_stdout_to_ckb_debug {
    () => {
        #[no_mangle]
        pub extern "C" fn __ckb_hijack_writev(_p: *mut core::ffi::c_void) -> core::ffi::c_long {
            -1
        }

        #[no_mangle]
        pub extern "C" fn __ckb_hijack_ioctl(_p: *mut core::ffi::c_void) -> core::ffi::c_long {
            -1
        }
    };
}
