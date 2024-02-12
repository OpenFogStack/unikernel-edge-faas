use std::{
    format,
    process::{Command, Stdio},
};

const QEMU: &str = "qemu-system-x86_64";

pub struct QemuCommand {
    is_microvm: bool,
    next_id: u32,
    args: Vec<String>,
}

#[allow(dead_code)]
impl QemuCommand {
    pub fn new(microvm: bool) -> Self {
        Self {
            is_microvm: microvm,
            next_id: 0,
            args: vec![],
        }
    }

    pub fn build(self) -> Command {
        let mut cmd = Command::new(QEMU);
        cmd.stdin(Stdio::null());
        cmd.args(self.args);

        return cmd;
    }

    fn new_device_id(&mut self) -> String {
        let n = self.next_id;
        self.next_id += 1;
        format!("dev{}", n)
    }

    pub fn with_virtio_blk(mut self, path: &str) -> Self {
        let id = self.new_device_id();

        let name = if self.is_microvm {
            "virtio-blk-device"
        } else {
            "virtio-blk-pci"
        };

        let drv_arg = format!("if=none,id={},format=raw,file={}", id, path);
        let dev_arg = format!("{},drive={}", name, id);

        let mut args = vec!["-drive".to_owned(), drv_arg, "-device".to_owned(), dev_arg];
        self.args.append(&mut args);

        return self;
    }

    pub fn with_virtio_net(mut self, tap: &str) -> Self {
        let id = self.new_device_id();

        let name = if self.is_microvm {
            "virtio-net-device"
        } else {
            "virtio-net-pci"
        };

        let net_arg = format!("tap,id={},ifname={},script=no,downscript=no", id, tap);
        let dev_arg = format!("{},netdev={}", name, id);

        let mut args = vec!["-netdev".to_owned(), net_arg, "-device".to_owned(), dev_arg];
        self.args.append(&mut args);

        return self;
    }

    pub fn with_virtio_scsi(mut self, path: &str) -> Self {
        let id = self.new_device_id();

        let name = if self.is_microvm {
            "virtio-scsi-device"
        } else {
            "virtio-scsi-pci"
        };

        let drv_arg = format!("if=none,id={},format=raw,file={}", id, path);
        let bus_arg = format!("{},id={}", name, self.new_device_id());
        let dev_arg = format!("scsi-hd,drive={}", id);

        self.with_option_val("-drive", &drv_arg)
            .with_option_val("-device", &bus_arg)
            .with_option_val("-device", &dev_arg)
    }

    pub fn with_virtio_9p(mut self, path: &str, mount_tag: &str) -> Self {
        let id = self.new_device_id();

        let name = if self.is_microvm {
            "virtio-9p-device"
        } else {
            "virtio-9p-pci"
        };

        let drv_arg = format!("local,security_model=passthrough,id={},path={}", id, path);
        let dev_arg = format!("{},fsdev={},mount_tag={}", name, id, mount_tag);

        self.with_option_val("-fsdev", &drv_arg)
            .with_option_val("-device", &dev_arg)
    }

    pub fn with_kvm(mut self) -> Self {
        self.args.append(&mut vec![String::from("-enable-kvm")]);

        return self;
    }

    pub fn with_mem(self, mb: u32) -> Self {
        self.with_option_val("-m", &mb.to_string())
    }

    pub fn with_raw_args(mut self, mut args: Vec<String>) -> Self {
        self.args.append(args.as_mut());
        return self;
    }

    pub fn with_option_val(self, key: &str, value: &str) -> Self {
        let args = vec![key.to_owned(), value.to_owned()];
        self.with_raw_args(args)
    }

    pub fn with_option(self, key: &str) -> Self {
        let args = vec![key.to_owned()];
        self.with_raw_args(args)
    }

    pub fn with_cmdline(self, cmdline: &str) -> Self {
        self.with_option_val("-append", cmdline)
    }

    pub fn with_kernel(self, path: &str) -> Self {
        self.with_option_val("-kernel", path)
    }
}
