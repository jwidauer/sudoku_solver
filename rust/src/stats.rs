use std::fmt;

pub struct Statistics {
    avg: f64,
    min: f64,
    max: f64,
    std_dev: f64,
}

pub fn compute_statistics(values: &[f64]) -> Statistics {
    let avg = values.iter().sum::<f64>() / values.len() as f64;
    let min = values
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max = values
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let std_dev =
        values.iter().map(|x| (x - avg).powi(2)).sum::<f64>().sqrt() / values.len() as f64;
    Statistics {
        avg,
        min: *min,
        max: *max,
        std_dev,
    }
}

impl fmt::Display for Statistics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "avg: {:.2}(+/-{:.2})us, min: {:.2}us, max: {:.2}us",
            self.avg, self.std_dev, self.min, self.max
        )
    }
}
