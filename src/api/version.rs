use anyhow::Result;
use chrono::DateTime;

const VERGEN_GIT_SHA: &str = std::env!("VERGEN_GIT_SHA");
const VERGEN_GIT_COMMIT_TIMESTAMP: &str = std::env!("VERGEN_GIT_COMMIT_TIMESTAMP");

pub fn get_git_commit_short_sha() -> String {
    VERGEN_GIT_SHA[..7].into()
}

pub fn get_git_commit_url() -> String {
    format!(
        "https://github.com/kvnxiao/pascalbot-rs/commit/{}",
        VERGEN_GIT_SHA
    )
}

pub fn get_git_commit_time_formatted() -> Result<String> {
    let datetime = DateTime::parse_from_rfc3339(VERGEN_GIT_COMMIT_TIMESTAMP)?;
    Ok(format!("<t:{}:f>", datetime.timestamp()))
}
