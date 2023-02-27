use std::any::type_name;
use std::fmt;

pub struct Init;
pub struct Mark;
pub struct Read;
pub struct Write;
pub struct Close;

/// Matches proper description with errno recieved after calling
/// fanotify functions
///
fn code_desc<T>(code: i32) -> String {
    match type_name::<T>() {
        "naughtyfy::errors::Init" => match code {
            libc::EINVAL => "An invalid value was passed in flags or event_f_flags. \
                    FAN_ALL_INIT_FLAGS (deprecated since Linux kernel version \
                    4.20) defines all allowable bits for flags."
                .to_string(),

            libc::EMFILE => "The number of fanotify groups for this user exceeds 128 or 
                    The per-process limit on the number of open file
                    descriptors has been reached."
                .to_string(),

            libc::ENOMEM => "The allocation of memory for the notification group
                    failed."
                .to_string(),

            libc::ENOSYS => "This kernel does not implement fanotify_init().  The
                    fanotify API is available only if the kernel was
                    configured with CONFIG_FANOTIFY."
                .to_string(),

            libc::EPERM => "The operation is not permitted because the caller lacks
                    the CAP_SYS_ADMIN capability."
                .to_string(),

            _ => "Unknown error occured".to_string(),
        },
        "naughtyfy::errors::Mark" => match code {
            libc::EBADF => "An invalid file descriptor was passed in fanotify_fd or 
                pathname is relative but dirfd is neither AT_FDCWD nor a
                valid file descriptor."
                .to_string(),
            libc::EINVAL => "An invalid value was passed in flags or mask, or
                fanotify_fd was not an fanotify file descriptor
                or the fanotify file descriptor was opened with
                FAN_CLASS_NOTIF or the fanotify group identifies
                filesystem objects by file handles and mask contains a
                flag for permission events (FAN_OPEN_PERM or
                FAN_ACCESS_PERM)."
                .to_string(),
            libc::ENODEV => "The filesystem object indicated by pathname is not
                associated with a filesystem that supports fsid (e.g.,
                tmpfs(5)).  This error can be returned only with an
                fanotify group that identifies filesystem objects by file
                handles."
                .to_string(),
            libc::ENOENT => "The filesystem object indicated by dirfd and pathname does
                not exist.  This error also occurs when trying to remove a
                mark from an object which is not marked."
                .to_string(),
            libc::ENOMEM => "The necessary memory could not be allocated.".to_string(),
            libc::ENOSPC => "The number of marks exceeds the limit of 8192 and the
                FAN_UNLIMITED_MARKS flag was not specified when the
                fanotify file descriptor was created with
                fanotify_init()."
                .to_string(),
            libc::ENOSYS => "This kernel does not implement fanotify_mark().  The
                fanotify API is available only if the kernel was
                configured with CONFIG_FANOTIFY."
                .to_string(),
            libc::ENOTDIR => "flags contains FAN_MARK_ONLYDIR, and dirfd and pathname do
                not specify a directory."
                .to_string(),
            libc::EOPNOTSUPP => "The object indicated by pathname is associated with a
                filesystem that does not support the encoding of file
                handles.  This error can be returned only with an fanotify
                group that identifies filesystem objects by file handles."
                .to_string(),
            libc::EXDEV => "The filesystem object indicated by pathname resides within
                a filesystem subvolume (e.g., btrfs(5)) which uses a
                different fsid than its root superblock.  This error can
                be returned only with an fanotify group that identifies
                filesystem objects by file handles."
                .to_string(),
            _ => "Unnown error occured.".to_string(),
        },
        "naughtyfy::errors::Read" => match code {
            libc::EAGAIN => "The file descriptor fd refers to a file other than a
                socket and has been marked nonblocking (O_NONBLOCK), and
                the read would block.  See open() for further details on
                the O_NONBLOCK flag or the file descriptor fd refers to a 
                socket and has been marked nonblocking (O_NONBLOCK), and 
                the read would block. POSIX.1-2001 allows either error to 
                be returned for this case, and does not require these 
                constants to have the same value, so a portable application 
                should check for both possibilities."
                .to_string(),
            libc::EBADF => "fd is not a valid file descriptor or is not open for
                writing."
                .to_string(),
            libc::EDESTADDRREQ => "fd refers to a datagram socket for which a peer address
                has not been set using connect(2)."
                .to_string(),
            libc::EDQUOT => "The user's quota of disk blocks on the filesystem
                containing the file referred to by fd has been exhausted."
                .to_string(),
            libc::EFAULT => "buf is outside your accessible address space.".to_string(),
            libc::EINTR => "The call was interrupted by a signal before any data was
                read"
                .to_string(),
            libc::EINVAL => "fd is attached to an object which is unsuitable for
                reading; or the file was opened with the O_DIRECT flag,
                and either the address specified in buf, the value
                specified in count, or the file offset is not suitably
                aligned or fd was created via a call to timerfd_create(2) and the
                wrong size buffer was given to fanotify_read()"
                .to_string(),
            libc::EIO => "I/O error.  This will happen for example when the process
                is in a background process group, tries to read from its
                controlling terminal, and either it is ignoring or
                blocking SIGTTIN or its process group is orphaned.  It may
                also occur when there is a low-level I/O error while
                reading from a disk or tape.  A further possible cause of
                EIO on networked filesystems is when an advisory lock had
                been taken out on the file descriptor and this lock has
                been lost."
                .to_string(),
            libc::EISDIR => "fd refers to a directory.".to_string(),
            libc::ENOMEM => "Cannot allocate memory to read buffer".to_string(),
            _ => "Unnown error occured.".to_string(),
        },
        "naughtyfy::errors::Write" => match code {
            libc::EAGAIN => "The file descriptor fd refers to a file other than a
                socket and has been marked nonblocking (O_NONBLOCK), and
                the read would block.  See open() for further details on
                the O_NONBLOCK flag or the file descriptor fd refers to a 
                socket and has been marked nonblocking (O_NONBLOCK), and 
                the read would block. POSIX.1-2001 allows either error to 
                be returned for this case, and does not require these 
                constants to have the same value, so a portable application 
                should check for both possibilities."
                .to_string(),
            libc::EBADF => "fd is not a valid file descriptor or is not open for
                reading."
                .to_string(),
            libc::EDESTADDRREQ => "fd refers to a datagram socket for which a peer address
                has not been set using connect(2)."
                .to_string(),
            libc::EDQUOT => "The user's quota of disk blocks on the filesystem
                containing the file referred to by fd has been exhausted."
                .to_string(),
            libc::EFAULT => "buf is outside your accessible address space.".to_string(),
            libc::EFBIG => "An attempt was made to write a file that exceeds the
                implementation-defined maximum file size or the process's
                file size limit, or to write at a position past the
                maximum allowed offset."
                .to_string(),
            libc::EINTR => "The call was interrupted by a signal before any data was
                written"
                .to_string(),
            libc::EINVAL => "fd is attached to an object which is unsuitable for
                writing; or the file was opened with the O_DIRECT flag,
                and either the address specified in buf, the value
                specified in count, or the file offset is not suitably
                aligned."
                .to_string(),
            libc::EIO => "A low-level I/O error occurred while modifying the inode.
                This error may relate to the write-back of data written by
                an earlier write(), which may have been issued to a
                different file descriptor on the same file.  Since Linux
                4.13, errors from write-back come with a promise that they
                may be reported by subsequent.  write() requests, and will
                be reported by a subsequent fsync(2) (whether or not they
                were also reported by write()).  An alternate cause of EIO
                on networked filesystems is when an advisory lock had been
                taken out on the file descriptor and this lock has been
                lost."
                .to_string(),
            libc::ENOSPC => "The device containing the file referred to by fd has no
                room for the data."
                .to_string(),
            libc::EPERM => "The operation was prevented by a file seal".to_string(),
            libc::EPIPE => "fd is connected to a pipe or socket whose reading end is
                closed.  When this happens the writing process will also
                receive a SIGPIPE signal.  (Thus, the write return value
                is seen only if the program catches, blocks or ignores
                this signal.)"
                .to_string(),
            _ => "Unnown error occured.".to_string(),
        },
        "naughtyfy::errors::Close" => match code {
            libc::EBADF => "fd isn't a valid open file descriptor.".to_string(),
            libc::EINTR => "The close() call was interrupted by a signal".to_string(),
            libc::EIO => "An I/O error occurred.".to_string(),
            libc::ENOSPC | libc::EDQUOT => {
                "On NFS, these errors are not normally reported against the
                first write which exceeds the available storage space, but
                instead against a subsequent write(2), fsync(2), or
                close()."
                    .to_string()
            }
            _ => "Unnown error occured.".to_string(),
        },
        _ => "Unnown error occured.".to_string(),
    }
}

pub struct FanotifyError<Type = Init> {
    code: i32,
    oserr: String,
    desc: String,
    ty: std::marker::PhantomData<Type>,
}

impl std::error::Error for FanotifyError {}

impl fmt::Display for FanotifyError<Init> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", code_desc::<Init>(self.code))
    }
}

impl fmt::Debug for FanotifyError<Init> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FanotifyInitError:
                OS error: {}
                description: {}",
            self.oserr, self.desc
        )
    }
}

impl fmt::Display for FanotifyError<Mark> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {} Description: {}", self.oserr, self.desc)
    }
}

impl fmt::Debug for FanotifyError<Mark> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FanotifyMarkError:
                OS error: {}
                description: {}",
            self.oserr, self.desc
        )
    }
}

impl fmt::Display for FanotifyError<Read> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {} Description: {}", self.oserr, self.desc)
    }
}
impl fmt::Debug for FanotifyError<Read> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FanotifyReadError:
                OS error: {}
                description: {}",
            self.oserr, self.desc
        )
    }
}

impl fmt::Display for FanotifyError<Write> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {} Description: {}", self.oserr, self.desc)
    }
}
impl fmt::Debug for FanotifyError<Write> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FanotifyWriteError:
                OS error: {}
                description: {}",
            self.oserr, self.desc
        )
    }
}

impl fmt::Display for FanotifyError<Close> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {} Description: {}", self.oserr, self.desc)
    }
}
impl fmt::Debug for FanotifyError<Close> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FanotifyCloseError:
                OS error: {}
                description: {}",
            self.oserr, self.desc
        )
    }
}

impl<T> From<std::io::Error> for FanotifyError<T> {
    fn from(error: std::io::Error) -> Self {
        FanotifyError {
            code: error.raw_os_error().unwrap(),
            oserr: error.to_string(),
            desc: code_desc::<T>(error.raw_os_error().unwrap()),
            ty: std::marker::PhantomData::<T>,
        }
    }
}
