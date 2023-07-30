use eos::eos::EquationOfState;
use ndarray::Array;
use plotly::{
    common::Title,
    {Plot, Scatter},
};

mod numerical_integration;

const NUM_NS: usize = 100;
const INV_KM_2: f64 = 1.3234e-6;
const H_STEP: f64 = 0.001;
const SOLAR_MASS: f64 = 1.4766;

fn main() {
    let eos_path = std::path::Path::new("config/processed/qmc700_with_crust.csv");
    let mut eos = EquationOfState::new(eos_path).expect("Failed to open Equation of state file");

    let indices = 0..(eos.eos.len() - 1);
    for idx in indices {
        eos.eos[idx].energy_density *= INV_KM_2;
        eos.eos[idx].pressure *= INV_KM_2;
    }

    let ns_initial_pressure_guesses = Array::logspace(40.0, -1.1, 1.5, NUM_NS);
    let mut star_masses: Vec<f64> = Vec::with_capacity(NUM_NS);
    let mut star_radii: Vec<f64> = Vec::with_capacity(NUM_NS);
    let mut t = H_STEP;
    let f = |r: &_, y: &_| eos.dmdr(r, y);

    for ns_p_guess in ns_initial_pressure_guesses {
        let m0 = 1.0e-10;
        let mut y0 = vec![ns_p_guess * INV_KM_2, m0];

        while y0[0] >= 0.0 {
            y0 = numerical_integration::euler(&f, &H_STEP, &t, &y0);
            t += H_STEP;
        }

        star_masses.push(y0[1] / SOLAR_MASS);
        star_radii.push(t);
    }

    // Create a scatter plot with the data
    let scatter = Scatter::new(star_radii, star_masses).name("Mass Radius Relation");

    // Create a plot and add the scatter plot
    let mut plot = Plot::new();
    plot.add_trace(scatter);

    // Customize the plot layout
    plot.set_layout(plotly::Layout::new().title(Title::from("Mass Radius Relation")));

    // Show the plot in a browser window
    plot.show();
}
