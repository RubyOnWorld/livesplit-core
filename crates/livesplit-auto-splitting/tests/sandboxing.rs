#![cfg(feature = "unstable")]

use livesplit_auto_splitting::{Runtime, Timer, TimerState};
use log::Log;
use std::{
    cell::RefCell,
    ffi::OsStr,
    fmt::Write,
    fs,
    path::PathBuf,
    process::{Command, Stdio},
    thread,
    time::Duration,
};

thread_local! {
    static BUF: RefCell<Option<String>> = RefCell::new(None);
}
struct Logger;
static LOGGER: Logger = Logger;

impl Log for Logger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }
    fn log(&self, record: &log::Record) {
        if record.target() != "Auto Splitter" {
            return;
        }
        BUF.with(|b| {
            if let Some(b) = &mut *b.borrow_mut() {
                let _ = writeln!(b, "{}", record.args());
            }
        });
    }
    fn flush(&self) {}
}

struct DummyTimer;

impl Timer for DummyTimer {
    fn state(&self) -> TimerState {
        TimerState::NotRunning
    }
    fn start(&mut self) {}
    fn split(&mut self) {}
    fn reset(&mut self) {}
    fn set_game_time(&mut self, time: time::Duration) {}
    fn pause_game_time(&mut self) {}
    fn resume_game_time(&mut self) {}
    fn set_variable(&mut self, key: &str, value: &str) {}
}

fn compile(crate_name: &str) -> anyhow::Result<Runtime<DummyTimer>> {
    let mut path = PathBuf::from("tests");
    path.push("test-cases");
    path.push(crate_name);

    let output = Command::new("cargo")
        .current_dir(&path)
        .arg("build")
        .arg("--target")
        .arg("wasm32-wasi")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .output()
        .unwrap();

    if !output.status.success() {
        let output = String::from_utf8_lossy(&output.stderr);
        panic!("{}", output);
    }

    path.push("target");
    path.push("wasm32-wasi");
    path.push("debug");
    let wasm_path = fs::read_dir(path)
        .unwrap()
        .find_map(|e| {
            let path = e.unwrap().path();
            if path.extension() == Some(OsStr::new("wasm")) {
                Some(path)
            } else {
                None
            }
        })
        .unwrap();

    Ok(Runtime::new(&wasm_path, DummyTimer)?)
}

fn run(crate_name: &str) -> anyhow::Result<()> {
    let mut runtime = compile(crate_name)?;
    runtime.step()?;
    Ok(())
}

#[test]
fn empty() {
    run("empty").unwrap();
}

#[test]
fn proc_exit() {
    assert!(run("proc-exit").is_err());
}

#[test]
fn create_file() {
    run("create-file").unwrap();
}

#[test]
fn stdout() {
    let _ = log::set_logger(&LOGGER);
    log::set_max_level(log::LevelFilter::Trace);
    BUF.with(|b| *b.borrow_mut() = Some(String::new()));
    run("stdout").unwrap();
    let output = BUF.with(|b| b.borrow_mut().take());
    // FIXME: For now we don't actually hook up stdout or stderr.
    assert_eq!(output.unwrap(), "");
}

#[test]
fn segfault() {
    assert!(run("segfault").is_err());
}

#[test]
fn env() {
    run("env").unwrap();
    assert!(std::env::var("AUTOSPLITTER_HOST_SHOULDNT_SEE_THIS").is_err());
}

#[test]
fn threads() {
    // There's no threads in WASI / WASM yet, so this is expected to trap.
    assert!(run("threads").is_err());
}

#[test]
fn sleep() {
    // FIXME: Sleeping can basically deadlock the code. We should have a limit on
    // how long it can sleep.
    run("sleep").unwrap();
}

#[test]
fn time() {
    run("time").unwrap();
}

#[test]
fn random() {
    run("random").unwrap();
}

// #[test]
// fn poll() {
//     // FIXME: This is basically what happens at the lower levels of sleeping. You
//     // can block on file descriptors and have a timeout with this. Both of which
//     // could deadlock the script.
//     run("poll").unwrap();
// }

#[test]
fn infinite_loop() {
    let mut runtime = compile("infinite-loop").unwrap();

    let interrupt = runtime.interrupt_handle();

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(5));
        interrupt.interrupt();
    });

    assert!(runtime.step().is_err());
}

// FIXME: Test Network

// FIXME: Test heavy amounts of allocations
