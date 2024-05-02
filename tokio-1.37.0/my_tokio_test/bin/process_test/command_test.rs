use tokio::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Like above, but use `output` which returns a future instead of
    // immediately returning the `Child`.
    let output = Command::new("echo").arg("hello").arg("world").output();

    let output = output.await?;

    assert!(output.status.success());
    assert_eq!(output.stdout, b"hello world\n");
    Ok(())
}
