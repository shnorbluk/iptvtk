fn filter_on_group<'i, I: Iterator<Item = IptvChannel> + 'i>(
    chans: I,
    group_name: &'i str,
) -> impl Iterator<Item = IptvChannel> + 'i {
    chans.filter(move |chan| chan.group_name == group_name)
}
#[derive(Eq, PartialEq, Debug, Clone)]
struct IptvChannel {
    group_name: String,
}

#[cfg(test)]
mod tests {
    use super::{filter_on_group, IptvChannel};
    fn groups_to_chans(groups: &[&str]) -> Vec<IptvChannel> {
        let iter = groups.to_vec().into_iter().map(|group: &str| IptvChannel {
            group_name: group.to_string(),
        });
        iter.collect()
    }

    fn check_filter(input: &[&str], group_name: &str, expected_output: &[&str]) {
        let chans = groups_to_chans(input);
        let filtered = filter_on_group(chans.into_iter(), group_name).collect::<Vec<_>>();
        let expected = groups_to_chans(expected_output);
        assert_eq!(filtered, expected);
    }
    #[test]
    fn filter_on_group_fr_returns_nothing_if_group_is_en() {
        check_filter(&["en"], "fr", &[])
    }
    #[test]
    fn filter_on_group_fr_returns_channel_if_group_is_fr() {
        check_filter(&["fr"], "fr", &["fr"])
    }

    #[test]
    fn filter_on_group_returns_the_relevent_one_of_two() {
        check_filter(&["en", "fr"], "fr", &["fr"])
    }
}
