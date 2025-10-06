use crate::database::DataBase;
use rodio::source::EmptyCallback;
use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink};
use tokio::runtime::Runtime;
use std::fs::File;
use std::io::BufReader;
use std::sync::atomic::{AtomicI32, Ordering};

pub static PLAYER: once_cell::sync::Lazy<Player> =
    once_cell::sync::Lazy::new(|| Player::default().unwrap());
static RUNTIME: once_cell::sync::Lazy<Runtime> = once_cell::sync::Lazy::new(|| {
    Runtime::new().expect("Failed to create Tokio runtime")
});

pub static ID:AtomicI32 = AtomicI32::new(0);

pub struct Player {
    _stream: OutputStream,
    sink: Sink,
    pub play_order: PlayOrder,
}

pub enum PlayOrder {
    InOrder,
    Random,
    SingleLoop,
}

impl Player {
    pub fn default() -> Result<Player, Box<dyn std::error::Error>> {
        let output_stream = OutputStreamBuilder::open_default_stream()?;
        let sink = Sink::connect_new(&output_stream.mixer());
        Ok(Player {
            _stream: output_stream,
            sink,
            play_order: PlayOrder::InOrder,
        })
    }

    pub async fn play(&self, id: i32, table: String, database: DataBase)-> Result<(), Box<dyn std::error::Error>> {
        let path =database.get_path_by_id(id, &table).await?;
        let source = decode(&path)?;
        self.sink.stop();
        self.sink.append(source);
        self.sink.play();
        Ok(())

    }

    pub fn toggle(&self) {
        if self.sink.is_paused(){
            self.sink.play();
        }else {
            self.sink.pause();
        }
    }

    pub fn next(&self) {
        self.sink.skip_one();
    }


    fn play_with_callback<F>(&self, callback: F)
    where
        F: Fn() + Send + 'static,
    {
        let callback = EmptyCallback::new(Box::new(callback));
        self.sink.append(callback);
    }
}

fn decode(path: &str) -> Result<Decoder<BufReader<File>>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let source = Decoder::new(BufReader::new(file))?;
    Ok(source)
}

pub async fn play(id:i32, table:String, database:DataBase){
    let database_clone=database.clone();
    let table_clone=table.clone();

    ID.store(id,Ordering::SeqCst);
    let current_id=ID.fetch_add(1,Ordering::SeqCst);
    let source=decode(&database.get_path_by_id(current_id,&table).await.unwrap()).unwrap();
    PLAYER.sink.stop();
    PLAYER.sink.append(source);
    PLAYER.play(id, table, database).await.unwrap();
    let callback = move|| {
        let id=ID.fetch_add(1, Ordering::SeqCst);
        RUNTIME.block_on(async {
            let path= database_clone.get_path_by_id(id, &table_clone).await.unwrap();
            let source=decode(&path).unwrap();
            PLAYER.sink.append(source);
        });
    };
    PLAYER.play_with_callback(callback);

}