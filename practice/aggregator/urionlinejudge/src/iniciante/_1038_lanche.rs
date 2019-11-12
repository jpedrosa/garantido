
use super::super::{split_columns};


fn run(lines: Vec<String>) -> String {
    let colunas = split_columns(&lines[0]);
    let cod: u32 = colunas[0].parse().unwrap();
    let quant: f64 = colunas[1].parse().unwrap();
    let total: f64 = quant * match cod {
        1 => 4.0, // Cachorro
        2 => 4.5, // X-salada
        3 => 5.0, // X-bacon
        4 => 2.0, // Torrada
        5 => 1.5, // Refrigerante
        _ => 0.0
    };
    format!("Total: R$ {:.2}", total)
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::*;

    #[test]
    fn test_run() {
        assert_run(run, &"3 2", "Total: R$ 10.00");
        assert_run(run, &"4 3", "Total: R$ 6.00");
        assert_run(run, &"2 3", "Total: R$ 13.50");
    }
}
