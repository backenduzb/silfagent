#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Silf::app::bootstrap::run().await
}
