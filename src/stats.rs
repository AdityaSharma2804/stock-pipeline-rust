//! StatsAccumulator computes streaming statistics without keeping all values in memory.

/// Accumulates sample statistics such as min, max, mean, and standard deviation.
pub struct StatsAccumulator {
    /// Number of values processed so far.
    pub count: u64,
    /// Smallest value seen so far.
    pub min: f64,
    /// Largest value seen so far.
    pub max: f64,
    mean: f64, // running average of seen values
    m2: f64,   // running sum of squared differences for variance
}

impl StatsAccumulator {
    /// Create a new empty accumulator.
    pub fn new() -> Self {
        Self {
            count: 0,
            min: f64::MAX,
            max: f64::MIN,
            mean: 0.0,
            m2: 0.0,
        }
    }

    /// Update the accumulator with a new price.
    pub fn update(&mut self, price: f64) {
        // update count and bounds
        self.count += 1;

        if price < self.min {
            self.min = price;
        }
        if price > self.max {
            self.max = price;
        }

        // update running mean and variance without storing all values
        let delta = price - self.mean;
        self.mean += delta / self.count as f64;
        let delta2 = price - self.mean;
        self.m2 += delta * delta2;
    }
    /// Merge another accumulator into this one.
    pub fn merge(&mut self, other: StatsAccumulator) {
        if other.count == 0 {
            return;
        }

        let combined_count = self.count + other.count;

        // parallel Welford merge for mean and M2
        let delta = other.mean - self.mean;
        let new_mean = self.mean + delta * (other.count as f64 / combined_count as f64);
        let new_m2 = self.m2
            + other.m2
            + delta * delta * (self.count as f64 * other.count as f64 / combined_count as f64);

        self.count = combined_count;
        self.mean = new_mean;
        self.m2 = new_m2;
        self.min = self.min.min(other.min);
        self.max = self.max.max(other.max);
    }

    /// Finalize statistics and return (min, max, mean, std_dev).
    pub fn finalize(&self) -> (f64, f64, f64, f64) {
        let std_dev = if self.count > 1 {
            (self.m2 / (self.count - 1) as f64).sqrt()
        } else {
            0.0
        };
        (self.min, self.max, self.mean, std_dev)
    }
}
