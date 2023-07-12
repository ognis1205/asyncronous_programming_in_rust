use futures;
use futures::executor::block_on;

struct Song;

async fn learn_song() -> Song {
    Song
}

async fn sing_song(song: Song) {
    println!("sing a song");
}

async fn dance() {
    println!("dance");
}

async fn leanr_and_song() {
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main() {
    let f1 = leanr_and_song();
    let f2 = dance();
    futures::join!(f1, f2);
}

fn main() {
    let f = async_main();
    block_on(f);
}
