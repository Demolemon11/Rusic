use std::{fs, path::PathBuf, time::Duration};

use lofty::{
    file::{AudioFile, TaggedFileExt},
    probe::Probe,
    tag::{Accessor, ItemKey},
};
use rfd::FileDialog;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AudioInfo {
    title: String,      // 标题
    genre: String,      // 流派
    artist: String,     // 艺术家
    album: String,      // 专辑
    track_num: String,  // 音轨号
    disk_num: String,   // 碟号
    bit_depth: u8,      // 位宽
    sampling_rate: u32, // 采样率
    duration: Duration, // 时长
    path: PathBuf,      // 音频路径
}
impl Default for AudiosInfo {
    fn default() -> Self {
        let folder = FileDialog::new()
            .pick_folder()
            .expect("Please choose a folder.");

        Self(
            audio_files(folder)
                .iter()
                .map(|item| {
                    let tagged_file = Probe::open(item)
                        //TODO ErrHandle
                        .unwrap()
                        .guess_file_type()
                        //TODO ErrHandle
                        .unwrap()
                        .read()
                        //TODO ErrHandle
                        .unwrap();

                    let tag = tagged_file.primary_tag().expect("No Title");

                    let title = tag.title().as_deref().unwrap_or("None").to_owned();
                    let album = tag.album().as_deref().unwrap_or("None").to_owned();
                    let artist = tag.artist().as_deref().unwrap_or("None").to_owned();
                    let genre = tag.genre().as_deref().unwrap_or("None").to_owned();
                    let track_num = tag
                        .get_string(&ItemKey::TrackNumber)
                        .unwrap_or("None")
                        .to_owned();
                    let disk_num = tag
                        .get_string(&ItemKey::DiscNumber)
                        //TODO ErrHandle
                        .unwrap_or("None")
                        .to_owned();

                    let bit_depth = tagged_file.properties().bit_depth().unwrap_or(0);
                    let sampling_rate = tagged_file.properties().sample_rate().unwrap_or(0);
                    let duration = tagged_file.properties().duration();
                    let path = item.to_owned();
                    AudioInfo {
                        title,
                        genre,
                        artist,
                        album,
                        track_num,
                        disk_num,
                        bit_depth,
                        sampling_rate,
                        duration,
                        path,
                    }
                })
                .collect::<Vec<_>>(),
        )
    }
}
#[derive(Serialize, Deserialize)]
pub struct AudiosInfo(pub Vec<AudioInfo>);

fn audio_files(path: PathBuf) -> Vec<PathBuf> {
    let iter1 = fs::read_dir(&path)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|item| {
            item.is_file()
                && (item.to_str().unwrap().to_lowercase().ends_with(".flac")
                    || item.to_str().unwrap().to_lowercase().ends_with(".mp3")
                    || item.to_str().unwrap().to_lowercase().ends_with(".ogg")
                    || item.to_str().unwrap().to_lowercase().ends_with(".wav"))
        });

    let iter2 = fs::read_dir(path)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|item| item.is_dir())
        .flat_map(|item| audio_files(item).into_iter());
    iter1.chain(iter2).collect()
}
