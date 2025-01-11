use std::cmp::Ordering;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Token {
    pub amount: u64,
    pub denom: String
}

impl PartialOrd for Token {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.denom != other.denom {
            return None;
        }
        Some(self.amount.cmp(&other.amount))
    }
}

impl Ord for Token {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.denom != other.denom {
            panic!("Cannot compare tokens with different denominations");
        }
        self.amount.cmp(&other.amount)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equality() {
        let token1 = Token {
            amount: 100,
            denom: "LOKI".to_string(),
        };

        let token2 = Token {
            amount: 100,
            denom: "LOKI".to_string(),
        };

        let token3 = Token {
            amount: 200,
            denom: "LOKI".to_string(),
        };

        let token4 = Token {
            amount: 100,
            denom: "USDT".to_string(),
        };

        assert_eq!(token1, token2);
        assert_ne!(token1, token3);
        assert_ne!(token1, token4);
    }

    #[test]
    fn test_comparison() {
        let token1 = Token {
            amount: 100,
            denom: "LOKI".to_string(),
        };

        let token2 = Token {
            amount: 200,
            denom: "LOKI".to_string(),
        };

        let token3 = Token {
            amount: 100,
            denom: "USDT".to_string(),
        };

        assert!(token1 < token2); // 100 < 200
        assert!(token2 > token1); // 200 > 100
        assert!(token1 <= token2); // 100 <= 200
        assert!(token2 >= token1); // 200 >= 100

        assert!(token1.partial_cmp(&token3).is_none());
    }

    #[test]
    #[should_panic(expected = "Cannot compare tokens with different denominations")]
    fn test_ord_panic() {
        let token1 = Token {
            amount: 100,
            denom: "LOKI".to_string(),
        };

        let token2 = Token {
            amount: 200,
            denom: "USDT".to_string(),
        };

        let _ = token1.cmp(&token2);
    }
}
