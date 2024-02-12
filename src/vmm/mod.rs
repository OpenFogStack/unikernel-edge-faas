use std::path::Path;

pub mod firecracker;
pub mod microvm;
pub mod qemu;
mod qemu_command;

#[derive(Debug)]
pub enum MachineError {
    InvalidConfig,
    VmmExitedEarly,
    VmmSpawnFailed,
}

pub enum Vmm {
    Firecracker(firecracker::Firecracker),
    Qemu(qemu::Qemu),
    Microvm(microvm::Microvm),
}

impl Vmm {
    pub fn spawn_firecracker(
        config: firecracker::Config,
        instance_dir: &Path,
    ) -> Result<Self, MachineError> {
        let fc = firecracker::Firecracker::spawn(config, instance_dir)?;

        Ok(Self::Firecracker(fc))
    }

    pub fn spawn_qemu(config: qemu::Config, instance_dir: &Path) -> Result<Self, MachineError> {
        let q = qemu::Qemu::spawn(config, instance_dir)?;

        Ok(Self::Qemu(q))
    }

    pub fn spawn_microvm(
        config: microvm::Config,
        instance_dir: &Path,
    ) -> Result<Self, MachineError> {
        let q = microvm::Microvm::spawn(config, instance_dir)?;

        Ok(Self::Microvm(q))
    }

    pub fn kill(self) -> Result<(), MachineError> {
        match self {
            Self::Firecracker(fc) => fc.kill(),
            Self::Qemu(qemu) => qemu.kill(),
            Self::Microvm(microvm) => microvm.kill(),
        }
    }
}
