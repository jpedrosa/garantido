
use super::super::{split_columns};


fn run(lines: Vec<String>) -> String {
    let colunas = split_columns(&lines[0]);
    let a: i32 = colunas[0].parse().unwrap();
    let b: i32 = colunas[1].parse().unwrap();
    let c: i32 = colunas[2].parse().unwrap();
    let d: i32 = colunas[3].parse().unwrap();
    let check = b > c && d > a && c + d > a + b && c >= 0 && d >= 0 && 
        a % 2 == 0;
    (if check {
        "Valores aceitos"
    } else {
        "Valores nao aceitos"
    }).to_string()
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::*;

    #[test]
    fn test_run() {
        assert_run(run, &"5 6 7 8", "Valores nao aceitos");
        assert_run(run, &"2 3 2 6", "Valores aceitos");
    }
}
