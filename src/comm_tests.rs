use crate::async_mutex::main_async_mutex;
use crate::futures_unordered::main_futures_unordered;
use crate::mpsc::main_mpsc;
use crate::select_channel::main_select_channel;
use crate::select_tokio::main_select_tokio;
use crate::sync::main_sync;
use crate::tokio_spawn::main_tokio_spawn;
use crate::unix_select::main_unix_select;
use rand::prelude::*;
use std::fmt;
pub enum CommTests {
    AsyncMutex,
    FuturesUnordered,
    MpscChannel,
    SelectChannel,
    SelectTokio,
    Sync,
    TokioSpawn,
    UnixSelect,
}
impl CommTests {
    pub async fn run_comm_tests(
        command: &str,
        tests: &[CommTests],
    ) -> Result<(), Box<dyn std::error::Error>> {
        for test in tests {
            println!("\nServer running {} communications test", test);
            match test {
                CommTests::AsyncMutex => main_async_mutex(command).await?,
                CommTests::FuturesUnordered => main_futures_unordered(command).await?,
                CommTests::MpscChannel => main_mpsc(command)?,
                CommTests::SelectChannel => main_select_channel(command)?,
                CommTests::SelectTokio => main_select_tokio(command).await?,
                CommTests::Sync => main_sync(command)?,
                CommTests::TokioSpawn => main_tokio_spawn(command).await?,
                CommTests::UnixSelect => main_unix_select(command)?,
            }
        }
        Ok(())
    }
}
pub fn random_sleep(who: &str, id: u32) {
    let ms: u8 = rand::thread_rng().gen();
    println!("{} {} sleeping for {} ms", who, id, ms);
    std::thread::sleep(std::time::Duration::from_millis(ms as u64));
    println!("{} {} awake", who, id);
}
impl fmt::Display for CommTests {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = match self {
            CommTests::AsyncMutex => "AsyncMutex",
            CommTests::FuturesUnordered => "FuturesUnordered",
            CommTests::MpscChannel => "MpscChannel",
            CommTests::SelectChannel => "SelectChannel",
            CommTests::SelectTokio => "SelectTokio",
            CommTests::Sync => "Sync",
            CommTests::TokioSpawn => "TokioSpawn",
            CommTests::UnixSelect => "UnixSelect",
        };
        write!(f, "{}", out)
    }
}
