pub async fn double(n: u32) -> u32 {
    n * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        let result = rt.block_on(
            double(4)
        );

        assert_eq!(result, 8);
    }

    #[tokio::test]
    async fn test_double2() {
        let result = double(3).await;
        assert_eq!(result, 6);
    }
}
