use std::{collections::BTreeMap, fmt};

use serde::{Deserialize, Deserializer, Serialize};

/// 配置文件
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub log: Log,
    pub language: String,
    pub retry_count: u8,
    pub root_dir: String,
    pub replace: bool,
    pub compress: Compress,
    pub pdf: Pdf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            log: Log::default(),
            language: "Chinese".to_owned(),
            retry_count: 5u8,
            root_dir: ".".to_owned(),
            replace: false,
            compress: Compress::default(),
            pdf: Pdf::default(),
        }
    }
}

/// 模块日志等级配置
#[derive(Debug, Serialize, Deserialize)]
pub struct Log {
    pub level: LogLevelMap,
}
impl Default for Log {
    fn default() -> Self {
        Self {
            level: LogLevelMap::default(),
        }
    }
}

/// 模块日志等级映射
#[derive(Debug, Serialize, Deserialize)]
pub struct LogLevelMap(BTreeMap<String, LogLevel>);

impl Default for LogLevelMap {
    fn default() -> Self {
        let mut map = BTreeMap::new();
        map.insert("nhentai_rs".to_owned(), LogLevel::Info);
        Self(map)
    }
}

impl fmt::Display for LogLevelMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut pairs: Vec<String> = Vec::new();
        for (key, value) in &self.0 {
            pairs.push(format!("{}={}", key, value));
        }
        write!(f, "{}", pairs.join(","))
    }
}

/// 日志等级
#[derive(Debug, Serialize)]
enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

/// 自定义日志等级反序列化，使其大小写不敏感
impl<'de> Deserialize<'de> for LogLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let level = String::deserialize(deserializer)?;
        Ok(match level.to_uppercase().as_str() {
            "TRACE" => LogLevel::Trace,
            "DEBUG" => LogLevel::Debug,
            "INFO" => LogLevel::Info,
            "WARN" => LogLevel::Warn,
            "ERROR" => LogLevel::Error,
            _ => return Err(serde::de::Error::custom("Invalid log level")),
        })
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "trace"),
            LogLevel::Debug => write!(f, "debug"),
            LogLevel::Info => write!(f, "info"),
            LogLevel::Warn => write!(f, "warn"),
            LogLevel::Error => write!(f, "error"),
        }
    }
}

/// 压缩配置
#[derive(Debug, Serialize, Deserialize)]
pub struct Compress {
    pub enable: bool,
    pub password: String,
    pub dir: String,
}

impl Default for Compress {
    fn default() -> Self {
        Self {
            enable: false,
            password: "".to_owned(),
            dir: "cpr".to_owned(),
        }
    }
}

/// PDF配置
#[derive(Debug, Serialize, Deserialize)]
pub struct Pdf {
    pub enable: bool,
    pub dir: String,
}

impl Default for Pdf {
    fn default() -> Self {
        Self {
            enable: false,
            dir: "pdf".to_owned(),
        }
    }
}
