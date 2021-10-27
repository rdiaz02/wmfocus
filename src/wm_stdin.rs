use anyhow::{Context, Result};
use std::{
    str::FromStr,
    num::ParseIntError,
    io::stdin,
    io::BufRead
};

use crate::DesktopWindow;

impl FromStr for DesktopWindow {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s.split_whitespace().collect();
        Ok(DesktopWindow {
            id: v[0].parse().unwrap(),
            x_window_id: Some(v[0].parse().unwrap()),
            pos: ( v[1].parse().unwrap(), v[2].parse().unwrap() ),
            size: ( v[3].parse().unwrap(), v[4].parse().unwrap() ),
            is_focused: v[5].parse().unwrap()
        })
    }
}    

pub fn get_windows() -> Result<Vec<DesktopWindow>> {
    let windows: Vec<DesktopWindow> = stdin()
        .lock()
        .lines()
        .filter_map(|line_result| line_result.ok())
        .filter_map(|line| line.parse().ok())
        .collect();
    Ok(windows)
}

pub fn focus_window(window: &DesktopWindow) -> Result<()> {
    Ok(())
}