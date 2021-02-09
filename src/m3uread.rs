use m3u::iptv::IptvEntries;
use m3u::iptv::IptvEntry;
use m3u::Reader;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub struct M3uReader {
    reader: Reader<BufReader<File>, IptvEntry>,
}
impl M3uReader {
    pub fn from_file(file: &Path) -> M3uReader {
        let reader = Reader::open_iptv(file).unwrap();
        M3uReader { reader }
    }
    pub fn read(&mut self) -> IptvEntries<BufReader<File>> {
        self.reader.iptv_entries()
    }
}

trait GetProp {
    fn get_prop(&self, prop: &str) -> Option<&str>;
}
impl GetProp for IptvEntry {
    fn get_prop(&self, prop: &str) -> Option<&str> {
        match prop {
            "name" => Some(&self.extinf.name),
            "url" => Some({
                if let m3u::Entry::Url(url) = &self.entry {
                    url.as_str()
                } else {
                    panic!("IPTV channel contains file path")
                }
            }),

            _ => self.extinf.iptv_props.get(prop).map(String::as_str),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GetProp;
    use m3u::iptv::IptvEntry;
    #[test]
    fn can_read_from_file() {
        read_first_chan();
    }

    fn read_first_chan() -> IptvEntry {
        use super::M3uReader;
        use std::path::PathBuf;
        let file = PathBuf::from("test/example.m3u");
        let mut reader = M3uReader::from_file(&file);
        let mut chans = reader.read();
        chans.next().unwrap().unwrap()
    }

    #[test]
    fn name_property_returns_channel_name() {
        let chan = read_first_chan();
        assert_eq!(chan.get_prop("name"), Some("#### BEIN SPORT #####"));
    }
    #[test]
    fn url_property_returns_channel_url() {
        let chan = read_first_chan();
        assert_eq!(
            chan.get_prop("url"),
            Some("http://borg.hopto.org:8090/html/greetings.mp4")
        );
    }
    #[test]
    fn group_name_property_returns_channel_group() {
        let chan = read_first_chan();
        assert_eq!(chan.get_prop("group-title"), Some("AR | BEIN SPORT"));
    }
}
