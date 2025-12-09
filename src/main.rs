use active_win_pos_rs::get_active_window;
use chrono::Local;
use clap::{Parser, Subcommand};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

// --- New Imports for Optimization ---
use xcap::Monitor;
use image::imageops::FilterType;
use std::sync::Mutex;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hasher};
use once_cell::sync::Lazy;

// --- Configuration ---
const INACTIVITY_THRESHOLD: Duration = Duration::from_secs(300); // 5 minutes
const KEY_FILE: &str = "session.key";
const LOG_FILE: &str = "productivity_log.csv";
const SCREENSHOT_DIR: &str = "screenshots";

// Global state to store the hash of the last screenshot (for deduplication)
static LAST_HASH: Lazy<Mutex<u64>> = Lazy::new(|| Mutex::new(0));

// Type alias for HMAC-SHA256
type HmacSha256 = Hmac<Sha256>;

// --- CLI Structure ---
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Starts the productivity monitoring session
    Monitor {
        /// Interval to log data in seconds
        #[arg(short, long, default_value_t = 60)]
        log_interval: u64,

        /// Interval to take screenshots in seconds
        #[arg(short, long, default_value_t = 300)]
        screenshot_interval: u64,
    },
    /// Verifies the integrity of the log file against the session key
    Verify {
        /// Path to the log file to verify
        #[arg(short, long, default_value = "productivity_log.csv")]
        file: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Monitor { log_interval, screenshot_interval } => {
            run_monitor(log_interval, screenshot_interval);
        }
        Commands::Verify { file } => {
            run_verification(&file);
        }
    }
}

// --- MODE: Monitor ---
fn run_monitor(log_interval: u64, screenshot_interval: u64) {
    println!("--- Starting Monitor Mode ---");
    println!("Logging every {}s | Screenshots every {}s", log_interval, screenshot_interval);
    println!("Optimizations: JPEG Compression + 50% Resize + Deduplication Active");

    fs::create_dir_all(SCREENSHOT_DIR).expect("Failed to create screenshot directory");
    let secret_key = load_or_create_key();

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(LOG_FILE)
        .expect("Failed to open log file");

    if file.metadata().unwrap().len() == 0 {
        writeln!(file, "Timestamp,Keystrokes,MouseEvents,Status,AppName,WindowTitle,ScreenshotPath,Signature")
            .expect("Failed to write CSV header");
    }

    // Graceful Shutdown
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        println!("\nCtrl+C received. Finalizing...");
    }).expect("Error setting Ctrl+C handler");

    // Input Listener
    let key_events = Arc::new(AtomicU64::new(0));
    let mouse_events = Arc::new(AtomicU64::new(0));
    let key_clone = key_events.clone();
    let mouse_clone = mouse_events.clone();

    thread::spawn(move || {
        rdev::listen(move |event| match event.event_type {
            rdev::EventType::KeyPress(_) => { key_clone.fetch_add(1, Ordering::SeqCst); }
            rdev::EventType::MouseMove { .. }
            | rdev::EventType::Wheel { .. }
            | rdev::EventType::ButtonPress(_) => { mouse_clone.fetch_add(1, Ordering::SeqCst); }
            _ => (),
        }).unwrap_or_else(|e| eprintln!("Listener error: {:?}", e));
    });

    // Loop Variables
    let mut last_activity_time = Instant::now();
    let mut last_log_time = Instant::now();
    let mut last_screenshot_time = Instant::now();
    let mut is_inactive = false;

    while running.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_millis(100));
        let now = Instant::now();

        // 1. Handle Screenshots
        let mut screenshot_filename = String::from("None");
        if now.duration_since(last_screenshot_time).as_secs() >= screenshot_interval {
            last_screenshot_time = now;
            if !is_inactive {
                screenshot_filename = capture_screenshots_optimized();
            }
        }

        // 2. Handle Logging
        if now.duration_since(last_log_time).as_secs() >= log_interval {
            last_log_time = now;

            let keys = key_events.swap(0, Ordering::SeqCst);
            let mice = mouse_events.swap(0, Ordering::SeqCst);
            let total_events = keys + mice;

            let status: &str;
            let mut app_name = String::from("N/A");
            let mut window_title = String::from("N/A");

            if total_events > 0 {
                last_activity_time = now;
                is_inactive = false;
                status = "ACTIVE";
                if let Ok(active_window) = get_active_window() {
                    app_name = active_window.app_name;
                    window_title = active_window.title.replace(',', "");
                }
            } else {
                if now.duration_since(last_activity_time) >= INACTIVITY_THRESHOLD {
                    status = "INACTIVE";
                    is_inactive = true;
                } else {
                    status = "IDLE";
                    if let Ok(active_window) = get_active_window() {
                        app_name = active_window.app_name;
                        window_title = active_window.title.replace(',', "");
                    }
                }
            }

            // Construct Data String
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            let raw_entry = format!(
                "{},{},{},{},{},\"{}\",{}",
                timestamp, keys, mice, status, app_name, window_title, screenshot_filename
            );

            // Sign
            let signature = sign_data(&raw_entry, &secret_key);

            // Write
            if let Err(e) = writeln!(file, "{},{}", raw_entry, signature) {
                eprintln!("Failed to write to log: {}", e);
            }
        }
    }
}

// --- MODE: Verify ---
fn run_verification(file_path: &str) {
    println!("--- Starting Verification Mode ---");
    println!("Checking file: {}", file_path);

    // 1. Load Key
    if !Path::new(KEY_FILE).exists() {
        eprintln!("ERROR: '{}' not found. Cannot verify logs without the original key.", KEY_FILE);
        return;
    }
    let key = load_key();

    // 2. Open Log File
    let file = match std::fs::File::open(file_path) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("ERROR: Could not open log file '{}'", file_path);
            return;
        }
    };
    let reader = BufReader::new(file);

    // 3. Iterate and Check
    let mut valid_count = 0;
    let mut error_count = 0;
    let mut total_lines = 0;

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap_or_default();
        if index == 0 { continue; } // Skip header
        if line.trim().is_empty() { continue; }

        total_lines += 1;

        match line.rfind(',') {
            Some(split_idx) => {
                let content = &line[..split_idx];
                let file_signature = &line[split_idx + 1..];
                let calculated_signature = sign_data(content, &key);

                if file_signature == calculated_signature {
                    valid_count += 1;
                } else {
                    error_count += 1;
                    println!("[TAMPER DETECTED] Line {}: Signature mismatch!", index + 1);
                    println!("   Content: {}", content);
                }
            }
            None => {
                println!("[FORMAT ERROR] Line {}: Could not parse CSV.", index + 1);
                error_count += 1;
            }
        }
    }

    println!("\n--- Verification Summary ---");
    println!("Total Rows Checked: {}", total_lines);
    println!("Valid Rows:         {}", valid_count);
    println!("Tampered/Invalid:   {}", error_count);

    if error_count == 0 {
        println!("✅ LOG INTEGRITY VERIFIED");
    } else {
        println!("❌ INTEGRITY COMPROMISED");
    }
}

// --- Optimized Capture Function ---
fn capture_screenshots_optimized() -> String {
    let monitors = Monitor::all().unwrap_or_else(|_| vec![]);
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let mut saved_filename = String::from("None");

    if let Some(monitor) = monitors.first() {
        if let Ok(image) = monitor.capture_image() {
            
            // 1. Deduplication: Hash the raw bytes
            let mut hasher = DefaultHasher::new();
            hasher.write(image.as_raw()); // Hash pixel data
            let new_hash = hasher.finish();

            // Check against previous hash
            let mut last_hash_guard = LAST_HASH.lock().unwrap();
            if *last_hash_guard == new_hash {
                return String::from("Duplicate (Skipped)");
            }
            *last_hash_guard = new_hash;

            // 2. Resizing: Downscale to 50%
            let new_width = image.width() / 2;
            let new_height = image.height() / 2;
            let resized_image = image::imageops::resize(&image, new_width, new_height, FilterType::Triangle);

            // 3. Compression: Save as JPEG
            let filename = format!("{}/screen_{}.jpg", SCREENSHOT_DIR, timestamp);
            
            if resized_image.save(&filename).is_ok() {
                saved_filename = filename;
            }
        }
    }
    saved_filename
}

fn sign_data(data: &str, key: &[u8]) -> String {
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(data.as_bytes());
    hex::encode(mac.finalize().into_bytes())
}

fn load_or_create_key() -> Vec<u8> {
    if Path::new(KEY_FILE).exists() {
        load_key()
    } else {
        println!("Generating new session key...");
        let key: [u8; 32] = rand::random();
        let mut file = std::fs::File::create(KEY_FILE).expect("Cannot create key file");
        file.write_all(&key).expect("Cannot write key file");
        key.to_vec()
    }
}

fn load_key() -> Vec<u8> {
    let mut file = std::fs::File::open(KEY_FILE).expect("Cannot open key file");
    let mut key = Vec::new();
    file.read_to_end(&mut key).expect("Cannot read key file");
    key
}