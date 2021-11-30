pub trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} , by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} : {}",self.username,self.content)
    }
}


fn main() {
    let tweet = Tweet {
        username: "packt_publish".to_string(),
        content: "Rust by example".to_string(),
        reply: false,
        retweet: false
    };
    println!("{}", tweet.summarize());

    let article = NewsArticle {
        headline: "ruslings".to_string(),
        location: "albania".to_string(),
        author: "yeswanth".to_string(),
        content: "rustlings examples".to_string(),
    };
    println!("{}", article.summarize());
}
