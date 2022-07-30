use std::collections::HashMap ;

type Table = HashMap<String, Vec<String>>;

fn show(table: &Table){
    for (artist,works) in table {
        println!("works by artist:{}", artist);
        for work in works{
            println!("{}", work);
        }
    }
}

fn display_show_fn() {
    let mut table = Table::new();
    table.insert("id".to_string(), vec!["1".to_string(), "2".to_string()]);
    table.insert("title".to_string(), vec!["Title 1".to_string(), "Title 2".to_string()]);
    show(&table);
    assert_eq!(table["id"][0], "1");  
}

fn print_padova(){
    let mut padova = vec![1,2,3,4,5];
    for x in 5..10{
        let next = padova[x-5] + padova[x - 4];
        padova.push(next);
    }
    println!("{:?}", padova); 
}  //padova is dropped here

fn move_operations(){
    let a = vec!["hello".to_string(), "hi".to_string()];
    let b = a.clone();
    let c = a.clone();
    // let d = a;
    println!("{:?}", a)

}

fn main() {
    print_padova();
    move_operations();
    display_show_fn();
}
