pub mod eos {

    use csv;
    use std::f64::consts::PI;
    use std::io;

    #[derive(Debug, Clone, Copy)]
    pub struct NuclearParams {
        pub pressure: f64,
        pub energy_density: f64,
    }

    pub struct EquationOfState {
        pub eos: Vec<NuclearParams>,
    }

    impl EquationOfState {
        pub fn new(
            equation_of_state_filepath: &std::path::Path,
        ) -> Result<EquationOfState, io::Error> {
            let eos_file = std::fs::File::open(equation_of_state_filepath)?;
            let mut rdr = csv::ReaderBuilder::new()
                .has_headers(false)
                .delimiter(b',')
                .from_reader(eos_file);

            let mut eos_data = std::vec::Vec::new();

            for line in rdr.records() {
                let raw_string_data = line?;
                let eos_line = NuclearParams {
                    pressure: raw_string_data[1].parse::<f64>().unwrap(),
                    energy_density: raw_string_data[0].parse::<f64>().unwrap(),
                };
                eos_data.push(eos_line);
            }

            Ok(EquationOfState { eos: eos_data })
        }

        fn get_energy_density(&self, pressure: f64) -> f64 {
            let closest_pressure_idx = (0..self.eos.len())
                .map(|idx| (self.eos[idx].pressure - pressure).abs())
                .enumerate()
                .min_by(|x, y| x.1.total_cmp(&y.1))
                .unwrap()
                .0;

            self.eos[closest_pressure_idx].energy_density
        }

        pub fn dmdr(&self, r: &f64, y: &Vec<f64>) -> Vec<f64> {
            let mut out: Vec<f64> = Vec::with_capacity(y.len());

            let pressure = y[0];
            let mass = y[1];
            let rho = self.get_energy_density(pressure);

            out.push(
                (-1.0 * (pressure + rho) * (mass + 4.0 * PI * pressure * r.powi(3)))
                    / (r * (r - 2.0 * mass)),
            );
            out.push(4.0 * PI * rho * r.powi(2));

            if pressure < 0.0 {
                out[0] = 0.0;
                out[1] = 0.0;
            }
            out
        }
    }
}
