
use super::super::{split_columns};


fn run(lines: Vec<String>) -> String {
    let colunas = split_columns(&lines[0]);
    let a: f64 = colunas[0].parse().unwrap();
    let b: f64 = colunas[1].parse().unwrap();
    let c: f64 = colunas[2].parse().unwrap();
    let sqrt_delta = ((b * b) - (4.0 * a * c)).sqrt();
    let r1 = (- b + sqrt_delta) / (2.0 * a);
    let r2 = (- b - sqrt_delta) / (2.0 * a);
    if r1.is_nan() || r1.is_infinite() || r2.is_nan() || r2.is_infinite() {
        "Impossivel calcular".to_string()
    } else {
        format!("R1 = {:.5}\nR2 = {:.5}", r1, r2)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::*;

    #[test]
    fn test_run() {
        assert_run(run, &"10.0 20.1 5.1", "R1 = -0.29788\nR2 = -1.71212");
        assert_run(run, &"0.0 20.0 5.0", "Impossivel calcular");
        assert_run(run, &"10.3 203.0 5.0", "R1 = -0.02466\nR2 = -19.68408");
        assert_run(run, &"10.0 3.0 5.0", "Impossivel calcular");
    }
}
