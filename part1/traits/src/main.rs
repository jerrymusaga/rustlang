// use traits::{Summary, Tweet};

pub trait Summary{
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more...{} )", self.summarize_author())
    } 
}

pub struct NewsArticle{
    pub headline: String,
    pub location: String,
    pub content: String,
    pub author: String
}

impl Summary for NewsArticle{
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.summarize_author(), self.location)
    }
}

// default impl 
// impl Summary for NewsArticle{}

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
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    
}

pub fn notify<T: Summary, U: Summary>(item1: &T, item2: &U) {
    println!("Breaking news: {}\n {}", item1.summarize(), item2.summarize());
}

struct Animal {
    name: String,
    age: u32
}

//impl inbuilt trait
impl ToString for Animal {
    fn to_string(&self) -> String {
        format!("{} is {} years old",self.name,self.age)
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
    println!("New article available: {}", article.summarize());

    let animal = Animal {
        name: String::from("Cow"),
        age: 45
    };
    println!("New Animal: {}",animal.to_string());

    notify(&tweet, &article);
}
