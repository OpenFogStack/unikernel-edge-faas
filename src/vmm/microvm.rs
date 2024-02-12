use super::qemu_command::QemuCommand;
use super::MachineError;
use std::{
    fs::File,
    os::fd::{FromRawFd, IntoRawFd},
    path::{Path, PathBuf},
    process::{self, Stdio},
};
use tracing::error;

#[derive(Debug)]
pub struct Config {
    /// Number of cpus allocated to the vm.
    pub vcpus: u32,
    /// Amount of ram in MiB allocated to the vm.
    pub memory: u32,
    /// Path to kernel image for direct kernel boot.
    pub kernel: PathBuf,
    /// Path to rootfs disk image.
    pub disk: PathBuf,
    /// Wether disk should be passed as virtio 9p host share or as a
    /// normal virtio block device
    pub disk_is_9p: bool,
    /// Commandline passed to kernel. Qemu will append additional
    /// Linux specific arguments after this for virtio mmio devices and
    /// the disk containing the rootfs depending on wheter acpi is enabled
    /// and the value of `auto_kernel_cmdline`
    pub cmdline: String,
    /// Name of the tap interface assigned to this machine.
    pub tap: String,
    /// Enable legagcy x86 interrupt controller
    pub pic_enable: bool,
    /// Enable legacy x86 programmable interval timer
    pub pit_enable: bool,
    /// Enable legacy x86 real time clock
    pub rtc_enable: bool,
    /// Pass additional arguments such as mmio device configuration
    /// via kernel command line.
    pub auto_kernel_cmdline: bool,
    /// Enable acpi
    pub acpi_enable: bool,
}

pub struct Microvm {
    vmm: process::Child,
}

impl Microvm {
    pub fn spawn(config: Config, instance_path: &Path) -> Result<Self, MachineError> {
        let log_file_path = instance_path.join("log.txt");
        let log_file = File::create(log_file_path).unwrap();
        let log_fd = log_file.into_raw_fd();

        let mut cmd = command_from_config(&config);
        cmd.stderr(unsafe { Stdio::from_raw_fd(log_fd) });
        cmd.stdout(unsafe { Stdio::from_raw_fd(log_fd) });
        cmd.stdin(Stdio::null());

        let vmm = cmd.spawn().map_err(|_| MachineError::VmmSpawnFailed)?;

        Ok(Microvm { vmm })
    }

    pub fn kill(mut self) -> Result<(), MachineError> {
        let res = self.vmm.kill().map_err(|_| MachineError::VmmExitedEarly);
        match self.vmm.wait() {
            Ok(_) => (),
            Err(_) => error!("Failed to wait!"),
        }

        return res;
    }
}

fn command_from_config(config: &Config) -> process::Command {
    let is_microvm = true;
    let mut q = QemuCommand::new(is_microvm)
        .with_option_val("-M", "microvm")
        .with_kvm()
        .with_option_val("-serial", "stdio")
        .with_option_val("-display", "none")
        .with_option("-no-user-config")
        .with_option("-nographic")
        .with_option("-nodefaults")
        .with_option("-no-reboot")
        .with_option_val("-smp", &config.vcpus.to_string())
        .with_mem(config.memory)
        .with_kernel(config.kernel.to_str().unwrap())
        .with_cmdline(&config.cmdline)
        .with_virtio_net(&config.tap);

    q = if config.disk_is_9p {
        q.with_virtio_9p(config.disk.to_str().unwrap(), "fs0")
    } else {
        q.with_virtio_blk(config.disk.to_str().unwrap())
    };

    if config.pic_enable {
        q = q.with_option_val("-M", "pic=on");
    }

    if config.pit_enable {
        q = q.with_option_val("-M", "pit=on");
    }

    if config.rtc_enable {
        q = q.with_option_val("-M", "rtc=on");
    }

    if config.acpi_enable {
        q = q.with_option_val("-M", "acpi=on");
    }

    if config.auto_kernel_cmdline {
        q = q.with_option_val("-M", "auto-kernel-cmdline=on");
    }

    q.build()
}
