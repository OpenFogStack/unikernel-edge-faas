use bollard::{
    container::{
        Config, CreateContainerOptions, KillContainerOptions, RemoveContainerOptions,
        StartContainerOptions, UpdateContainerOptions,
    },
    service::HostConfig,
    Docker,
};
use std::net::Ipv4Addr;
use tracing::info;

use crate::registry::ContainerConfig;

pub struct Container {
    name: String,
    ip: Ipv4Addr,
}

impl Container {
    pub async fn run(id: &str, container: &ContainerConfig) -> Result<Self, String> {
        info!(
            "Starting instance (docker, runtime = {:?}) {}",
            container.runtime, id
        );
        let docker = Docker::connect_with_local_defaults().map_err(|e| e.to_string())?;

        let options = Some(CreateContainerOptions {
            name: id.clone(),
            platform: None,
        });

        let hc = if let Some(rt) = &container.runtime {
            Some(HostConfig {
                runtime: Some(rt.to_string()),
                ..Default::default()
            })
        } else {
            None
        };

        let callback = format!("http://172.17.0.1:3000/ready/{}", id);
        let config = Config {
            image: Some(container.image.clone()),
            cmd: Some(vec![callback]),
            host_config: hc,
            ..Default::default()
        };

        docker
            .create_container(options, config)
            .await
            .map_err(|e| e.to_string())?;

        let mem = i64::from(container.memory) * 1024 * 1024;
        let options = UpdateContainerOptions::<String> {
            memory: Some(mem),
            memory_swap: Some(mem),
            // HACK: there is no other way to set number of cpus
            // only use this for performance benchmarks and comment
            // out otherwise.
            cpuset_cpus: Some("1".to_string()),
            ..Default::default()
        };
        docker
            .update_container(&id, options)
            .await
            .map_err(|e| e.to_string())?;

        docker
            .start_container::<String>(&id, Some(StartContainerOptions::default()))
            .await
            .map_err(|e| e.to_string())?;

        let info = docker
            .inspect_container(id, None)
            .await
            .map_err(|e| e.to_string())?;
        let ip = info
            .network_settings
            .clone()
            .unwrap()
            .ip_address
            .clone()
            .unwrap();

        Ok(Self {
            name: id.to_owned(),
            ip: ip.parse().unwrap(),
        })
    }

    pub fn ip(&self) -> Ipv4Addr {
        self.ip
    }

    pub async fn kill(&self) -> Result<(), String> {
        let docker = Docker::connect_with_local_defaults().map_err(|e| e.to_string())?;

        let options = Some(KillContainerOptions { signal: "SIGINT" });
        docker
            .kill_container(&self.name, options)
            .await
            .map_err(|e| e.to_string())?;

        let options = Some(RemoveContainerOptions {
            force: true,
            ..Default::default()
        });
        docker
            .remove_container(&self.name, options)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
