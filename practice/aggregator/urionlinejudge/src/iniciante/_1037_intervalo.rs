

fn run(lines: Vec<String>) -> String {
    let n: f64 = lines[0].parse().unwrap();
    (if n >= 0.0 && n <= 25.0 {
        "Intervalo [0,25]"
    } else if n > 25.0 && n <= 50.0 {
        "Intervalo (25,50]"
    } else if n > 50.0 && n <= 75.0 {
        "Intervalo (50,75]"
    } else if n > 75.0 && n <= 100.0 {
        "Intervalo (75,100]"
    } else {
        "Fora de intervalo"
    }).to_string()
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::*;

    #[test]
    fn test_run() {
        assert_run(run, &"25.01", "Intervalo (25,50]");
        assert_run(run, &"25.00", "Intervalo [0,25]");
        assert_run(run, &"100.00", "Intervalo (75,100]");
        assert_run(run, &"-25.02", "Fora de intervalo");
    }
}
