use super::MachineError;
use serde_json::json;
use std::os::fd::{FromRawFd, IntoRawFd};
use std::path::PathBuf;
use std::{
    fs::File,
    io::Write,
    path::Path,
    process::{self, Stdio},
};
use tracing::{error, info};

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
    /// Create copy of rootfs for this machine
    pub copy_rootfs: bool,
    /// Commandline passed to kernel. Firecracker will append additional
    /// Linux specific arguments after this for virtio mmio devices and
    /// the disk containing the rootfs.
    pub cmdline: String,
    /// Name of the tap interface assigned to this machine.
    pub tap: String,
}

impl Config {
    fn serialize_drive(&self) -> serde_json::Value {
        let disk = self.disk.canonicalize().unwrap();
        json!({
            "drive_id": "rootfs",
            "path_on_host": disk,
            "is_root_device": true,
            "is_read_only": false
        })
    }

    fn serialize_netif(&self) -> serde_json::Value {
        json!({
            "iface_id": "eth0",
            "guest_mac": "AA:FC:00:00:00:01",
            "host_dev_name": self.tap
        })
    }

    fn serialize_machine(&self) -> serde_json::Value {
        json!({
            "vcpu_count": self.vcpus,
            "mem_size_mib": self.memory
        })
    }

    fn serialize_bootsource(&self) -> serde_json::Value {
        json!({
            "kernel_image_path": self.kernel,
            "boot_args": self.cmdline
        })
    }

    fn serialize_config(&self) -> serde_json::Value {
        json!({
            "machine-config": self.serialize_machine(),
            "boot-source": self.serialize_bootsource(),
            "drives": [ self.serialize_drive() ],
            "network-interfaces": [ self.serialize_netif() ]
        })
    }

    fn write_config_file(&self, path: &Path) -> Result<(), MachineError> {
        let config: String = serde_json::to_string_pretty(&self.serialize_config()).unwrap();
        let mut file = File::create(path).map_err(|_| MachineError::InvalidConfig)?;

        file.write_all(config.as_bytes())
            .map_err(|_| MachineError::InvalidConfig)
    }
}

#[derive(Debug)]
pub struct Firecracker {
    /// Handle to the vmm process executing this machine.
    vmm: process::Child,
    disk: Option<PathBuf>,
}

impl Firecracker {
    fn firecracker_no_api(config_arg: &str, log: File) -> process::Command {
        let firecracker = if let Ok(f) = std::env::var("FIRECRACKER_EXE") {
            f
        } else {
            "firecracker".to_string()
        };

        let mut cmd = process::Command::new(firecracker);

        cmd.args(["--no-api", "--config-file", config_arg]);

        let log_fd = log.into_raw_fd();
        cmd.stderr(unsafe { Stdio::from_raw_fd(log_fd) });
        cmd.stdout(unsafe { Stdio::from_raw_fd(log_fd) });
        cmd.stdin(Stdio::null());

        return cmd;
    }

    pub fn spawn(mut config: Config, tmp_path: &Path) -> Result<Self, MachineError> {
        let disk = if config.copy_rootfs {
            let img_path = tmp_path.join("rootfs.img");
            info!("Copying rootfs to {:?}", img_path);
            std::fs::copy(&config.disk, &img_path).unwrap();

            config.disk = img_path;
            Some(config.disk.clone())
        } else {
            None
        };

        let config_file_path = tmp_path.join("config.json");
        config.write_config_file(&config_file_path)?;

        let log_file_path = tmp_path.join("log.txt");
        let log_file = File::create(log_file_path).unwrap();

        let config_arg = config_file_path.to_str().unwrap();
        let vmm = Firecracker::firecracker_no_api(config_arg, log_file)
            .spawn()
            .map_err(|_| MachineError::VmmSpawnFailed)?;

        Ok(Self { vmm, disk })
    }

    pub fn kill(mut self) -> Result<(), MachineError> {
        let res = self.vmm.kill().map_err(|_| MachineError::VmmExitedEarly);
        match self.vmm.wait() {
            Ok(_) => (),
            Err(_) => error!("Failed to wait!"),
        }

        if let Some(disk) = self.disk {
            std::fs::remove_file(disk).unwrap();
        }

        return res;
    }
}
