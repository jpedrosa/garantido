

fn run(lines: Vec<String>) -> String {
    let mut n: u32 = lines[0].parse().unwrap();
    let ano = n / 365;
    n -= ano * 365;
    let mes = n / 30;
    n -= mes * 30;
    format!("{} ano(s)\n{} mes(es)\n{} dia(s)", ano, mes, n)
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::*;

    #[test]
    fn test_run() {
        assert_run(run, &"400", "1 ano(s)\n1 mes(es)\n5 dia(s)");
        assert_run(run, &"800", "2 ano(s)\n2 mes(es)\n10 dia(s)");
        assert_run(run, &"30", "0 ano(s)\n1 mes(es)\n0 dia(s)");
    }
}
