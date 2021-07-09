fn main() {
    let mut v = vec![1,2,3,4,5];
    let first = &v[0];
    
    let third = v.get(2);
    println!("{}, {:?}", first, third);

    let some_text = Some(String::from("hello"));
    let text = some_text.clone().unwrap();

    let text_length = some_text.as_ref().map(|s| s.len()).unwrap();

    println!("{:?} : {} is length {}", some_text, text, text_length);

 /* // vec : 練習問題
    // 練習問題  1
    // 整数のリストが与えられ、
    // mean(ベクタを使って平均値)、
    // median(ソートされた時に真ん中に来る値)、
    // mode(最も頻繁に出現する値; ハッシュマップがここでは有効活用できるでしょう)
    // を返してください。
 
    let list = vec![4, 76, 3, 31, 23, 89, 8, 3, 6];

    println!("vec: {:?}", list);
    println!("mean: {}", mean(&list));
    println!("median: {}", median(&list));
    println!("mode: {}", mode(&list));

    fn mean(list: &Vec<i32>) -> f64 {
        let mut sum = 0.0;
        for i in list {
            sum += *i as f64;
        }
        sum / list.len() as f64
    }

    fn median(list: &Vec<i32>) -> i32 {
        let mut sort_list = Vec::new();
        for i in list {
            sort_list.push(i);
        }
        sort_list.sort();
        let mid = sort_list[list.len() as usize / 2];
        *mid
    }

    fn mode(list: &Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for i in list {
            let cunt = map.entry(i).or_insert(0);
            *cunt += 1;
        }
        let mode = map.iter().max_by(|x, y| x.1.cmp(&y.1)).unwrap();
        **mode.0 as i32
    }
*/
    /* VEC型について
    let mut v = vec![100, 32, 57];

    v.pop();

    for i in &mut v {
        *i += 50;
        println!("{}", i)
    }
    */

    /* String型について
    let s1 = String::from("hello");
    let s2 = String::from(".....");
    let s3 = String::from("world");
    let len = s1.len();

    let s = format!("{} : {} : {}", s1, s2, s3);

    println!("{}", s1);

    println!("{}, len: {}", s, len);
    */

    /* HashMapについて*/
    // use std::collections::HashMap;

    // let teams = vec![String::from("blue"), String::from("yellow")];
    // let initial_scores = vec![10, 50];

    // let score: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // let mut map = HashMap::new();
    // map.insert(String::from("AAA"), String::from("aaa"));
    // map.insert(String::from("BBB"), String::from("bbb"));
    // map.insert(String::from("CCC"), String::from("ccc"));

    // let s = map.get("AAA");

    // println!("map :{:?}", map);
    // println!("s :{:?}", s);

    // for (key, value) in &map {
    //     println!("{}&{}", key, value);
    // }

    // let text = "hello world wonderful world";

    // let mut map = HashMap::new();

    // for word in text.split_whitespace() {
    //     let count = map.entry(word).or_insert(0);
    //     *count += 1;
    // }

    // println!("{:?}", map);
}
