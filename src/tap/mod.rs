mod ifreq;
mod sockaddr;

use ifreq::Ifreq;
use libc::IFF_TAP;
use sockaddr::SockaddrConvertible;
use std::{
    net::Ipv4Addr,
    os::raw::{c_char, c_short},
    str,
};
use tracing::error;

pub struct TapRaw {
    ifname: String,
    raw_fd: i32,
    raw_socket: i32,
}

#[derive(thiserror::Error, Debug)]
pub enum TapError {
    #[error("{0}")]
    NixError(#[from] nix::Error),
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    #[error("{0}")]
    RawError(String),
}

impl<T> Into<Result<T, TapError>> for TapError {
    fn into(self) -> Result<T, TapError> {
        Err(self)
    }
}

#[allow(dead_code)]
impl TapRaw {
    fn open_tundev_raw() -> i32 {
        let tundev = b"/dev/net/tun\0";
        unsafe {
            libc::open(
                tundev.as_ptr().cast::<c_char>(),
                libc::O_RDWR | libc::O_NONBLOCK,
            )
        }
    }

    pub fn new(name: &str) -> Result<Self, TapError> {
        let fd = Self::open_tundev_raw();

        if fd < 0 {
            return TapError::RawError(format!("Could not open /dev/net/tun ({})", fd)).into();
        }

        let mut req = Ifreq::with_name(&name);
        req.ifreq.ifr_ifru.ifru_flags = IFF_TAP as c_short;

        unsafe { ifreq::tunsetiff(fd, &req.ifreq as *const _ as _) }?;
        let ifname = req.name_as_string();

        let socket = unsafe { libc::socket(libc::AF_INET, libc::SOCK_DGRAM, 0) };

        if socket < 0 {
            return TapError::RawError(format!("Could not create socket ({})", socket)).into();
        }

        Ok(TapRaw {
            ifname,
            raw_fd: fd,
            raw_socket: socket,
        })
    }

    pub fn set_persistent(&self, persist: bool) -> Result<(), TapError> {
        let val = if persist { 1 } else { 0 };
        unsafe { ifreq::tunsetpersist(self.raw_fd, val) }?;
        Ok(())
    }

    pub fn set_address(&self, address: &Ipv4Addr) -> Result<(), TapError> {
        let mut req = Ifreq::with_name(&self.ifname);
        req.ifreq.ifr_ifru.ifru_addr = address.to_sockaddr();
        unsafe { ifreq::siocsifaddr(self.raw_socket, &req.ifreq) }?;
        Ok(())
    }

    pub fn get_address(&self) -> Result<Ipv4Addr, TapError> {
        let mut req = Ifreq::with_name(&self.ifname);
        unsafe { ifreq::siocgifaddr(self.raw_socket, &mut req.ifreq) }?;
        Ok(Ipv4Addr::from_sockaddr(unsafe {
            req.ifreq.ifr_ifru.ifru_addr
        }))
    }

    pub fn set_netmask(&self, address: &Ipv4Addr) -> Result<(), TapError> {
        let mut req = Ifreq::with_name(&self.ifname);
        req.ifreq.ifr_ifru.ifru_addr = address.to_sockaddr();
        unsafe { ifreq::siocsifnetmask(self.raw_socket, &req.ifreq) }?;
        Ok(())
    }

    pub fn get_netmask(&self) -> Result<Ipv4Addr, TapError> {
        let mut req = Ifreq::with_name(&self.ifname);
        unsafe { ifreq::siocgifnetmask(self.raw_socket, &mut req.ifreq) }?;
        Ok(Ipv4Addr::from_sockaddr(unsafe {
            req.ifreq.ifr_ifru.ifru_addr
        }))
    }

    pub fn set_flags(&self, flags: i16) -> Result<(), TapError> {
        let mut req = Ifreq::with_name(&self.ifname);
        req.ifreq.ifr_ifru.ifru_flags = flags;
        unsafe { ifreq::siocsifflags(self.raw_socket, &req.ifreq) }?;
        Ok(())
    }

    pub fn get_flags(&self) -> Result<i16, TapError> {
        let mut req = Ifreq::with_name(&self.ifname);
        unsafe { ifreq::siocgifflags(self.raw_socket, &mut req.ifreq) }?;
        unsafe { Ok(req.ifreq.ifr_ifru.ifru_flags) }
    }

    pub fn set_ifup(&self) -> Result<(), TapError> {
        let flags = libc::IFF_UP as i16 | libc::IFF_RUNNING as i16;
        self.set_flags(flags)
    }

    pub fn set_ifdown(&self) -> Result<(), TapError> {
        let mut flags = self.get_flags()?;
        let upflags = libc::IFF_UP as i16 | libc::IFF_RUNNING as i16;
        flags &= !upflags;
        self.set_flags(flags)
    }

    pub fn close(self) {
        unsafe {
            libc::close(self.raw_fd);
            libc::close(self.raw_socket);
        }
    }
}

pub struct Tap {
    ifname: String,
}

impl Tap {
    pub fn create(name: &str, ip: &Ipv4Addr, netmask: &Ipv4Addr) -> Result<Self, TapError> {
        let raw = TapRaw::new(name).unwrap();
        raw.set_address(ip)?;
        raw.set_netmask(netmask)?;
        raw.set_persistent(true)?;
        raw.set_ifup()?;
        let ifname = raw.ifname.to_owned();
        raw.close();

        Ok(Tap { ifname })
    }

    pub fn remove(&self) -> Result<(), TapError> {
        let raw = TapRaw::new(&self.ifname)?;
        if let Err(e) = raw.set_ifdown() {
            error!(
                "Failed to set interface {} down during removal: {}",
                self.ifname, e
            );
            return Err(e);
        }
        if let Err(e) = raw.set_persistent(false) {
            error!(
                "Faile to set interface {} non-persistent during removal: {}",
                self.ifname, e
            );
            return Err(e);
        }
        raw.close();
        Ok(())
    }

    pub fn name(&self) -> &str {
        &self.ifname
    }
}
