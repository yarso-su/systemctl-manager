use super::Terminal;
use std::process::Command;

pub struct Operation {
    pub operation_type: OperationType,
    pub name: String,
}

#[derive(PartialEq)]
pub enum OperationType {
    Status,
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

    fn needs_sudo(&self) -> bool {
        self.operation_type != OperationType::Status
    }

    pub fn execute(&self) {
        let operation_type = match self.operation_type {
            OperationType::Status => "status",
            OperationType::Start => "start",
            OperationType::Stop => "stop",
            OperationType::Reload => "reload",
            OperationType::Restart => "restart",
            OperationType::Enable => "enable",
            OperationType::Disable => "disable",
        };

        if self.needs_sudo() {
            if Command::new("sudo")
                .args(["systemctl", operation_type, &self.name])
                .status()
                .is_err()
            {
                let _ = Terminal::print("Command failed\r\n");
            }

            return;
        }

        let _ = Command::new("systemctl")
            .args([operation_type, &self.name])
            .status();
    }
}
