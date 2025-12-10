use std::process::Command;

pub struct Operation {
    pub operation_type: OperationType,
    pub name: String,
}

pub enum OperationType {
    Start,
    Stop,
    Reload,
    Restart,
    Enable,
    Disable,
}

impl Operation {
    pub fn new(operation_type: OperationType, name: String) -> Self {
        Self {
            operation_type,
            name,
        }
    }

    pub fn execute(&self) -> std::io::Result<()> {
        let operation_type = match self.operation_type {
            OperationType::Start => "start",
            OperationType::Stop => "stop",
            OperationType::Reload => "reload",
            OperationType::Restart => "restart",
            OperationType::Enable => "enable",
            OperationType::Disable => "disable",
        };

        Command::new("sudo")
            .args(["systemctl", operation_type, &self.name])
            .status()?;

        Ok(())
    }
}
