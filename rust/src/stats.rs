use std::{fmt, time::Duration};

pub struct Statistics {
    avg: Duration,
    min: Duration,
    max: Duration,
    std_dev: Duration,
    total: Duration,
}

pub fn compute_statistics(values: &[Duration]) -> Statistics {
    let avg = values.iter().cloned().sum::<Duration>() / values.len() as u32;
    let min = values.iter().min().unwrap();
    let max = values.iter().max().unwrap();
    let variance = values
        .iter()
        .map(|&x| (x.as_secs_f64() - avg.as_secs_f64()).powi(2))
        .sum::<f64>()
        / (values.len() - 1) as f64;
    let std_dev = Duration::from_secs_f64(variance.sqrt());
    let total = values.iter().sum();
    Statistics {
        avg,
        min: *min,
        max: *max,
        std_dev,
        total,
    }
}

impl fmt::Display for Statistics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "avg: {:.2?}(+/-{:.2?}), min: {:.2?}, max: {:.2?}, total: {:.2?}",
            self.avg, self.std_dev, self.min, self.max, self.total
        )
    }
}
