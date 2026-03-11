#[cfg(test)]
pub mod test_sandbox {
    use micrograd_rs::value::Value;
    use petgraph::dot::{Config, Dot};
    #[test]

    pub fn test_sandbox() {
        let a = Value::new_with_label(2.0, "a");
        let b = Value::new_with_label(-3.0, "b");
        let c = Value::new_with_label(10.0, "c");
        let d = a * b + c;

        assert_eq!(d.data, 4.0);

        std::fs::write(
            "test.dot",
            format!(
                "{}",
                Dot::with_config(
                    &d.draw(),
                    &[
                        Config::EdgeNoLabel,
                        Config::RankDir(petgraph::dot::RankDir::LR)
                    ]
                )
            ),
        )
        .unwrap();
    }
}

#[cfg(test)]
pub mod test {
    use micrograd_rs::value::Value;

    #[test]
    pub fn test_value() {
        let value = Value::new(1.0);

        println!("{value:?}");
        assert_eq!(value.data, 1.0)
    }

    #[test]
    pub fn test_add() {
        let a = Value::new(1.0);
        let b = Value::new(2.0);

        assert_eq!((a + b).data, 3.0)
    }

    #[test]
    pub fn test_mul() {
        let a = Value::new(-3.0);
        let b = Value::new(10.0);

        assert_eq!((a * b).data, -30.0)
    }
}
