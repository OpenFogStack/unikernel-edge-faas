use crate::{
    docker::{self, Container},
    network::NetLease,
    registry::{ContainerConfig, FunctionConfig, VmConfig, VmmConfig},
    vmm::{firecracker, microvm, qemu, MachineError, Vmm},
};
use hyper::{client::HttpConnector, Client};
use std::{net::Ipv4Addr, path::PathBuf};
use tracing::{error, info};

pub enum InstanceRunner {
    Vmm(Vmm, NetLease),
    Container(Container),
}

#[allow(dead_code)]
pub struct Instance {
    id: String,
    machine: InstanceRunner,
    client: Client<HttpConnector>,
}

fn make_tmpdir(id: &str) -> PathBuf {
    let tmpdir = std::env::temp_dir();

    let dir = tmpdir.join(PathBuf::from(format!("faas/{}", id)));
    std::fs::create_dir_all(dir.as_path()).unwrap();

    return dir;
}

fn format_cmdline(
    cmdline: &str,
    id: &str,
    ip: &Ipv4Addr,
    gw: &Ipv4Addr,
    mask: &Ipv4Addr,
) -> String {
    let callback = format!("http://{}:3000/ready/{}", gw, id);
    let patched = cmdline
        .replace("%ip", &ip.to_string())
        .replace("%gateway", &gw.to_string())
        .replace("%netmask", &mask.to_string())
        .replace("%callback", &callback);

    patched
}

impl Instance {
    pub async fn new(
        id: &str,
        function: &FunctionConfig,
        lease: Option<NetLease>,
    ) -> Result<Self, String> {
        match &function.runtime {
            crate::registry::RuntimeConfig::Vm(vm) => Self::start_vm(id, vm, lease.unwrap()).await,
            crate::registry::RuntimeConfig::Container(container) => {
                Self::start_container(id, container).await
            }
        }
    }

    async fn start_container(id: &str, container: &ContainerConfig) -> Result<Self, String> {
        let container = docker::Container::run(id, container).await?;
        Ok(Instance {
            id: id.to_owned(),
            machine: InstanceRunner::Container(container),
            client: Client::new(),
        })
    }

    async fn start_vm(id: &str, vm: &VmConfig, lease: NetLease) -> Result<Self, String> {
        let id = id.to_owned();
        let instance_dir = make_tmpdir(&id);

        let cmdline = format_cmdline(
            &vm.cmdline,
            &id,
            &lease.guest_addr(),
            &lease.host_addr(),
            &lease.netmask(),
        );

        let machine = match &vm.hypervisor {
            VmmConfig::Microvm(config) => {
                let mc = microvm::Config {
                    vcpus: vm.cpus,
                    memory: vm.memory,
                    kernel: vm.kernel.to_path_buf(),
                    disk: vm.image.to_path_buf(),
                    disk_is_9p: config.disk_type.is_9p(),
                    cmdline,
                    tap: lease.netif.name().to_owned(),
                    pic_enable: config.pic,
                    pit_enable: config.pit,
                    rtc_enable: config.rtc,
                    auto_kernel_cmdline: config.auto_kernel_cmdline,
                    acpi_enable: config.acpi,
                };

                Vmm::spawn_microvm(mc, &instance_dir).map_err(|e| format!("error: {:?}", e))?
            }
            VmmConfig::Qemu(config) => {
                let qc = qemu::Config {
                    vcpus: vm.cpus,
                    memory: vm.memory,
                    kernel: vm.kernel.to_path_buf(),
                    disk: vm.image.to_path_buf(),
                    disk_is_9p: config.disk_type.is_9p(),
                    cmdline,
                    tap: lease.netif.name().to_owned(),
                };

                Vmm::spawn_qemu(qc, &instance_dir).map_err(|e| format!("error: {:?}", e))?
            }
            VmmConfig::Firecracker(config) => {
                let fc = firecracker::Config {
                    vcpus: vm.cpus,
                    memory: vm.memory,
                    kernel: vm.kernel.to_path_buf(),
                    disk: vm.image.to_path_buf(),
                    copy_rootfs: config.copy_rootfs,
                    cmdline,
                    tap: lease.netif.name().to_owned(),
                };

                Vmm::spawn_firecracker(fc, &instance_dir).map_err(|e| format!("error: {:?}", e))?
            }
        };

        info!(
            "Starting instance {} (tap={}, ip={})",
            id,
            lease.netif.name(),
            lease.guest_addr()
        );

        Ok(Instance {
            id,
            machine: InstanceRunner::Vmm(machine, lease),
            client: Client::new(),
        })
    }

    pub fn ip(&self) -> Ipv4Addr {
        match &self.machine {
            InstanceRunner::Vmm(_, lease) => lease.guest_addr().to_owned(),
            InstanceRunner::Container(container) => container.ip(),
        }
    }

    pub fn id(&self) -> String {
        self.id.to_owned()
    }

    pub fn client(&self) -> Client<HttpConnector> {
        self.client.clone()
    }

    pub async fn kill(self) {
        info!("Stopping instance {}", self.id);

        match self.machine {
            InstanceRunner::Vmm(vmm, lease) => {
                match vmm.kill() {
                    Ok(_) => (),
                    Err(MachineError::VmmExitedEarly) => error!(
                        "Failed to cleanly stop vmm for instance {}. Process already exited.",
                        self.id
                    ),
                    Err(e) => {
                        error!(
                            "Failed to stop vmm for instance {}. Not releasing network lease: {:?}",
                            self.id, e
                        );
                        return;
                    }
                }

                lease.release();
            }
            InstanceRunner::Container(container) => {
                if let Err(e) = container.kill().await {
                    error!("Failed to stop container {}: {}", self.id, e);
                }
            }
        }
    }
}
