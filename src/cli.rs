use clap::Error;
use std::ffi::OsString;
use std::fmt::Debug;
use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;

pub fn parse_args<I>(args: I) -> Result<TransformOpts, Error>
where
    I: IntoIterator + Debug,
    I::Item: Into<OsString> + Clone,
{
    TransformOpts::from_iter_safe(args)
}
#[derive(StructOpt, Debug, PartialEq)]
pub struct TransformOpts {
    /// the m3u file
    #[structopt(short = "i")]
    input_file: PathBuf,
    /// the m3u output file
    #[structopt(short = "o")]
    output_file: PathBuf,
    #[structopt(short = "f")]
    filter_on: FilterOn,
}
#[derive(StructOpt, Debug, PartialEq)]
pub struct FilterOn {
    pub prop: String,
    pub value: String,
}
fn split_once(s: &str, sep: char) -> Option<(&str, &str)> {
    let mut parts = s.splitn(2, sep);
    if let Some(prop) = parts.next() {
        if let Some(value) = parts.next() {
            return Some((prop, value));
        }
    }
    None
}

impl FromStr for FilterOn {
    type Err = String;
    fn from_str(s: &str) -> Result<FilterOn, String> {
        if let Some((prop, val)) = split_once(s, '=') {
            Ok(FilterOn {
                prop: prop.to_string(),
                value: val.to_string(),
            })
        } else {
            Err(format!("'{}' is not in property=value format", s))
        }
    }
}

pub fn run<F: FileTransformFacade>(opts: TransformOpts, transform: &F) {
    transform.transform(opts.input_file, opts.output_file, opts.filter_on);
}
pub trait FileTransformFacade {
    fn transform(&self, input_file: PathBuf, output_file: PathBuf, filter_on: FilterOn);
}

/*
*/

#[cfg(test)]
mod tests {
    use super::parse_args;
    use super::run;
    use super::FileTransformFacade;
    use super::FilterOn;
    use super::TransformOpts;
    use std::cell::RefCell;
    use std::path::PathBuf;
    use std::str::FromStr;
    /*
    use super::ChannelListProcessor;
    use super::ChannelListTransformer;
    use super::VecBox;
    */
    #[test]
    fn error_if_no_arg() {
        assert!(!parse_args::<Vec<String>>(Vec::new()).is_ok());
    }
    #[test]
    fn ok_if_input_and_ouput_files_given() {
        let args = vec!["progname", "-i", "toto", "-o", "tata", "-f", "="];
        assert_eq!(
            parse_args(args).unwrap(),
            TransformOpts {
                input_file: "toto".into(),
                output_file: "tata".into(),
                filter_on: FilterOn {
                    prop: String::new(),
                    value: "".to_string()
                }
            }
        );
    }
    /*
     */
    struct TransformMock {
        inp: RefCell<String>,
        outp: RefCell<String>,
        grp: RefCell<String>,
    }
    impl FileTransformFacade for TransformMock {
        fn transform(&self, input_file: PathBuf, output_file: PathBuf, filter_on: FilterOn) {
            self.inp.replace(input_file.to_str().unwrap().to_string());
            self.outp.replace(output_file.to_str().unwrap().to_string());
            self.grp.replace(filter_on.value);
        }
    }

    impl TransformMock {
        fn new() -> TransformMock {
            TransformMock {
                inp: RefCell::new("riendutout".to_string()),
                outp: RefCell::new("riendutout".to_string()),
                grp: RefCell::new("riendutout".to_string()),
            }
        }
        fn has_processed(&self, inp: &str, outp: &str, group: &str) -> bool {
            self.inp.borrow().as_str() == inp
                && self.outp.borrow().as_str() == outp
                && self.grp.borrow().as_str() == group
        }
    }

    fn process_opts(inp: &str, outp: &str, filter_on: &str) -> TransformMock {
        let opts = TransformOpts {
            input_file: inp.into(),
            output_file: outp.into(),
            filter_on: FilterOn::from_str(filter_on).unwrap(),
        };

        let transform = TransformMock::new();
        run(opts, &transform);
        transform
    }
    #[test]
    fn run_transform_if_in_and_out_files_are_set() {
        let transform = process_opts("toto", "tata", "group-title=.");
        assert!(transform.has_processed("toto", "tata", "."));
    }

    #[test]
    fn run_transform_with_empty_filenames_if_filenames_empty() {
        let transform = process_opts("", "", "group-title=.");
        assert!(!transform.has_processed("toto", "tata", "."));
        assert!(transform.has_processed("", "", "."));
    }

    #[test]
    fn filter_on_group_toto_if_relevant_option() {
        let transform = process_opts("toto", "tata", "group-title=EU");
        assert!(transform.has_processed("toto", "tata", "EU"));
    }
}
