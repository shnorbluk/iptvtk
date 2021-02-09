use std::fmt::Debug;
pub trait ChannelListTransformer<Chan, I>
where
    I: IntoIterator<Item = Chan>,
{
    fn transform_channel_list(&self, chans: Vec<Chan>) -> Vec<Chan>;
}
pub struct FilteringTransformer<P: PartialEq> {
    prop: P,
    value: String,
}
impl<P: PartialEq> FilteringTransformer<P> {
    pub fn new(prop: P, value: String) -> FilteringTransformer<P> {
        FilteringTransformer { prop, value }
    }
}
pub trait GetProperty<P> {
    fn get(&self, prop: &P) -> Option<&str>;
}

macro_rules! debug{
    ($( $args:expr ),*) => {
        if cfg!(test) {
            println!( $( $args ),* ); }
    }
}

impl<P: PartialEq + Debug, Chan: GetProperty<P>, I: IntoIterator<Item = Chan>>
    ChannelListTransformer<Chan, I> for FilteringTransformer<P>
{
    fn transform_channel_list(&self, chans: Vec<Chan>) -> Vec<Chan> {
        chans
            .into_iter()
            .inspect(|chan| {
                debug!(
                    "chan[{:?}]={:?}, exp={}",
                    &self.prop,
                    chan.get(&self.prop),
                    &self.value
                )
            })
            .filter(|chan| chan.get(&self.prop).map_or(false, |v| *v == self.value))
            .collect()
    }
}
pub struct TransformChain<Chan, I: IntoIterator<Item = Chan>>(
    Vec<Box<dyn ChannelListTransformer<Chan, I>>>,
);
impl<Chan, I: IntoIterator<Item = Chan>> Default for TransformChain<Chan, I> {
    fn default() -> TransformChain<Chan, I> {
        TransformChain(Vec::new())
    }
}
impl<Chan, I: IntoIterator<Item = Chan>> TransformChain<Chan, I> {
    pub fn new(tran: impl ChannelListTransformer<Chan, I> + 'static) -> TransformChain<Chan, I> {
        TransformChain(vec![Box::new(tran)])
    }
    #[cfg(test)]
    fn add(
        mut self,
        tran: impl ChannelListTransformer<Chan, I> + 'static,
    ) -> TransformChain<Chan, I> {
        self.0.push(Box::new(tran));
        self
    }
    pub fn process_all_channels(self, channel_list: Vec<Chan>) -> Vec<Chan> {
        let mut chan_vec: Vec<Chan> = channel_list;
        for transf in self.0 {
            chan_vec = (*transf).transform_channel_list(chan_vec)
        }
        chan_vec
    }
}

/*
*/

#[cfg(test)]
mod tests {
    use super::ChannelListTransformer;
    use super::FilteringTransformer;
    use super::GetProperty;
    use super::TransformChain;
    struct PassiveTransformer {}
    impl ChannelListTransformer<Chan, Vec<Chan>> for PassiveTransformer {
        fn transform_channel_list(&self, chans: Vec<Chan>) -> Vec<Chan> {
            chans
        }
    }
    fn process_chans_and_check<T: ChannelListTransformer<Chan, Vec<Chan>>>(
        groups: &[&str],
        transf: TransformChain<Chan, Vec<Chan>>,
        exp: &[&str],
    ) {
        let channels = chans(groups);
        let expected = chans(exp);
        assert_eq!(transf.process_all_channels(channels), expected);
    }

    #[test]
    fn keep_chans_unchanged_if_no_transformer() {
        let chans = ["chan1", "chan2"];
        process_chans_and_check::<PassiveTransformer>(&chans, TransformChain::default(), &chans);
    }
    #[test]
    fn keep_chans_unchanged_if_passive_transformer() {
        let chans = ["chan1", "chan2"];
        process_chans_and_check::<PassiveTransformer>(
            &chans,
            TransformChain::new(PassiveTransformer {}),
            &chans,
        );
    }

    #[test]
    fn keep_no_chan_if_dummy_filter() {
        let chans = ["chan1", "chan2"];
        let transf = FilteringTransformer {
            prop: "".to_string(),
            value: "".to_string(),
        };
        let exp = [];
        let chain: TransformChain<Chan, Vec<Chan>> = TransformChain::new(transf);
        process_chans_and_check::<FilteringTransformer<String>>(&chans, chain, &exp);
    }
    #[derive(Clone, Debug, PartialEq, Eq)]
    struct Chan {
        group: String,
    }
    fn chans(slice: &[&str]) -> Vec<Chan> {
        slice.into_iter().map(|&s: &&str| Chan::new(&s)).collect()
    }
    impl Chan {
        fn new(grp: &str) -> Chan {
            Chan {
                group: grp.to_string(),
            }
        }
    }
    impl GetProperty<String> for Chan {
        fn get(&self, _key: &String) -> Option<&str> {
            Some(&self.group)
        }
    }
    #[test]
    fn keep_chan1_if_filter_on_group_chan1() {
        let chans = ["chan1", "chan2"];
        let transf = FilteringTransformer {
            prop: "".to_string(),
            value: "chan1".to_string(),
        };
        let exp = ["chan1"];
        process_chans_and_check::<FilteringTransformer<String>>(
            &chans,
            TransformChain::new(transf),
            &exp,
        );
    }
    #[test]
    fn apply_all_if_3_transformers() {
        let chans = ["chan1", "chan2", "chan3"];
        let transf1 = FilteringTransformer {
            prop: "".to_string(),
            value: "chan1".to_string(),
        };
        let transf2 = FilteringTransformer {
            prop: "".to_string(),
            value: "chan2".to_string(),
        };
        let transf3 = PassiveTransformer {};
        let exp = [];
        process_chans_and_check::<FilteringTransformer<String>>(
            &chans,
            TransformChain::new(transf1).add(transf2).add(transf3),
            &exp,
        );
    }
}
