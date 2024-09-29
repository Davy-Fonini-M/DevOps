// src/main.rs

/// Função que soma dois números.
pub fn soma(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_soma() {
        assert_eq!(soma(2, 3), 5);      // Testa soma positiva
        assert_eq!(soma(-1, 1), 0);     // Testa soma com número negativo
        assert_eq!(soma(0, 0), 0);      // Testa soma com zeros
        assert_eq!(soma(-2, -3), -5);   // Testa soma de negativos
    }
}


