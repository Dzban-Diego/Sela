mod root;
use futures::future::TryJoinAll;
use std::{
    env,
    fs::{self, File},
    io::Cursor,
};
use tokio::{self, main};

#[main]
async fn main() {
    let language = get_language();
    let songs = get_songs(language).await;
    let local_songs = get_local_songs();

    let mut que: Vec<tokio::task::JoinHandle<()>> = Vec::new();

    for song in songs {
        let file_name = song
            .file
            .url
            .split("/")
            .last()
            .expect("Nie udało się pobrać nazwy pliku");
        if local_songs.contains(&file_name.to_string()) {
            print!("'{}' jest pobrana :)\n", file_name);
            continue;
        }

        let url = song.file.url;
        let res = tokio::spawn(async move { download_song(url.to_string()).await });
        que.push(res);
        // download_song(url.to_string()).await;
    }

    match que.into_iter().collect::<TryJoinAll<_>>().await {
        Ok(_) => print!("Zaktualizowano piosenki!"),
        Err(e) => print!("{}", e),
    }
}

async fn get_songs(lang: String) -> Vec<root::Mp3> {
    let url = format!("https://b.jw-cdn.org/apis/pub-media/GETPUBMEDIALINKS?output=json&pub=osg&fileformat=MP3%2CAAC&alllangs=0&langwritten={}&txtCMSLang={}", lang, lang);
    print!("Url: {}\n", url);
    let data = reqwest::get(url).await.unwrap().json::<root::Root>().await;

    match data {
        Ok(res) => {
            let test = res.files;
            for key in test {
                let value = serde_json::from_value::<root::Files>(key.1)
                    .expect("Nie udało się odczytać json");
                return value.mp3;
            }
            panic!("Nie udało się odczytać json!")
        }
        Err(e) => panic!("Nie udało się pobrać piosenek. Err: {}", e),
    }
}

async fn download_song(url: String) {
    let name = &url.split("/").last().unwrap();
    print!("Pobieranie {}...\n", name);

    let resp = reqwest::get(&url).await.expect("request failed");
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

fn get_language() -> String {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        return args[1][..].to_string();
    };
    print!(
        "Pobieranie języka polskiego. Możesz też podać skrót języka. \n(Polski: P, Angielski: E)\n"
    );
    "P".to_string()
}
