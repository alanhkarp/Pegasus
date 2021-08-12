use anyhow::Result;
use pegasus::comm_tests::CommTests;
/// # Agent for Applications
///
/// Each application talks to the agent that spawned it via the application's
/// stdin and stdout whether the application runs native or in a Docker
/// container.

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "docker")]
    let command = {
        let build = std::process::Command::new("docker")
            .arg("build")
            .arg("-t")
            .arg("client")
            .arg(".")
            .output()?;
        println!("\nBuilt client container {:?}", build.status);
        "docker run --rm -v /tmp/socket:/socket -i client"
    };
    #[cfg(not(feature = "docker"))]
    let command = "target/debug/client";
    println!("\nServer: client command: {}", command);
    let tests = vec![
        CommTests::AsyncMutex,
        CommTests::FuturesUnordered,
        CommTests::MpscChannel,
        CommTests::SelectChannel,
        CommTests::SelectTokio,
        CommTests::Sync,
        CommTests::TokioSpawn,
        CommTests::UnixSelect,
    ];
    CommTests::run_comm_tests(command, &tests).await?;
    Ok(())
}
