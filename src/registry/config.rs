use std::path::PathBuf;

fn serde_true() -> bool {
    true
}
fn serde_false() -> bool {
    false
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub enum DiskType {
    #[serde(rename = "9p")]
    Virtio9p,
    #[serde(rename = "block")]
    VirtioBlock,
}

impl DiskType {
    pub fn is_9p(&self) -> bool {
        match self {
            Self::Virtio9p => true,
            Self::VirtioBlock => false,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[allow(dead_code)]
pub struct MicrovmConfig {
    #[serde(rename = "disk-type")]
    pub disk_type: DiskType,
    #[serde(default = "serde_false")]
    pub pic: bool,
    #[serde(default = "serde_false")]
    pub pit: bool,
    #[serde(default = "serde_false")]
    pub rtc: bool,
    #[serde(rename = "auto-kernel-cmdline")]
    #[serde(default = "serde_true")]
    pub auto_kernel_cmdline: bool,
    #[serde(default = "serde_false")]
    pub acpi: bool,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[allow(dead_code)]
pub struct QemuConfig {
    #[serde(rename = "disk-type")]
    pub disk_type: DiskType,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct FirecrackerConfig {
    #[serde(rename = "copy-rootfs")]
    #[serde(default = "serde_true")]
    pub copy_rootfs: bool,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum VmmConfig {
    Microvm(MicrovmConfig),
    Qemu(QemuConfig),
    Firecracker(FirecrackerConfig),
}

fn default_memory_size() -> u32 {
    128
}

fn default_num_cpus() -> u32 {
    1
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct VmConfig {
    pub cmdline: String,
    pub kernel: PathBuf,
    pub image: PathBuf,
    #[serde(default = "default_memory_size")]
    pub memory: u32,
    #[serde(default = "default_num_cpus")]
    pub cpus: u32,
    pub hypervisor: VmmConfig,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ContainerConfig {
    pub image: String,
    #[serde(default = "default_memory_size")]
    pub memory: u32,
    #[serde(default)]
    pub runtime: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum RuntimeConfig {
    Vm(VmConfig),
    Container(ContainerConfig),
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[allow(dead_code)]
pub struct FunctionConfig {
    pub keepalive: u64,
    #[serde(rename = "concurrent-requests")]
    pub concurrent_requests: usize,
    #[serde(rename = "single-use")]
    #[serde(default = "serde_false")]
    pub single_use: bool,
    #[serde(flatten)]
    pub runtime: RuntimeConfig,
}

impl FunctionConfig {
    pub fn from_string(string: &str) -> Result<FunctionConfig, String> {
        let conf = toml::from_str::<FunctionConfig>(string).map_err(|e| e.message().to_owned())?;

        if conf.concurrent_requests < 1 {
            return Err("concurrent-requests < 1".to_owned());
        }

        Ok(conf)
    }

    pub fn needs_netif(&self) -> bool {
        if let RuntimeConfig::Vm(_) = self.runtime {
            return true;
        } else {
            return false;
        }
    }
}
