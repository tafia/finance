use math::{standard_normal_cdf, standard_normal_pdf};
use option::{Call, Greeks, Put};
use {ACCURACY, HIGH_VALUE, MAX_ITERATIONS};

pub enum VolError {
    HighVol,
    TooManyIterations,
}

#[derive(Default, Debug)]
pub struct GreeksBuilder {
    pub delta: bool,
    pub gamma: bool,
    pub theta: bool,
    pub vega: bool,
    pub rho: bool,
}

impl Call {
    /// Compute the price using the Black and Scoles formula
    pub fn bs_price(&self) -> f64 {
        let sqrt_t = self.t.sqrt();
        let sig_sqrt_t = self.vol * sqrt_t;
        let d1 =
            ((self.s / self.k).ln() + (self.r - self.q) * self.t) / sig_sqrt_t + sig_sqrt_t / 2.0;
        let d2 = d1 - sig_sqrt_t;
        let cdf1 = standard_normal_cdf(d1);
        let cdf2 = standard_normal_cdf(d2);
        self.s * (-self.q * self.t).exp() * cdf1 - self.k * (-self.r * self.t).exp() * cdf2
    }

    /// Compute the price and the Greeks using the Black and Scoles formula
    /// Reuse intermediate variables to compute greeks
    pub fn bs_price_and_greeks(&self, builder: &GreeksBuilder) -> (f64, Greeks) {
        let sqrt_t = self.t.sqrt();
        let sig_sqrt_t = self.vol * sqrt_t;
        let d1 =
            ((self.s / self.k).ln() + (self.r - self.q) * self.t) / sig_sqrt_t + sig_sqrt_t / 2.0;
        let d2 = d1 - sig_sqrt_t;
        let cdf1 = standard_normal_cdf(d1);
        let cdf2 = standard_normal_cdf(d2);
        let pdf1 = if builder.gamma || builder.vega {
            standard_normal_pdf(d1)
        } else {
            0.
        };
        let eqt = (-self.q * self.t).exp();
        let ert = (-self.r * self.t).exp();
        let price = self.s * eqt * cdf1 - self.k * ert * cdf2;
        // for other greeks, see wikipedia
        // https://en.wikipedia.org/wiki/Greeks_(finance)
        let greeks = Greeks {
            delta: if builder.delta { eqt * cdf1 } else { 0. },
            gamma: if builder.gamma {
                pdf1 / (self.s * sig_sqrt_t)
            } else {
                0.
            },
            theta: if builder.theta {
                -(self.s * eqt * self.vol * pdf1) / (2.0 * sqrt_t) - self.r * self.k * ert * cdf2
                    + self.q * self.s * eqt * cdf1
            } else {
                0.
            },
            vega: if builder.vega {
                self.s * eqt * sqrt_t * pdf1
            } else {
                0.
            },
            rho: if builder.rho {
                self.k * self.t * ert * cdf2
            } else {
                0.
            },
        };
        (price, greeks)
    }
}

impl Put {
    /// Compute the price using the Black and Scoles formula
    pub fn bs_price(&self) -> f64 {
        let sqrt_t = self.t.sqrt();
        let sig_sqrt_t = self.vol * sqrt_t;
        let d1 =
            ((self.s / self.k).ln() + (self.r - self.q) * self.t) / sig_sqrt_t + sig_sqrt_t / 2.0;
        let d2 = d1 - sig_sqrt_t;
        let cdf1 = standard_normal_cdf(-d1);
        let cdf2 = standard_normal_cdf(-d2);
        self.k * (-self.r * self.t).exp() * cdf2 - self.s * (-self.q * self.t).exp() * cdf1
    }

    /// Compute the price and the Greeks using the Black and Scoles formula
    /// Reuse intermediate variables to compute greeks
    pub fn bs_price_and_greeks(&self, builder: &GreeksBuilder) -> (f64, Greeks) {
        let sqrt_t = self.t.sqrt();
        let sig_sqrt_t = self.vol * sqrt_t;
        let d1 =
            ((self.s / self.k).ln() + (self.r - self.q) * self.t) / sig_sqrt_t + sig_sqrt_t / 2.0;
        let d2 = d1 - sig_sqrt_t;
        let cdf1 = standard_normal_cdf(-d1);
        let cdf2 = standard_normal_cdf(-d2);
        let pdf1 = if builder.gamma || builder.vega {
            standard_normal_pdf(d1)
        } else {
            0.
        };
        let eqt = (-self.q * self.t).exp();
        let ert = (-self.r * self.t).exp();
        let price = self.k * ert * cdf2 - self.s * eqt * cdf1;
        // for other greeks, see wikipedia
        // https://en.wikipedia.org/wiki/Greeks_(finance)
        let greeks = Greeks {
            delta: if builder.delta { -eqt * cdf1 } else { 0. },
            gamma: if builder.gamma {
                eqt * pdf1 / (self.s * sig_sqrt_t)
            } else {
                0.
            },
            theta: if builder.theta {
                -(self.s * eqt * self.vol * pdf1) / (2.0 * sqrt_t) + self.r * self.k * ert * cdf2
                    - self.q * self.s * eqt * cdf1
            } else {
                0.
            },
            vega: if builder.vega {
                self.s * eqt * sqrt_t * pdf1
            } else {
                0.
            },
            rho: if builder.rho {
                -self.k * self.t * ert * cdf2
            } else {
                0.
            },
        };
        (price, greeks)
    }
}

macro_rules! impl_implied_vol {
    ($option:ident) => {
        impl $option {
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

                self.vol = (call_price / self.s) / (0.398 * self.t.sqrt());
                let builder = GreeksBuilder {
                    vega: true,
                    ..GreeksBuilder::default()
                };
                for _ in 0..MAX_ITERATIONS {
                    let (price, greeks) = self.bs_price_and_greeks(&builder);
                    let diff = call_price - price;
                    if diff.abs() < ACCURACY {
                        return Ok(());
                    }
                    self.vol += diff / greeks.vega;
                }
                Err(VolError::TooManyIterations)
            }
        }
    };
}

impl_implied_vol!(Call);
impl_implied_vol!(Put);
