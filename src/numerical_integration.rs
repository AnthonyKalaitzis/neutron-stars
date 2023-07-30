pub fn euler(
    f: &dyn Fn(&f64, &Vec<f64>) -> Vec<f64>,
    h: &f64,
    t0: &f64,
    y0: &Vec<f64>,
) -> Vec<f64> {
    let mut out = Vec::with_capacity(y0.len());
    for (f_val, y) in std::iter::zip(f(t0, y0), y0) {
        out.push(h * f_val + y);
    }
    out
}
