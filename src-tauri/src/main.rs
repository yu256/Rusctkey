// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rusctkey::AppBuilder;

pub fn main() {
    AppBuilder::new().run();
}
