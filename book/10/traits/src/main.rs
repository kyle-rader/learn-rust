pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct BuzzfeedArticle {
    pub author: String,
    pub content: String,
}

impl Summary for BuzzfeedArticle {}

fn main() {
    println!("Hello, traits!");

    let tweet = Tweet {
        username: "john_doe".to_string(),
        content: "something quick and witty".to_string(),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let news = NewsArticle {
        headline: "WWII Ends!".to_string(),
        location: "New York, NY".to_string(),
        author: "Walter Cronkite".to_string(),
        content: "long string here".to_string(),
    };

    println!("1 new news article: {}", news.summarize());

    let buzzfeed_post = BuzzfeedArticle {
        author: "foobar".to_string(),
        content: "the spark notes of NYT article".to_string(),
    };

    println!("1 new Buzzfeed post: {}", buzzfeed_post.summarize());
}
