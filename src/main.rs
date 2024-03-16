mod root;
use std::{
    fs::{self, File},
    io::Cursor,
};
use tokio::{self, main};

#[main]
async fn main() {
    let songs = get_songs().await;
    let local_songs = get_local_songs();

    for song in songs {
        let file_name = song
            .file
            .url
            .split("/")
            .last()
            .expect("Nie udało się pobrać nazwy pliku");

        print!(
            "Spawdzanie piosenki: {}, tytuł: {}\n",
            file_name, song.title
        );

        if local_songs.contains(&file_name.to_string()) {
            print!("Piosenka jest pobrana :)\n");
            continue;
        }

        print!("Pobieranie\n");
        download_song(&song.file.url).await;
    }
}

async fn get_songs() -> Vec<root::Mp3> {
    let data = reqwest::get("https://b.jw-cdn.org/apis/pub-media/GETPUBMEDIALINKS?output=json&pub=osg&fileformat=MP3%2CAAC&alllangs=0&langwritten=P&txtCMSLang=P").await.unwrap().json::<root::Root>().await;
    match data {
        Ok(res) => res.files.p.mp3,
        Err(_) => panic!("Nie udało się pobrać piosenek"),
    }
}

async fn download_song(url: &str) {
    let name = url.split("/").last().unwrap();

    let resp = reqwest::get(url).await.expect("request failed");
    let mut content = Cursor::new(resp.bytes().await.expect("Nie udało się pobrać pliku"));
    let mut file = File::create(name).expect("failed to create file");
    std::io::copy(&mut content, &mut file).expect("Nie udało się skopiować pliku");
}

fn get_local_songs() -> Vec<String> {
    let paths = fs::read_dir("./").unwrap();
    paths
        .filter_map(|path| match path {
            Ok(path_string) => Some(path_string.file_name()),
            _ => None,
        })
        .filter_map(|file_name| Some(String::from(file_name.to_str().unwrap())))
        .collect::<Vec<String>>()
}
