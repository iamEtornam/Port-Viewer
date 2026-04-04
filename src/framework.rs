use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Framework {
    NextJs,
    Vite,
    Angular,
    Remix,
    Astro,
    Express,
    Fastify,
    Nuxt,
    Django,
    FastAPI,
    Flask,
    Rails,
    Puma,
    Go,
    Cargo,
    Unknown(String),
}

impl Framework {
    pub fn display_name(&self) -> &str {
        match self {
            Framework::NextJs => "Next.js",
            Framework::Vite => "Vite",
            Framework::Angular => "Angular",
            Framework::Remix => "Remix",
            Framework::Astro => "Astro",
            Framework::Express => "Express",
            Framework::Fastify => "Fastify",
            Framework::Nuxt => "Nuxt",
            Framework::Django => "Django",
            Framework::FastAPI => "FastAPI",
            Framework::Flask => "Flask",
            Framework::Rails => "Rails",
            Framework::Puma => "Puma",
            Framework::Go => "Go",
            Framework::Cargo => "Rust/Cargo",
            Framework::Unknown(name) => name,
        }
    }

    pub fn emoji(&self) -> &str {
        match self {
            Framework::NextJs => "⚡",
            Framework::Vite => "⚡",
            Framework::Angular => "🅰️",
            Framework::Remix => "💿",
            Framework::Astro => "🚀",
            Framework::Express => "🚂",
            Framework::Fastify => "⚡",
            Framework::Nuxt => "💚",
            Framework::Django => "🎸",
            Framework::FastAPI => "⚡",
            Framework::Flask => "🌶️",
            Framework::Rails => "🛤️",
            Framework::Puma => "🐆",
            Framework::Go => "🐹",
            Framework::Cargo => "🦀",
            Framework::Unknown(_) => "📦",
        }
    }
}

#[derive(Debug, Deserialize)]
struct PackageJson {
    dependencies: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "devDependencies")]
    dev_dependencies: Option<std::collections::HashMap<String, String>>,
}

pub async fn detect_framework(cwd: &str, cmdline: &str) -> Option<Framework> {
    // 1. Try package.json
    if let Ok(framework) = detect_from_package_json(cwd).await {
        return Some(framework);
    }

    // 2. Try cmdline
    if let Some(framework) = detect_from_cmdline(cmdline) {
        return Some(framework);
    }

    // 3. Try process name
    detect_from_process_name(cmdline)
}

async fn detect_from_package_json(cwd: &str) -> Result<Framework> {
    let package_json_path = Path::new(cwd).join("package.json");
    if !package_json_path.exists() {
        return Err(anyhow::anyhow!("No package.json"));
    }

    let content = tokio::fs::read_to_string(&package_json_path).await?;
    let package: PackageJson = serde_json::from_str(&content)?;

    let all_deps: Vec<String> = package
        .dependencies
        .unwrap_or_default()
        .keys()
        .chain(package.dev_dependencies.unwrap_or_default().keys())
        .cloned()
        .collect();

    // Priority order
    if all_deps.iter().any(|d| d == "next") {
        return Ok(Framework::NextJs);
    }
    if all_deps.iter().any(|d| d == "vite") {
        return Ok(Framework::Vite);
    }
    if all_deps.iter().any(|d| d.starts_with("@angular/core")) {
        return Ok(Framework::Angular);
    }
    if all_deps.iter().any(|d| d.starts_with("@remix-run")) {
        return Ok(Framework::Remix);
    }
    if all_deps.iter().any(|d| d == "astro") {
        return Ok(Framework::Astro);
    }
    if all_deps.iter().any(|d| d == "express") {
        return Ok(Framework::Express);
    }
    if all_deps.iter().any(|d| d == "fastify") {
        return Ok(Framework::Fastify);
    }
    if all_deps.iter().any(|d| d == "nuxt") {
        return Ok(Framework::Nuxt);
    }

    Err(anyhow::anyhow!("No known framework"))
}

fn detect_from_cmdline(cmdline: &str) -> Option<Framework> {
    let lower = cmdline.to_lowercase();

    if lower.contains("django") {
        Some(Framework::Django)
    } else if lower.contains("uvicorn") || lower.contains("fastapi") {
        Some(Framework::FastAPI)
    } else if lower.contains("flask") {
        Some(Framework::Flask)
    } else if lower.contains("rails") {
        Some(Framework::Rails)
    } else if lower.contains("puma") {
        Some(Framework::Puma)
    } else if lower.contains("cargo run") || lower.contains("cargo watch") {
        Some(Framework::Cargo)
    } else if lower.contains("go run") {
        Some(Framework::Go)
    } else {
        None
    }
}

fn detect_from_process_name(cmdline: &str) -> Option<Framework> {
    let lower = cmdline.to_lowercase();

    if lower.starts_with("node") {
        Some(Framework::Unknown("Node.js".to_string()))
    } else if lower.starts_with("python") {
        Some(Framework::Unknown("Python".to_string()))
    } else if lower.starts_with("ruby") {
        Some(Framework::Unknown("Ruby".to_string()))
    } else if lower.starts_with("go") {
        Some(Framework::Go)
    } else if lower.starts_with("cargo") {
        Some(Framework::Cargo)
    } else {
        None
    }
}
