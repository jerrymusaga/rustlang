// use traits::{Summary, Tweet};

pub trait Summary{
    fn summarize(&self) -> String {
        String::from("Read more")
    } 
}

pub struct NewsArticle{
    pub headline: String,
    pub location: String,
    pub content: String,
    pub author: String
}

// impl Summary for NewsArticle{
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

// default impl 
impl Summary for NewsArticle{}

pub struct Tweet{
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet {
    fn summarize(&self) -> String{
        format!("{}: {}", self.username, self.reply)
    }
    
}

fn main() {
    let tweet = Tweet {
        username: String::from("ref finance"),
        content: String::from("A new tweet coming from finance"),
        reply: true,
        retweet: false
    };
    println!("1 new Tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Love is war"),
        location: String::from("Arctic"),
        content: String::from("this is a letter from war"),
        author: String::from("Mike")
    };
    println!("New article available: {}", article.summarize())
}
