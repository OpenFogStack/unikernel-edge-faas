use std::fmt::Debug;
use std::fs;
use std::path::{Path, PathBuf};

mod config;

pub use config::*;

#[derive(Debug)]
pub struct Registry {
    /// Canonical path to function directory
    functions: PathBuf,
}

impl Registry {
    pub fn new(functions: &Path) -> Self {
        let functions = functions.canonicalize().unwrap();

        if !functions.exists() {
            panic!(
                "Function path at {} does not exist",
                functions.to_str().unwrap()
            );
        }

        Registry { functions }
    }

    pub fn get_function(&self, name: &str) -> Result<FunctionConfig, String> {
        let mut function_config = self.load_function_config(name)?;

        match &mut function_config.runtime {
            config::RuntimeConfig::Vm(vm) => {
                // TODO: validate kernel and image path
                let kernel_path = self.functions.join(name).join(&vm.kernel);
                let function_kernel = kernel_path
                    .canonicalize()
                    .map_err(|_| format!("Failed to canonicalize path {:?}", kernel_path))?;

                let image_path = self.functions.join(name).join(&vm.image);
                let function_image = image_path
                    .canonicalize()
                    .map_err(|_| format!("Failed to canonicalize path {:?}", image_path))?;

                vm.kernel = function_kernel;
                vm.image = function_image;
            }
            config::RuntimeConfig::Container(_container) => (),
        }

        Ok(function_config)
    }

    fn load_function_config(&self, name: &str) -> Result<config::FunctionConfig, String> {
        let fpath = self.functions.join(name).join("function.toml");

        let config_str = fs::read_to_string(fpath).map_err(|e| e.to_string())?;

        config::FunctionConfig::from_string(&config_str)
    }
}
