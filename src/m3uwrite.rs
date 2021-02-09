use m3u::iptv::IptvEntry;
use std::path::Path;
pub fn write_to_file<I: Iterator<Item = IptvEntry>>(chans: I, path: &Path) {
    let mut file = std::fs::File::create(path).unwrap();
    let mut writer = m3u::Writer::new_iptv(&mut file).unwrap();
    for entry in chans {
        writer.write_entry(&entry).unwrap();
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn can_write_to_file() {
        use m3u::iptv;
        use std::env::temp_dir;
        let mut path = temp_dir();
        path.push("output.m3u");

        //let file = PathBuf::from("test/output.m3u");
        let chan = m3u::path_entry("")
            .extend(123.0, "Sample artist - Sample title")
            .with_iptv(iptv!("a" = "b"));
        let chans = vec![chan];
        super::write_to_file(chans.into_iter(), &path);
        println!("{:?}", path);
        assert!(path.exists());
    }
}
