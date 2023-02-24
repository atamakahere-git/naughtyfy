//! Low level function mapping for fanotify
//!
use lazy_static::lazy_static;
use libc::c_void;

use crate::structs::*;
use std::{io::Error, mem, os::unix::ffi::OsStrExt, slice};

lazy_static! {
    /// Get current platform sizeof of fanotify_event_metadata.
    pub static ref FAN_EVENT_METADATA_LEN: usize = mem::size_of::<fanotify_event_metadata>();
}

/// Length of memory to be allocated for read buffer
pub static mut FAN_EVENT_BUFFER_LEN: usize = 250;

/// Initializes a new fanotify group and returns a
/// file descriptor for the event queue associated with the group.
///
/// The file descriptor is used in calls to [`fanotify_mark()`] to
/// specify the files, directories, mounts, or filesystems for which
/// fanotify events shall be created.  These events are received by
/// reading from the file descriptor.  Some events are only
/// informative, indicating that a file has been accessed.  Other
/// events can be used to determine whether another application is
/// permitted to access a file or directory.  Permission to access
/// filesystem objects is granted by writing to the file descriptor.
///
/// Multiple programs may be using the fanotify interface at the same
/// time to monitor the same files.
///
/// In the current implementation, the number of fanotify groups per
/// user is limited to 128.  This limit cannot be overridden.
///
/// Calling [`fanotify_init()`] requires the `CAP_SYS_ADMIN` capability.
/// This constraint might be relaxed in future versions of the API.
/// Therefore, certain additional capability checks have been
/// implemented as indicated below.
///
/// The flags argument contains a multi-bit field defining the
/// notification class of the listening application and further
/// single bit fields specifying the behavior of the file descriptor.
///
/// If multiple listeners for permission events exist, the
/// notification class is used to establish the sequence in which the
/// listeners receive the events.
pub fn fanotify_init(flags: u32, event_f_flags: u32) -> Result<i32, Error> {
    unsafe {
        match libc::fanotify_init(flags, event_f_flags) {
            -1 => Err(Error::last_os_error()),
            fd => Ok(fd),
        }
    }
}

/// Converts the implemented types to [`std::ffi::OsStr`] using `as_os_str()` method. <br>
/// This is *NOT* [`std::path::Path`]
///
/// # Example
/// ```
/// # pub trait Path {
/// # fn as_os_str(&self) -> &std::ffi::OsStr;
/// # }
/// #
/// # impl Path for str {
/// #     fn as_os_str(&self) -> &std::ffi::OsStr {
/// #         std::ffi::OsStr::new(self)
/// #     }
/// # }
/// let path = std::path::Path::new("/usr/bin");
/// let ostr = path.as_os_str();
/// assert_eq!(ostr,"/usr/bin");
/// ```
pub trait Path {
    fn as_os_str(&self) -> &std::ffi::OsStr;
}

impl Path for std::path::Path {
    fn as_os_str(&self) -> &std::ffi::OsStr {
        self.as_os_str()
    }
}

impl Path for str {
    fn as_os_str(&self) -> &std::ffi::OsStr {
        std::ffi::OsStr::new(self)
    }
}

impl Path for String {
    fn as_os_str(&self) -> &std::ffi::OsStr {
        std::ffi::OsStr::new(self.as_str())
    }
}

/// Adds, removes, or modifies an fanotify mark on a
/// filesystem object. The caller must have read permission on the
/// filesystem object that is to be marked.
///
/// # Arguments
/// * `fanotify_fd` - File descriptor returned by [`fanotify_init()`].
/// * `flags` - Bit mask describing the modification to perform.
/// * `mask` - Which events shall be listened for (or which shall be ignored).
/// * `dirfd` - Defines the filesystem object to be marked.
/// * `path` - Filesystem path of file or diretory.
///
/// The filesystem object to be marked is determined by the file
/// descriptor dirfd and the pathname specified in path:
///
/// * If pathname is `NULL`, dirfd defines the filesystem object to be
///   marked.
/// * If pathname is `NULL`, and dirfd takes the special value
///   `AT_FDCWD`, the current working directory is to be marked.

/// * If pathname is absolute, it defines the filesystem object to
///   be marked, and dirfd is ignored.

/// * If pathname is relative, and dirfd does not have the value
///   `AT_FDCWD`, then the filesystem object to be marked is
///   determined by interpreting pathname relative the directory
///   referred to by dirfd.

/// * If pathname is relative, and dirfd has the value `AT_FDCWD`,
///   then the filesystem object to be marked is determined by
///   interpreting pathname relative to the current working
///   directory.
/// 
/// # Example 
/// This example will panic because of [capabilities](https://man7.org/linux/man-pages/man7/capabilities.7.html) 
/// ```rust 
/// # #[should_panic]
/// # fn ex() {
///     # use naughtyfy::flags::*;
///     # use naughtyfy::structs::*;
///     # use naughtyfy::low_api::*;
///     let fd = fanotify_init(FAN_CLASS_NOTIF, 0).unwrap();
///     fanotify_mark(fd, FAN_MARK_ADD | FAN_MARK_MOUNT, FAN_ACCESS, libc::AT_FDCWD, "./");
/// # }
/// ``` 
pub fn fanotify_mark<P: ?Sized + Path>(
    fanotify_fd: i32,
    flags: u32,
    mask: u64,
    dirfd: i32,
    path: &P,
) -> Result<(), Error> {
    unsafe {
        match libc::fanotify_mark(
            fanotify_fd,
            flags,
            mask,
            dirfd,
            path.as_os_str()
                .as_bytes()
                .iter()
                .map(|p| *p as i8)
                .collect::<Vec<i8>>()
                .as_ptr(),
        ) {
            0 => Ok(()),
            _ => Err(Error::last_os_error()),
        }
    }
}

/// This function ateempts to read from a file descriptor `fanotify_fd`
/// into a `Vec<fanotify_event_metadata>` and return a Result.
pub fn fanotify_read(fanotify_fd: i32) -> Result<Vec<fanotify_event_metadata>, Error> {
    let mut vec = Vec::new();
    unsafe {
        let buffer = libc::malloc(*FAN_EVENT_METADATA_LEN * FAN_EVENT_BUFFER_LEN);
        if buffer == libc::PT_NULL as *mut c_void {
            return Err(Error::last_os_error());
        }
        let sizeof = libc::read(
            fanotify_fd,
            buffer,
            *FAN_EVENT_METADATA_LEN * FAN_EVENT_BUFFER_LEN,
        );
        if sizeof != libc::EAGAIN as isize && sizeof > 0 {
            let src = slice::from_raw_parts(
                buffer as *mut fanotify_event_metadata,
                sizeof as usize / *FAN_EVENT_METADATA_LEN,
            );
            vec = src.to_vec();
        }
        libc::free(buffer);
    }
    Ok(vec)
}
pub fn close_fd(fd: i32) {
    unsafe {
        libc::close(fd);
    }
}
