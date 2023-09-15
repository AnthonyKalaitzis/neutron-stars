use std::usize;

use nalgebra::SVector;

pub fn euler<const D: usize>(
    f: &dyn Fn(f64, SVector<f64, D>) -> SVector<f64, D>,
    h: f64,
    t0: f64,
    y0: SVector<f64, D>,
) -> SVector<f64, D> {
    y0 + f(t0, y0) * h
}

pub fn rk4<const D: usize>(
    f: &dyn Fn(f64, SVector<f64, D>) -> SVector<f64, D>,
    h: f64,
    t0: f64,
    y0: SVector<f64, D>,
) -> SVector<f64, D> {
    let k1 = f(t0, y0);
    let k2 = f(t0 + (h / 2.0), y0 + (h * k1 / 2.0));
    let k3 = f(t0 + (h / 2.0), y0 + (h * k2 / 2.0));
    let k4 = f(t0 + h, y0 + (h * k3));
    y0 + (h / 6.0) * (k1 + (2.0 * k2) + (2.0 * k3) + k4)
}

#[cfg(test)]
mod tests {
    use crate::numerical_integration::{euler, rk4};
    use nalgebra::Vector1;

    #[test]
    fn initial_value_problem_euler() {
        let f = |_t: f64, y: Vector1<f64>| y.clone();
        let t0 = 0.0;
        let mut y0 = Vector1::new(1.0);
        let h = 1.0;

        y0 = euler(&f, h, t0, y0);
        assert_eq!(y0[0], 2.0);
        y0 = euler(&f, h, t0, y0);
        assert_eq!(y0[0], 4.0);
        y0 = euler(&f, h, t0, y0);
        assert_eq!(y0[0], 8.0)
    }

    #[test]
    fn initial_value_problem_rk4() {
        let f = |_t: f64, y: Vector1<f64>| y.clone();
        let mut t0 = 0.0;
        let mut y0 = Vector1::new(1.0);
        let h = 0.1;

        let mut counter = 0;
        while counter < 10 {
            counter += 1;
            y0 = rk4(&f, h, t0, y0);
            t0 += h;
        }
        println!("{:?}", y0)
    }
}
