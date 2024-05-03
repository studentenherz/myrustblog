use rss::{Channel, ChannelBuilder, Item, ItemBuilder};

use crate::Config;
use common::{utils::get_summary, Post};

const MAX_SUMMARY_SIZE: usize = 200;

pub fn create_rss_feed(latest_posts: &Vec<Post>, config: &Config) -> Channel {
    let items: Vec<Item> = latest_posts
        .iter()
        .map(|post| {
            ItemBuilder::default()
                .title(post.title.clone())
                .author(post.author.clone())
                .description(get_summary(&post.content, MAX_SUMMARY_SIZE))
                .link(format!("{}/post/{}", config.WEBSITE_URL, &post.slug))
                .pub_date(post.published_at.clone().to_rfc2822())
                .build()
        })
        .collect();

    ChannelBuilder::default()
        .title(config.RSS_TITLE.clone())
        .link(config.WEBSITE_URL.clone())
        .description(config.RSS_DESCRIPTION.clone())
        .items(items)
        .build()
}
