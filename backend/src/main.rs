#![allow(dead_code)]
#![allow(unused_variables)]

use rand::prelude::*;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct TelemetryReading {
    chamber_pressure: f64,
    thrust: f64,
    flow_rate: f64,
    temperature: f64,
    vibration: f64,
    stage: LaunchStage,
}

#[derive(Debug, Clone, Copy)]
enum LaunchStage{
    PreIgnition,
    Ignition,
    FullThrust,
    MaxQ,
    ThrottleDown,
    ThrottleUp,
    MainEngineCutoff,
}

fn generate_reading(stage: LaunchStage) -> TelemetryReading {
    match stage {
        LaunchStage::PreIgnition => TelemetryReading {
            chamber_pressure: add_noise(0.0, 0.1),
            thrust: add_noise(0.0, 0.1),
            flow_rate: add_noise(0.0, 0.1),
            temperature: add_noise(20.0, 5.0),
            vibration: add_noise(0.0, 0.1),
            stage,
        },
        LaunchStage::Ignition => TelemetryReading {
            chamber_pressure: add_noise(50.0, 5.0),
            thrust: add_noise(200.0, 20.0),
            flow_rate: add_noise(10.0, 0.1),
            temperature: add_noise(150.0, 5.0),
            vibration: add_noise(1.0, 0.1),
            stage,
        },
        LaunchStage::FullThrust => TelemetryReading {
            chamber_pressure: add_noise(100.0, 5.0),
            thrust: add_noise(500.0, 25.0),
            flow_rate: add_noise(20.0, 0.1),
            temperature: add_noise(300.0, 5.0),
            vibration: add_noise(2.0, 0.1),
            stage,
        },
        LaunchStage::MaxQ => TelemetryReading {
            chamber_pressure: add_noise(120.0, 5.0),
            thrust: add_noise(600.0, 25.0),
            flow_rate: add_noise(25.0, 0.1),
            temperature: add_noise(350.0, 5.0),
            vibration: add_noise(3.0, 0.1),
            stage,
        },
        LaunchStage::ThrottleDown => TelemetryReading {
            chamber_pressure: add_noise(80.0, 5.0),
            thrust: add_noise(400.0, 25.0),
            flow_rate: add_noise(15.0, 0.1),
            temperature: add_noise(250.0, 5.0),
            vibration: add_noise(1.5, 0.1),
            stage,
        },
        LaunchStage::ThrottleUp => TelemetryReading {
            chamber_pressure: add_noise(110.0, 5.0),
            thrust: add_noise(550.0, 25.0),
            flow_rate: add_noise(22.5, 0.1),
            temperature: add_noise(320.0, 5.0),
            vibration: add_noise(2.5, 0.1),
            stage,
        },
        LaunchStage::MainEngineCutoff => TelemetryReading {
            chamber_pressure: add_noise(10.0, 5.0),
            thrust: add_noise(50.0, 25.0),
            flow_rate: add_noise(5.0, 0.1),
            temperature: add_noise(100.0, 5.0),
            vibration: add_noise(1.0, 0.1),
            stage,
        },
    }           
    
}

fn add_noise (value: f64, max: f64) -> f64 {
    let mut rng: ThreadRng = rand::rng();
    let noise: f64 = rng.random_range(-max..max);
    value + noise
}

fn run_stage(stage: LaunchStage, duration: Duration) {
    let start = std::time::Instant::now();
    while start.elapsed() < duration {
        let reading = generate_reading(stage);
        println!("{:?}", reading);
        thread::sleep(Duration::from_millis(100));
    }
}

fn main() {
    run_stage(LaunchStage::PreIgnition, Duration::from_secs(3));
    run_stage(LaunchStage::Ignition, Duration::from_secs(2));
    run_stage(LaunchStage::FullThrust, Duration::from_secs(8));
    run_stage(LaunchStage::MaxQ, Duration::from_secs(4));
    run_stage(LaunchStage::ThrottleDown, Duration::from_secs(3));
    run_stage(LaunchStage::ThrottleUp, Duration::from_secs(3));
    run_stage(LaunchStage::MainEngineCutoff, Duration::from_secs(2));
}
