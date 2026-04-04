use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortEntry {
    pub port: u16,
    pub pid: u32,
    pub address: String,
    pub process: ProcessInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub command: String,
    pub uptime: String,
    pub uptime_seconds: u64,
    pub memory_kb: u64,
    pub ppid: u32,
    pub status: ProcessStatus,
    pub cwd: Option<String>,
    pub project_name: Option<String>,
    pub framework: Option<crate::framework::Framework>,
    pub git_branch: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProcessStatus {
    Healthy,
    Orphaned,
    Zombie,
}

impl ProcessStatus {
    pub fn symbol(&self) -> &'static str {
        match self {
            ProcessStatus::Healthy => "●",
            ProcessStatus::Orphaned => "◐",
            ProcessStatus::Zombie => "✕",
        }
    }

    pub fn color(&self) -> colored::Color {
        match self {
            ProcessStatus::Healthy => colored::Color::Green,
            ProcessStatus::Orphaned => colored::Color::Yellow,
            ProcessStatus::Zombie => colored::Color::Red,
        }
    }
}

impl fmt::Display for ProcessStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use colored::Colorize;
        write!(f, "{}", self.symbol().color(self.color()))
    }
}

impl ProcessInfo {
    pub fn memory_mb(&self) -> f64 {
        self.memory_kb as f64 / 1024.0
    }

    pub fn is_dev_process(&self) -> bool {
        const DEV_RUNTIMES: &[&str] = &[
            "node", "python", "python3", "ruby", "go", "cargo", "java", "javac", "mvn", "gradle",
            "npm", "yarn", "pnpm", "bun", "deno", "php", "elixir", "mix", "dotnet", "rails",
            "puma", "uvicorn", "gunicorn",
        ];

        let name_lower = self.name.to_lowercase();
        DEV_RUNTIMES
            .iter()
            .any(|runtime| name_lower.contains(runtime))
            || self.framework.is_some()
    }

    pub fn is_docker_process(&self) -> bool {
        self.name == "docker" || self.name.contains("docker")
    }

    pub fn is_system_process(&self) -> bool {
        const SYSTEM_APPS: &[&str] = &[
            "Spotify",
            "Raycast",
            "Slack",
            "Discord",
            "Electron",
            "Google Chrome",
            "Safari",
            "Firefox",
            "systemd",
            "launchd",
            "cron",
            "sshd",
            "httpd",
        ];

        SYSTEM_APPS.iter().any(|app| self.name.contains(app))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerContainer {
    pub id: String,
    pub name: String,
    pub image: String,
    pub ports: Vec<PortMapping>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortMapping {
    pub host_port: u16,
    pub container_port: u16,
}

impl DockerContainer {
    pub fn detect_service(&self) -> Option<&'static str> {
        let image_lower = self.image.to_lowercase();

        if image_lower.contains("postgres") {
            Some("PostgreSQL")
        } else if image_lower.contains("redis") {
            Some("Redis")
        } else if image_lower.contains("mongo") {
            Some("MongoDB")
        } else if image_lower.contains("mysql") || image_lower.contains("mariadb") {
            Some("MySQL")
        } else if image_lower.contains("nginx") {
            Some("nginx")
        } else if image_lower.contains("localstack") {
            Some("LocalStack")
        } else if image_lower.contains("elasticsearch") {
            Some("Elasticsearch")
        } else if image_lower.contains("rabbitmq") {
            Some("RabbitMQ")
        } else if image_lower.contains("kafka") {
            Some("Kafka")
        } else {
            None
        }
    }
}
