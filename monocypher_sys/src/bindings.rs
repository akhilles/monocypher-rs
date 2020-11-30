/* automatically generated by rust-bindgen 0.56.0 */

pub const __WORDSIZE: u32 = 64;
pub const __DARWIN_ONLY_64_BIT_INO_T: u32 = 0;
pub const __DARWIN_ONLY_UNIX_CONFORMANCE: u32 = 1;
pub const __DARWIN_ONLY_VERS_1050: u32 = 0;
pub const __DARWIN_UNIX03: u32 = 1;
pub const __DARWIN_64_BIT_INO_T: u32 = 1;
pub const __DARWIN_VERS_1050: u32 = 1;
pub const __DARWIN_NON_CANCELABLE: u32 = 0;
pub const __DARWIN_SUF_64_BIT_INO_T: &'static [u8; 9usize] = b"$INODE64\0";
pub const __DARWIN_SUF_1050: &'static [u8; 6usize] = b"$1050\0";
pub const __DARWIN_SUF_EXTSN: &'static [u8; 14usize] = b"$DARWIN_EXTSN\0";
pub const __DARWIN_C_ANSI: u32 = 4096;
pub const __DARWIN_C_FULL: u32 = 900000;
pub const __DARWIN_C_LEVEL: u32 = 900000;
pub const __STDC_WANT_LIB_EXT1__: u32 = 1;
pub const __DARWIN_NO_LONG_LONG: u32 = 0;
pub const _DARWIN_FEATURE_64_BIT_INODE: u32 = 1;
pub const _DARWIN_FEATURE_ONLY_UNIX_CONFORMANCE: u32 = 1;
pub const _DARWIN_FEATURE_UNIX_CONFORMANCE: u32 = 3;
pub const __PTHREAD_SIZE__: u32 = 8176;
pub const __PTHREAD_ATTR_SIZE__: u32 = 56;
pub const __PTHREAD_MUTEXATTR_SIZE__: u32 = 8;
pub const __PTHREAD_MUTEX_SIZE__: u32 = 56;
pub const __PTHREAD_CONDATTR_SIZE__: u32 = 8;
pub const __PTHREAD_COND_SIZE__: u32 = 40;
pub const __PTHREAD_ONCE_SIZE__: u32 = 8;
pub const __PTHREAD_RWLOCK_SIZE__: u32 = 192;
pub const __PTHREAD_RWLOCKATTR_SIZE__: u32 = 16;
pub const INT8_MAX: u32 = 127;
pub const INT16_MAX: u32 = 32767;
pub const INT32_MAX: u32 = 2147483647;
pub const INT64_MAX: u64 = 9223372036854775807;
pub const INT8_MIN: i32 = -128;
pub const INT16_MIN: i32 = -32768;
pub const INT32_MIN: i32 = -2147483648;
pub const INT64_MIN: i64 = -9223372036854775808;
pub const UINT8_MAX: u32 = 255;
pub const UINT16_MAX: u32 = 65535;
pub const UINT32_MAX: u32 = 4294967295;
pub const UINT64_MAX: i32 = -1;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST64_MIN: i64 = -9223372036854775808;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const INT_LEAST64_MAX: u64 = 9223372036854775807;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const UINT_LEAST64_MAX: i32 = -1;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST16_MIN: i32 = -32768;
pub const INT_FAST32_MIN: i32 = -2147483648;
pub const INT_FAST64_MIN: i64 = -9223372036854775808;
pub const INT_FAST8_MAX: u32 = 127;
pub const INT_FAST16_MAX: u32 = 32767;
pub const INT_FAST32_MAX: u32 = 2147483647;
pub const INT_FAST64_MAX: u64 = 9223372036854775807;
pub const UINT_FAST8_MAX: u32 = 255;
pub const UINT_FAST16_MAX: u32 = 65535;
pub const UINT_FAST32_MAX: u32 = 4294967295;
pub const UINT_FAST64_MAX: i32 = -1;
pub const INTPTR_MAX: u64 = 9223372036854775807;
pub const INTPTR_MIN: i64 = -9223372036854775808;
pub const UINTPTR_MAX: i32 = -1;
pub const SIZE_MAX: i32 = -1;
pub const RSIZE_MAX: i32 = -1;
pub const WINT_MIN: i32 = -2147483648;
pub const WINT_MAX: u32 = 2147483647;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub type size_t = ::std::os::raw::c_ulong;
pub type wchar_t = ::std::os::raw::c_int;
pub type max_align_t = u128;
pub type int_least8_t = i8;
pub type int_least16_t = i16;
pub type int_least32_t = i32;
pub type int_least64_t = i64;
pub type uint_least8_t = u8;
pub type uint_least16_t = u16;
pub type uint_least32_t = u32;
pub type uint_least64_t = u64;
pub type int_fast8_t = i8;
pub type int_fast16_t = i16;
pub type int_fast32_t = i32;
pub type int_fast64_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u16;
pub type uint_fast32_t = u32;
pub type uint_fast64_t = u64;
pub type __int8_t = ::std::os::raw::c_schar;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __int16_t = ::std::os::raw::c_short;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_longlong;
pub type __uint64_t = ::std::os::raw::c_ulonglong;
pub type __darwin_intptr_t = ::std::os::raw::c_long;
pub type __darwin_natural_t = ::std::os::raw::c_uint;
pub type __darwin_ct_rune_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union __mbstate_t {
    pub __mbstate8: [::std::os::raw::c_char; 128usize],
    pub _mbstateL: ::std::os::raw::c_longlong,
    _bindgen_union_align: [u64; 16usize],
}
#[test]
fn bindgen_test_layout___mbstate_t() {
    assert_eq!(
        ::std::mem::size_of::<__mbstate_t>(),
        128usize,
        concat!("Size of: ", stringify!(__mbstate_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__mbstate_t>(),
        8usize,
        concat!("Alignment of ", stringify!(__mbstate_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__mbstate_t>())).__mbstate8 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__mbstate_t),
            "::",
            stringify!(__mbstate8)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__mbstate_t>()))._mbstateL as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__mbstate_t),
            "::",
            stringify!(_mbstateL)
        )
    );
}
pub type __darwin_mbstate_t = __mbstate_t;
pub type __darwin_ptrdiff_t = ::std::os::raw::c_long;
pub type __darwin_size_t = ::std::os::raw::c_ulong;
pub type __darwin_va_list = __builtin_va_list;
pub type __darwin_wchar_t = ::std::os::raw::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type __darwin_wint_t = ::std::os::raw::c_int;
pub type __darwin_clock_t = ::std::os::raw::c_ulong;
pub type __darwin_socklen_t = __uint32_t;
pub type __darwin_ssize_t = ::std::os::raw::c_long;
pub type __darwin_time_t = ::std::os::raw::c_long;
pub type __darwin_blkcnt_t = __int64_t;
pub type __darwin_blksize_t = __int32_t;
pub type __darwin_dev_t = __int32_t;
pub type __darwin_fsblkcnt_t = ::std::os::raw::c_uint;
pub type __darwin_fsfilcnt_t = ::std::os::raw::c_uint;
pub type __darwin_gid_t = __uint32_t;
pub type __darwin_id_t = __uint32_t;
pub type __darwin_ino64_t = __uint64_t;
pub type __darwin_ino_t = __darwin_ino64_t;
pub type __darwin_mach_port_name_t = __darwin_natural_t;
pub type __darwin_mach_port_t = __darwin_mach_port_name_t;
pub type __darwin_mode_t = __uint16_t;
pub type __darwin_off_t = __int64_t;
pub type __darwin_pid_t = __int32_t;
pub type __darwin_sigset_t = __uint32_t;
pub type __darwin_suseconds_t = __int32_t;
pub type __darwin_uid_t = __uint32_t;
pub type __darwin_useconds_t = __uint32_t;
pub type __darwin_uuid_t = [::std::os::raw::c_uchar; 16usize];
pub type __darwin_uuid_string_t = [::std::os::raw::c_char; 37usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_pthread_handler_rec {
    pub __routine: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    pub __arg: *mut ::std::os::raw::c_void,
    pub __next: *mut __darwin_pthread_handler_rec,
}
#[test]
fn bindgen_test_layout___darwin_pthread_handler_rec() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_pthread_handler_rec>(),
        24usize,
        concat!("Size of: ", stringify!(__darwin_pthread_handler_rec))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_pthread_handler_rec>(),
        8usize,
        concat!("Alignment of ", stringify!(__darwin_pthread_handler_rec))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_pthread_handler_rec>())).__routine as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_pthread_handler_rec),
            "::",
            stringify!(__routine)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_pthread_handler_rec>())).__arg as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_pthread_handler_rec),
            "::",
            stringify!(__arg)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_pthread_handler_rec>())).__next as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_pthread_handler_rec),
            "::",
            stringify!(__next)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _opaque_pthread_attr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 56usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_attr_t() {
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_attr_t>(),
        64usize,
        concat!("Size of: ", stringify!(_opaque_pthread_attr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_attr_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_attr_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_opaque_pthread_attr_t>())).__sig as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_attr_t),
            "::",
            stringify!(__sig)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_opaque_pthread_attr_t>())).__opaque as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_attr_t),
            "::",
            stringify!(__opaque)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _opaque_pthread_cond_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 40usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_cond_t() {
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_cond_t>(),
        48usize,
        concat!("Size of: ", stringify!(_opaque_pthread_cond_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_cond_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_cond_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_opaque_pthread_cond_t>())).__sig as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_cond_t),
            "::",
            stringify!(__sig)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_opaque_pthread_cond_t>())).__opaque as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_cond_t),
            "::",
            stringify!(__opaque)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_condattr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 8usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_condattr_t() {
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_condattr_t>(),
        16usize,
        concat!("Size of: ", stringify!(_opaque_pthread_condattr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_condattr_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_condattr_t))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_opaque_pthread_condattr_t>())).__sig as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_condattr_t),
            "::",
            stringify!(__sig)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_opaque_pthread_condattr_t>())).__opaque as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_condattr_t),
            "::",
            stringify!(__opaque)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _opaque_pthread_mutex_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 56usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_mutex_t() {
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_mutex_t>(),
        64usize,
        concat!("Size of: ", stringify!(_opaque_pthread_mutex_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_mutex_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_mutex_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_opaque_pthread_mutex_t>())).__sig as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_mutex_t),
            "::",
            stringify!(__sig)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_opaque_pthread_mutex_t>())).__opaque as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_mutex_t),
            "::",
            stringify!(__opaque)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_mutexattr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 8usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_mutexattr_t() {
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_mutexattr_t>(),
        16usize,
        concat!("Size of: ", stringify!(_opaque_pthread_mutexattr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_mutexattr_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_mutexattr_t))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_opaque_pthread_mutexattr_t>())).__sig as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_mutexattr_t),
            "::",
            stringify!(__sig)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_opaque_pthread_mutexattr_t>())).__opaque as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_mutexattr_t),
            "::",
            stringify!(__opaque)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_once_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 8usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_once_t() {
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_once_t>(),
        16usize,
        concat!("Size of: ", stringify!(_opaque_pthread_once_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_once_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_once_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_opaque_pthread_once_t>())).__sig as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_once_t),
            "::",
            stringify!(__sig)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_opaque_pthread_once_t>())).__opaque as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_once_t),
            "::",
            stringify!(__opaque)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _opaque_pthread_rwlock_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 192usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_rwlock_t() {
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_rwlock_t>(),
        200usize,
        concat!("Size of: ", stringify!(_opaque_pthread_rwlock_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_rwlock_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_rwlock_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_opaque_pthread_rwlock_t>())).__sig as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_rwlock_t),
            "::",
            stringify!(__sig)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_opaque_pthread_rwlock_t>())).__opaque as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_rwlock_t),
            "::",
            stringify!(__opaque)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_rwlockattr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 16usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_rwlockattr_t() {
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_rwlockattr_t>(),
        24usize,
        concat!("Size of: ", stringify!(_opaque_pthread_rwlockattr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_rwlockattr_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_rwlockattr_t))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_opaque_pthread_rwlockattr_t>())).__sig as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_rwlockattr_t),
            "::",
            stringify!(__sig)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_opaque_pthread_rwlockattr_t>())).__opaque as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_rwlockattr_t),
            "::",
            stringify!(__opaque)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _opaque_pthread_t {
    pub __sig: ::std::os::raw::c_long,
    pub __cleanup_stack: *mut __darwin_pthread_handler_rec,
    pub __opaque: [::std::os::raw::c_char; 8176usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_t() {
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_t>(),
        8192usize,
        concat!("Size of: ", stringify!(_opaque_pthread_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_opaque_pthread_t>())).__sig as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_t),
            "::",
            stringify!(__sig)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_opaque_pthread_t>())).__cleanup_stack as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_t),
            "::",
            stringify!(__cleanup_stack)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_opaque_pthread_t>())).__opaque as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_t),
            "::",
            stringify!(__opaque)
        )
    );
}
pub type __darwin_pthread_attr_t = _opaque_pthread_attr_t;
pub type __darwin_pthread_cond_t = _opaque_pthread_cond_t;
pub type __darwin_pthread_condattr_t = _opaque_pthread_condattr_t;
pub type __darwin_pthread_key_t = ::std::os::raw::c_ulong;
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
pub type __darwin_pthread_mutexattr_t = _opaque_pthread_mutexattr_t;
pub type __darwin_pthread_once_t = _opaque_pthread_once_t;
pub type __darwin_pthread_rwlock_t = _opaque_pthread_rwlock_t;
pub type __darwin_pthread_rwlockattr_t = _opaque_pthread_rwlockattr_t;
pub type __darwin_pthread_t = *mut _opaque_pthread_t;
pub type u_int8_t = ::std::os::raw::c_uchar;
pub type u_int16_t = ::std::os::raw::c_ushort;
pub type u_int32_t = ::std::os::raw::c_uint;
pub type u_int64_t = ::std::os::raw::c_ulonglong;
pub type register_t = i64;
pub type user_addr_t = u_int64_t;
pub type user_size_t = u_int64_t;
pub type user_ssize_t = i64;
pub type user_long_t = i64;
pub type user_ulong_t = u_int64_t;
pub type user_time_t = i64;
pub type user_off_t = i64;
pub type syscall_arg_t = u_int64_t;
pub type intmax_t = ::std::os::raw::c_long;
pub type uintmax_t = ::std::os::raw::c_ulong;
#[doc = ""]
#[doc = " Type definitions ///"]
#[doc = ""]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct crypto_sign_vtable {
    pub hash: ::std::option::Option<
        unsafe extern "C" fn(hash: *mut u8, message: *const u8, message_size: size_t),
    >,
    pub init: ::std::option::Option<unsafe extern "C" fn(ctx: *mut ::std::os::raw::c_void)>,
    pub update: ::std::option::Option<
        unsafe extern "C" fn(
            ctx: *mut ::std::os::raw::c_void,
            message: *const u8,
            message_size: size_t,
        ),
    >,
    pub final_: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut ::std::os::raw::c_void, hash: *mut u8),
    >,
    pub ctx_size: size_t,
}
#[test]
fn bindgen_test_layout_crypto_sign_vtable() {
    assert_eq!(
        ::std::mem::size_of::<crypto_sign_vtable>(),
        40usize,
        concat!("Size of: ", stringify!(crypto_sign_vtable))
    );
    assert_eq!(
        ::std::mem::align_of::<crypto_sign_vtable>(),
        8usize,
        concat!("Alignment of ", stringify!(crypto_sign_vtable))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_sign_vtable>())).hash as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_sign_vtable),
            "::",
            stringify!(hash)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_sign_vtable>())).init as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_sign_vtable),
            "::",
            stringify!(init)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_sign_vtable>())).update as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_sign_vtable),
            "::",
            stringify!(update)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_sign_vtable>())).final_ as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_sign_vtable),
            "::",
            stringify!(final_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_sign_vtable>())).ctx_size as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_sign_vtable),
            "::",
            stringify!(ctx_size)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct crypto_poly1305_ctx {
    pub r: [u32; 4usize],
    pub h: [u32; 5usize],
    pub c: [u32; 5usize],
    pub pad: [u32; 4usize],
    pub c_idx: size_t,
}
#[test]
fn bindgen_test_layout_crypto_poly1305_ctx() {
    assert_eq!(
        ::std::mem::size_of::<crypto_poly1305_ctx>(),
        80usize,
        concat!("Size of: ", stringify!(crypto_poly1305_ctx))
    );
    assert_eq!(
        ::std::mem::align_of::<crypto_poly1305_ctx>(),
        8usize,
        concat!("Alignment of ", stringify!(crypto_poly1305_ctx))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_poly1305_ctx>())).r as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_poly1305_ctx),
            "::",
            stringify!(r)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_poly1305_ctx>())).h as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_poly1305_ctx),
            "::",
            stringify!(h)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_poly1305_ctx>())).c as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_poly1305_ctx),
            "::",
            stringify!(c)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_poly1305_ctx>())).pad as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_poly1305_ctx),
            "::",
            stringify!(pad)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_poly1305_ctx>())).c_idx as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_poly1305_ctx),
            "::",
            stringify!(c_idx)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct crypto_blake2b_ctx {
    pub hash: [u64; 8usize],
    pub input_offset: [u64; 2usize],
    pub input: [u64; 16usize],
    pub input_idx: size_t,
    pub hash_size: size_t,
}
#[test]
fn bindgen_test_layout_crypto_blake2b_ctx() {
    assert_eq!(
        ::std::mem::size_of::<crypto_blake2b_ctx>(),
        224usize,
        concat!("Size of: ", stringify!(crypto_blake2b_ctx))
    );
    assert_eq!(
        ::std::mem::align_of::<crypto_blake2b_ctx>(),
        8usize,
        concat!("Alignment of ", stringify!(crypto_blake2b_ctx))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_blake2b_ctx>())).hash as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_blake2b_ctx),
            "::",
            stringify!(hash)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_blake2b_ctx>())).input_offset as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_blake2b_ctx),
            "::",
            stringify!(input_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_blake2b_ctx>())).input as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_blake2b_ctx),
            "::",
            stringify!(input)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_blake2b_ctx>())).input_idx as *const _ as usize },
        208usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_blake2b_ctx),
            "::",
            stringify!(input_idx)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_blake2b_ctx>())).hash_size as *const _ as usize },
        216usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_blake2b_ctx),
            "::",
            stringify!(hash_size)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_sign_ctx_abstract {
    pub hash: *const crypto_sign_vtable,
    pub buf: [u8; 96usize],
    pub pk: [u8; 32usize],
}
#[test]
fn bindgen_test_layout_crypto_sign_ctx_abstract() {
    assert_eq!(
        ::std::mem::size_of::<crypto_sign_ctx_abstract>(),
        136usize,
        concat!("Size of: ", stringify!(crypto_sign_ctx_abstract))
    );
    assert_eq!(
        ::std::mem::align_of::<crypto_sign_ctx_abstract>(),
        8usize,
        concat!("Alignment of ", stringify!(crypto_sign_ctx_abstract))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_sign_ctx_abstract>())).hash as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_sign_ctx_abstract),
            "::",
            stringify!(hash)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_sign_ctx_abstract>())).buf as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_sign_ctx_abstract),
            "::",
            stringify!(buf)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_sign_ctx_abstract>())).pk as *const _ as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_sign_ctx_abstract),
            "::",
            stringify!(pk)
        )
    );
}
pub type crypto_check_ctx_abstract = crypto_sign_ctx_abstract;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_sign_ctx {
    pub ctx: crypto_sign_ctx_abstract,
    pub hash: crypto_blake2b_ctx,
}
#[test]
fn bindgen_test_layout_crypto_sign_ctx() {
    assert_eq!(
        ::std::mem::size_of::<crypto_sign_ctx>(),
        360usize,
        concat!("Size of: ", stringify!(crypto_sign_ctx))
    );
    assert_eq!(
        ::std::mem::align_of::<crypto_sign_ctx>(),
        8usize,
        concat!("Alignment of ", stringify!(crypto_sign_ctx))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_sign_ctx>())).ctx as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_sign_ctx),
            "::",
            stringify!(ctx)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<crypto_sign_ctx>())).hash as *const _ as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(crypto_sign_ctx),
            "::",
            stringify!(hash)
        )
    );
}
pub type crypto_check_ctx = crypto_sign_ctx;
extern "C" {
    #[doc = ""]
    #[doc = " High level interface ///"]
    #[doc = ""]
    pub fn crypto_verify16(a: *const u8, b: *const u8) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_verify32(a: *const u8, b: *const u8) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_verify64(a: *const u8, b: *const u8) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_wipe(secret: *mut ::std::os::raw::c_void, size: size_t);
}
extern "C" {
    pub fn crypto_lock(
        mac: *mut u8,
        cipher_text: *mut u8,
        key: *const u8,
        nonce: *const u8,
        plain_text: *const u8,
        text_size: size_t,
    );
}
extern "C" {
    pub fn crypto_unlock(
        plain_text: *mut u8,
        key: *const u8,
        nonce: *const u8,
        mac: *const u8,
        cipher_text: *const u8,
        text_size: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_lock_aead(
        mac: *mut u8,
        cipher_text: *mut u8,
        key: *const u8,
        nonce: *const u8,
        ad: *const u8,
        ad_size: size_t,
        plain_text: *const u8,
        text_size: size_t,
    );
}
extern "C" {
    pub fn crypto_unlock_aead(
        plain_text: *mut u8,
        key: *const u8,
        nonce: *const u8,
        mac: *const u8,
        ad: *const u8,
        ad_size: size_t,
        cipher_text: *const u8,
        text_size: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_blake2b(hash: *mut u8, message: *const u8, message_size: size_t);
}
extern "C" {
    pub fn crypto_blake2b_general(
        hash: *mut u8,
        hash_size: size_t,
        key: *const u8,
        key_size: size_t,
        message: *const u8,
        message_size: size_t,
    );
}
extern "C" {
    pub fn crypto_blake2b_init(ctx: *mut crypto_blake2b_ctx);
}
extern "C" {
    pub fn crypto_blake2b_update(
        ctx: *mut crypto_blake2b_ctx,
        message: *const u8,
        message_size: size_t,
    );
}
extern "C" {
    pub fn crypto_blake2b_final(ctx: *mut crypto_blake2b_ctx, hash: *mut u8);
}
extern "C" {
    pub fn crypto_blake2b_general_init(
        ctx: *mut crypto_blake2b_ctx,
        hash_size: size_t,
        key: *const u8,
        key_size: size_t,
    );
}
extern "C" {
    pub static crypto_blake2b_vtable: crypto_sign_vtable;
}
extern "C" {
    pub fn crypto_argon2i(
        hash: *mut u8,
        hash_size: u32,
        work_area: *mut ::std::os::raw::c_void,
        nb_blocks: u32,
        nb_iterations: u32,
        password: *const u8,
        password_size: u32,
        salt: *const u8,
        salt_size: u32,
    );
}
extern "C" {
    pub fn crypto_argon2i_general(
        hash: *mut u8,
        hash_size: u32,
        work_area: *mut ::std::os::raw::c_void,
        nb_blocks: u32,
        nb_iterations: u32,
        password: *const u8,
        password_size: u32,
        salt: *const u8,
        salt_size: u32,
        key: *const u8,
        key_size: u32,
        ad: *const u8,
        ad_size: u32,
    );
}
extern "C" {
    pub fn crypto_key_exchange(
        shared_key: *mut u8,
        your_secret_key: *const u8,
        their_public_key: *const u8,
    );
}
extern "C" {
    pub fn crypto_sign_public_key(public_key: *mut u8, secret_key: *const u8);
}
extern "C" {
    pub fn crypto_sign(
        signature: *mut u8,
        secret_key: *const u8,
        public_key: *const u8,
        message: *const u8,
        message_size: size_t,
    );
}
extern "C" {
    pub fn crypto_check(
        signature: *const u8,
        public_key: *const u8,
        message: *const u8,
        message_size: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_sign_init_first_pass(
        ctx: *mut crypto_sign_ctx_abstract,
        secret_key: *const u8,
        public_key: *const u8,
    );
}
extern "C" {
    pub fn crypto_sign_update(
        ctx: *mut crypto_sign_ctx_abstract,
        message: *const u8,
        message_size: size_t,
    );
}
extern "C" {
    pub fn crypto_sign_init_second_pass(ctx: *mut crypto_sign_ctx_abstract);
}
extern "C" {
    pub fn crypto_sign_final(ctx: *mut crypto_sign_ctx_abstract, signature: *mut u8);
}
extern "C" {
    pub fn crypto_check_init(
        ctx: *mut crypto_check_ctx_abstract,
        signature: *const u8,
        public_key: *const u8,
    );
}
extern "C" {
    pub fn crypto_check_update(
        ctx: *mut crypto_check_ctx_abstract,
        message: *const u8,
        message_size: size_t,
    );
}
extern "C" {
    pub fn crypto_check_final(ctx: *mut crypto_check_ctx_abstract) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_sign_public_key_custom_hash(
        public_key: *mut u8,
        secret_key: *const u8,
        hash: *const crypto_sign_vtable,
    );
}
extern "C" {
    pub fn crypto_sign_init_first_pass_custom_hash(
        ctx: *mut crypto_sign_ctx_abstract,
        secret_key: *const u8,
        public_key: *const u8,
        hash: *const crypto_sign_vtable,
    );
}
extern "C" {
    pub fn crypto_check_init_custom_hash(
        ctx: *mut crypto_check_ctx_abstract,
        signature: *const u8,
        public_key: *const u8,
        hash: *const crypto_sign_vtable,
    );
}
extern "C" {
    pub fn crypto_from_eddsa_private(x25519: *mut u8, eddsa: *const u8);
}
extern "C" {
    pub fn crypto_from_eddsa_public(x25519: *mut u8, eddsa: *const u8);
}
extern "C" {
    pub fn crypto_hidden_to_curve(curve: *mut u8, hidden: *const u8);
}
extern "C" {
    pub fn crypto_curve_to_hidden(
        hidden: *mut u8,
        curve: *const u8,
        tweak: u8,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn crypto_hidden_key_pair(hidden: *mut u8, secret_key: *mut u8, seed: *mut u8);
}
extern "C" {
    #[doc = ""]
    #[doc = " Low level primitives ///"]
    #[doc = ""]
    pub fn crypto_hchacha20(out: *mut u8, key: *const u8, in_: *const u8);
}
extern "C" {
    pub fn crypto_chacha20(
        cipher_text: *mut u8,
        plain_text: *const u8,
        text_size: size_t,
        key: *const u8,
        nonce: *const u8,
    );
}
extern "C" {
    pub fn crypto_xchacha20(
        cipher_text: *mut u8,
        plain_text: *const u8,
        text_size: size_t,
        key: *const u8,
        nonce: *const u8,
    );
}
extern "C" {
    pub fn crypto_ietf_chacha20(
        cipher_text: *mut u8,
        plain_text: *const u8,
        text_size: size_t,
        key: *const u8,
        nonce: *const u8,
    );
}
extern "C" {
    pub fn crypto_chacha20_ctr(
        cipher_text: *mut u8,
        plain_text: *const u8,
        text_size: size_t,
        key: *const u8,
        nonce: *const u8,
        ctr: u64,
    ) -> u64;
}
extern "C" {
    pub fn crypto_xchacha20_ctr(
        cipher_text: *mut u8,
        plain_text: *const u8,
        text_size: size_t,
        key: *const u8,
        nonce: *const u8,
        ctr: u64,
    ) -> u64;
}
extern "C" {
    pub fn crypto_ietf_chacha20_ctr(
        cipher_text: *mut u8,
        plain_text: *const u8,
        text_size: size_t,
        key: *const u8,
        nonce: *const u8,
        ctr: u32,
    ) -> u32;
}
extern "C" {
    pub fn crypto_poly1305(mac: *mut u8, message: *const u8, message_size: size_t, key: *const u8);
}
extern "C" {
    pub fn crypto_poly1305_init(ctx: *mut crypto_poly1305_ctx, key: *const u8);
}
extern "C" {
    pub fn crypto_poly1305_update(
        ctx: *mut crypto_poly1305_ctx,
        message: *const u8,
        message_size: size_t,
    );
}
extern "C" {
    pub fn crypto_poly1305_final(ctx: *mut crypto_poly1305_ctx, mac: *mut u8);
}
extern "C" {
    pub fn crypto_x25519_public_key(public_key: *mut u8, secret_key: *const u8);
}
extern "C" {
    pub fn crypto_x25519(
        raw_shared_secret: *mut u8,
        your_secret_key: *const u8,
        their_public_key: *const u8,
    );
}
extern "C" {
    pub fn crypto_x25519_dirty_small(pk: *mut u8, sk: *const u8);
}
extern "C" {
    pub fn crypto_x25519_dirty_fast(pk: *mut u8, sk: *const u8);
}
extern "C" {
    pub fn crypto_x25519_inverse(
        blind_salt: *mut u8,
        private_key: *const u8,
        curve_point: *const u8,
    );
}
pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: ::std::os::raw::c_uint,
    pub fp_offset: ::std::os::raw::c_uint,
    pub overflow_arg_area: *mut ::std::os::raw::c_void,
    pub reg_save_area: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout___va_list_tag() {
    assert_eq!(
        ::std::mem::size_of::<__va_list_tag>(),
        24usize,
        concat!("Size of: ", stringify!(__va_list_tag))
    );
    assert_eq!(
        ::std::mem::align_of::<__va_list_tag>(),
        8usize,
        concat!("Alignment of ", stringify!(__va_list_tag))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__va_list_tag>())).gp_offset as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(gp_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__va_list_tag>())).fp_offset as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(fp_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__va_list_tag>())).overflow_arg_area as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(overflow_arg_area)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__va_list_tag>())).reg_save_area as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(reg_save_area)
        )
    );
}
