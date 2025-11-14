
use anyhow::{Context, Result};
use std::collections::HashMap;

pub fn load_config(path: &str) -> Result<HashMap<String, String>> {
    let txt = std::fs::read_to_string(path).with_context(|| format!("reading {path}"))?;
    Ok(parse_kv(&txt))
}

fn parse_kv(input: &str) -> HashMap<String, String> {
    input
        .lines()
        .filter_map(|line| line.split_once('='))
        .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
        .collect()
}
