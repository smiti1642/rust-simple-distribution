use plotters::prelude::*;
use rand::thread_rng;
use rand::Rng;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: {} <n> <l> <a>", args[0]);
        std::process::exit(1);
    }

    let n = args[1].parse::<usize>().unwrap_or_else(|_| {
        eprintln!("Error: <n> must be an integer.");
        std::process::exit(1);
    });

    let l = args[2].parse::<usize>().unwrap_or_else(|_| {
        eprintln!("Error: <l> must be an integer.");
        std::process::exit(1);
    });

    let a = args[3].parse::<usize>().unwrap_or_else(|_| {
        eprintln!("Error: <a> must be an integer.");
        std::process::exit(1);
    });

    let mut rng = thread_rng();
    let mut m = vec![0; a];

    for _ in 0..n {
        let x: f64 = (0..l).map(|_| rng.gen::<f64>()).sum::<f64>() / l as f64;
        let index = ((x * a as f64) as usize).min(a - 1);
        m[index] += 1;
    }

    let m: Vec<f64> = m.iter().map(|&count| count as f64 / n as f64 * a as f64 * 0.1).collect();

    let root = BitMapBackend::new("distribution.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Distribution", ("sans-serif", 50))
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..a, 0f64..1f64)?;

    chart.configure_mesh().draw()?;
    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            .data(m.into_iter().enumerate().map(|(x, y)| (x, y))),
    )?;

    Ok(())
}