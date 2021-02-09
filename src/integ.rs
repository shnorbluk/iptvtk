use crate::cli;
use crate::cli::FileTransformFacade;
use crate::cli::FilterOn;
use crate::m3uread::M3uReader;
use crate::m3uwrite::write_to_file;
use crate::processor::FilteringTransformer;
use crate::processor::GetProperty;
use crate::processor::TransformChain;
use m3u::iptv::IptvEntry;
use std::ffi::OsString;
use std::fmt::Debug;
use std::path::PathBuf;
struct FileTransform {}
impl FileTransformFacade for FileTransform {
    fn transform(&self, input_file: PathBuf, output_file: PathBuf, filter_on: FilterOn) {
        let mut reader = M3uReader::from_file(&input_file);
        let chans = reader.read().map(Result::unwrap).collect();
        let filter = FilteringTransformer::new(filter_on.prop, filter_on.value);
        let chain: TransformChain<IptvEntry, Vec<IptvEntry>> = TransformChain::new(filter);
        let chans = chain.process_all_channels(chans);
        write_to_file(chans.into_iter(), &output_file);
    }
}
impl GetProperty<String> for IptvEntry {
    fn get(&self, prop: &String) -> Option<&str> {
        self.extinf.iptv_props.get(prop).map(String::as_ref)
    }
}

pub fn main<A: Iterator<Item = OsString> + Debug>(args: A) {
    let cli = cli::parse_args(args).unwrap_or_else(|err: clap::Error| err.exit());
    let transform = FileTransform {};
    cli::run(cli, &transform);
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;
    use std::path::Path;
    use std::str::FromStr;
    macro_rules! assert_delta {
        ($x:expr, $y:expr, $d:expr) => {
            if !($x - $y < $d || $y - $x < $d) {
                panic!("abs({}-{})>{}", $x, $y, $d);
            }
        };
    }
    fn main(prog: &str, inp: &str, out: &str, filter: &str) {
        let opts = vec![
            OsString::from_str(prog).unwrap(),
            OsString::from_str("-i").unwrap(),
            OsString::from_str(inp).unwrap(),
            OsString::from_str("-o").unwrap(),
            OsString::from_str(out).unwrap(),
            OsString::from_str("-f").unwrap(),
            OsString::from_str(filter).unwrap(),
        ];
        super::main(opts.into_iter());
    }
    #[test]
    fn output_file_smaller_if_dummy_filter() {
        let src_path_str = "test/example.m3u";
        let src_path = Path::new(src_path_str);
        let mut dest_path = std::env::temp_dir();
        dest_path.push("output.m3u");
        let dest_path_str = dest_path.to_str().unwrap();
        main("prog", src_path_str, dest_path_str, "toto=titi");
        let src_len = src_path.metadata().unwrap().len();
        let dest_len = dest_path.metadata().unwrap().len();
        assert!(
            src_len - dest_len > 10,
            "src={} dest={} src.len={} dest.len={}",
            src_path_str,
            dest_path_str,
            src_len,
            dest_len
        );
    }

    #[test]
    fn input_and_output_size_equal_if_filtering_on_actual_group() {
        let src_path_str = "test/example.m3u";
        let src_path = Path::new(src_path_str);
        let mut dest_path = std::env::temp_dir();
        dest_path.push("output.m3u");
        let dest_path_str = dest_path.to_str().unwrap();
        main(
            "prog",
            src_path_str,
            dest_path_str,
            "group-title=AR | BEIN SPORT",
        );
        let src_len = src_path.metadata().unwrap().len();
        let dest_len = dest_path.metadata().unwrap().len();
        assert_delta!(src_len, dest_len, 3);
    }
}
