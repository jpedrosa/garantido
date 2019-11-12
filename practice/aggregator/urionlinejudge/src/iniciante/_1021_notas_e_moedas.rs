

fn run(lines: Vec<String>) -> String {
    let mut n: f64 = lines[0].parse().unwrap();
    let nota_100 = (n / 100.0).floor();
    n -= nota_100 * 100.0;
    let nota_50 = (n / 50.0).floor();
    n -= nota_50 * 50.0;
    let nota_20 = (n / 20.0).floor();
    n -= nota_20 * 20.0;
    let nota_10 = (n / 10.0).floor();
    n -= nota_10 * 10.0;
    let nota_5 = (n / 5.0).floor();
    n -= nota_5 * 5.0;
    let nota_2 = (n / 2.0).floor();
    n -= nota_2 * 2.0;
    let moeda_100_cent = (n / 1.0).floor();
    n -= moeda_100_cent * 1.0;
    let moeda_50_cent = (n / 0.5).floor();
    n -= moeda_50_cent * 0.5;
    let moeda_25_cent = (n / 0.25).floor();
    n -= moeda_25_cent * 0.25;
    let moeda_10_cent = (n / 0.1).floor();
    n -= moeda_10_cent * 0.1;
    let moeda_5_cent = (n / 0.05).floor();
    n -= moeda_5_cent * 0.05;
    let moeda_1_cent = (n / 0.01).floor();
    format!("NOTAS:\n\
    {} nota(s) de R$ 100.00\n\
    {} nota(s) de R$ 50.00\n\
    {} nota(s) de R$ 20.00\n\
    {} nota(s) de R$ 10.00\n\
    {} nota(s) de R$ 5.00\n\
    {} nota(s) de R$ 2.00\n\
    MOEDAS:\n\
    {} moeda(s) de R$ 1.00\n\
    {} moeda(s) de R$ 0.50\n\
    {} moeda(s) de R$ 0.25\n\
    {} moeda(s) de R$ 0.10\n\
    {} moeda(s) de R$ 0.05\n\
    {} moeda(s) de R$ 0.01", nota_100, nota_50, nota_20, nota_10, nota_5,
        nota_2, moeda_100_cent, moeda_50_cent, moeda_25_cent, moeda_10_cent,
        moeda_5_cent, moeda_1_cent)
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::*;

    #[test]
    fn test_run() {
        assert_run(run, &"576.73", "NOTAS:\n\
        5 nota(s) de R$ 100.00\n\
        1 nota(s) de R$ 50.00\n\
        1 nota(s) de R$ 20.00\n\
        0 nota(s) de R$ 10.00\n\
        1 nota(s) de R$ 5.00\n\
        0 nota(s) de R$ 2.00\n\
        MOEDAS:\n\
        1 moeda(s) de R$ 1.00\n\
        1 moeda(s) de R$ 0.50\n\
        0 moeda(s) de R$ 0.25\n\
        2 moeda(s) de R$ 0.10\n\
        0 moeda(s) de R$ 0.05\n\
        3 moeda(s) de R$ 0.01");
        assert_run(run, &"4.00", "NOTAS:\n\
        0 nota(s) de R$ 100.00\n\
        0 nota(s) de R$ 50.00\n\
        0 nota(s) de R$ 20.00\n\
        0 nota(s) de R$ 10.00\n\
        0 nota(s) de R$ 5.00\n\
        2 nota(s) de R$ 2.00\n\
        MOEDAS:\n\
        0 moeda(s) de R$ 1.00\n\
        0 moeda(s) de R$ 0.50\n\
        0 moeda(s) de R$ 0.25\n\
        0 moeda(s) de R$ 0.10\n\
        0 moeda(s) de R$ 0.05\n\
        0 moeda(s) de R$ 0.01");
        assert_run(run, &"91.01", "NOTAS:\n\
        0 nota(s) de R$ 100.00\n\
        1 nota(s) de R$ 50.00\n\
        2 nota(s) de R$ 20.00\n\
        0 nota(s) de R$ 10.00\n\
        0 nota(s) de R$ 5.00\n\
        0 nota(s) de R$ 2.00\n\
        MOEDAS:\n\
        1 moeda(s) de R$ 1.00\n\
        0 moeda(s) de R$ 0.50\n\
        0 moeda(s) de R$ 0.25\n\
        0 moeda(s) de R$ 0.10\n\
        0 moeda(s) de R$ 0.05\n\
        1 moeda(s) de R$ 0.01");
    }
}
