use math::{standard_normal_cdf, standard_normal_pdf};
use option::{Call, Greeks};
use {ACCURACY, HIGH_VALUE, MAX_ITERATIONS};

pub enum VolError {
    HighVol,
    TooManyIterations,
}

impl Call {
    /// Compute the price using the Black and Scoles formula
    pub fn bs_price(&self) -> f64 {
        let sqrt_t = self.t.sqrt();
        let sig_sqrt_t = self.vol * sqrt_t;
        let d1 = ((self.s / self.k).ln() + self.r * self.t) / sig_sqrt_t + sig_sqrt_t / 2.0;
        let d2 = d1 - sig_sqrt_t;
        let cdf1 = standard_normal_cdf(d1);
        let cdf2 = standard_normal_cdf(d2);
        self.s * cdf1 - self.k * (-self.r * self.t).exp() * cdf2
    }

    /// Compute the price and the Greeks using the Black and Scoles formula
    /// Reuse intermediate variables to compute greeks
    pub fn bs_price_and_greeks(&self) -> (f64, Greeks) {
        let sqrt_t = self.t.sqrt();
        let sig_sqrt_t = self.vol * sqrt_t;
        let d1 = ((self.s / self.k).ln() + self.r * self.t) / sig_sqrt_t + sig_sqrt_t / 2.0;
        let d2 = d1 - sig_sqrt_t;
        let cdf1 = standard_normal_cdf(d1);
        let cdf2 = standard_normal_cdf(d2);
        let pdf1 = standard_normal_pdf(d1);
        let price = self.s * cdf1 - self.k * (-self.r * self.t).exp() * cdf2;
        let greeks = Greeks {
            delta: cdf1,
            gamma: pdf1 / (self.s * sig_sqrt_t),
            theta: -(self.s * self.vol * pdf1) / (2.0 * sqrt_t)
                - self.r * self.k * (-self.r * self.t).exp() * cdf2,
            vega: self.s * sqrt_t * pdf1,
            rho: self.k * self.t * (-self.r * self.t).exp() * cdf2,
        };
        (price, greeks)
    }

    /// Update self.vol based on observable call_price
    pub fn bissect_implied_vol(&mut self, call_price: f64) -> Result<(), VolError> {
        let mut low_call = self.clone();
        low_call.vol = 1.0e-5;

        if low_call.bs_price() > call_price {
            self.vol = 0.0;
            return Ok(());
        }

        let mut high_call = self.clone();
        high_call.vol = 0.3;
        while high_call.bs_price() < call_price {
            high_call.vol *= 2.0;
            if high_call.vol > HIGH_VALUE {
                return Err(VolError::HighVol);
            }
        }
        for _ in 0..MAX_ITERATIONS {
            self.vol = (low_call.vol + high_call.vol) * 0.5;
            let diff = self.bs_price() - call_price;
            if diff.abs() < ACCURACY {
                return Ok(());
            }
            if diff < 0.0 {
                low_call.vol = self.vol;
            } else {
                high_call.vol = self.vol;
            }
        }
        Err(VolError::TooManyIterations)
    }

    /// Update self.vol based on observable call_price
    pub fn newton_implied_vol(&mut self, call_price: f64) -> Result<(), VolError> {
        self.vol = 1.0e-5;
        if self.bs_price() > call_price {
            return Ok(());
        }

        let sqrt_t = self.t.sqrt();
        self.vol = (call_price / self.s) / (0.398 * sqrt_t);
        for _ in 0..MAX_ITERATIONS {
            let diff = call_price - self.bs_price();
            if diff.abs() < ACCURACY {
                return Ok(());
            }
            let d1 =
                ((self.s / self.k).ln() + sqrt_t) / (self.vol * sqrt_t) + 0.5 * self.vol * sqrt_t;
            let vega = self.s * sqrt_t * standard_normal_pdf(d1);
            self.vol += diff / vega;
        }
        Err(VolError::TooManyIterations)
    }
}
