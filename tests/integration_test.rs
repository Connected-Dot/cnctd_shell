#[cfg(test)]
mod tests {
    use cnctd_shell::Shell;

    #[tokio::test]
    async fn test_commands() {
        Shell::run("ls", true).await.unwrap();
        Shell::run_with_exit_status("ls", true).await.unwrap();
    }
}
