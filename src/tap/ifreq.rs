use ::nix::{ioctl_read_bad, ioctl_write_int, ioctl_write_ptr_bad};
use libc::{c_char, IFNAMSIZ};
use std::ffi::CStr;
use std::{mem, ptr, str};

ioctl_write_ptr_bad!(siocsifflags, libc::SIOCSIFFLAGS, libc::ifreq);
ioctl_write_ptr_bad!(siocsifaddr, libc::SIOCSIFADDR, libc::ifreq);
ioctl_write_ptr_bad!(siocsifnetmask, libc::SIOCSIFNETMASK, libc::ifreq);
ioctl_write_int!(tunsetiff, b'T', 202);
ioctl_write_int!(tunsetpersist, b'T', 203);
ioctl_read_bad!(siocgifmtu, libc::SIOCGIFMTU, libc::ifreq);
ioctl_read_bad!(siocgifflags, libc::SIOCGIFFLAGS, libc::ifreq);
ioctl_read_bad!(siocgifaddr, libc::SIOCGIFADDR, libc::ifreq);
ioctl_read_bad!(siocgifdstaddr, libc::SIOCGIFDSTADDR, libc::ifreq);
ioctl_read_bad!(siocgifbrdaddr, libc::SIOCGIFBRDADDR, libc::ifreq);
ioctl_read_bad!(siocgifnetmask, libc::SIOCGIFNETMASK, libc::ifreq);

pub struct Ifreq {
    pub ifreq: libc::ifreq,
}

impl Ifreq {
    pub fn zeroed() -> Self {
        let ifreq: libc::ifreq = unsafe { mem::zeroed() };

        Ifreq { ifreq }
    }

    pub fn with_name(name: &str) -> Self {
        let mut req = Self::zeroed();

        unsafe {
            ptr::copy_nonoverlapping(
                name.as_ptr().cast::<c_char>(),
                req.ifreq.ifr_name.as_mut_ptr(),
                name.len().min(IFNAMSIZ),
            )
        }

        req
    }

    pub fn name_as_string(&self) -> String {
        let name = unsafe {
            str::from_utf8_unchecked(CStr::from_ptr(self.ifreq.ifr_name.as_ptr()).to_bytes())
        };

        name.to_owned()
    }
}
