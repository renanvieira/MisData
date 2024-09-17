
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};


#[tokio::main]
async fn main() {
    // TODO: track tasks that are reading the pages to implement a basic lock

    let t1 = tokio::spawn(async {
        let mut f = tokio::fs::OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open("/tmp/db/data.file")
            .await
            .unwrap();

        let data = "auei|test".as_bytes();

        let _ = f.write(data).await.unwrap();
    });

    let t2 = tokio::spawn(async {
        let mut f = tokio::fs::OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open("/tmp/db/data.file")
            .await
            .unwrap();

        let data = "auei_another_test".as_bytes();
        let buf: Vec<u8> = vec![0; 4096 - data.len()];

        let block = [data, &buf].concat();
        dbg!(&block.len());

        let _ = f.seek(std::io::SeekFrom::Start(4096)).await;

        let _ = f.write(&block).await.unwrap();
    });

    let _ = tokio::join!(t1, t2);

    let t3 = tokio::spawn(async {
        let mut f = tokio::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open("/tmp/db/data.file")
            .await
            .unwrap();

        let _ = f.seek(std::io::SeekFrom::Start(4096)).await;

        let mut data = vec![0; 4096];

        let _ = f.read_exact(&mut data).await.unwrap();

        println!(
            "Page: {:?}",
            String::from_utf8(data.to_vec())
                .unwrap()
                .trim_end_matches("\0")
        );
    });

    let _ = tokio::join!(t3);
}
