
fn main()  {

    let input = include_str!("../input/15.txt");
    let input_list: Vec<&str> = input.split(",").collect();
    let count: Vec<u32> = input_list.iter().map(|&seq| calc(seq)).collect();
    let sum: u32 = count.iter().sum();
    println!("{:?}", sum);

}

fn calc(seq: &str) -> u32{
    let mut count:u32 = 0;
    let ords: Vec<u32> = seq.chars().map(|y| y as u32).collect();
    ords.iter().for_each(|&element| {
        count = count + element;
        count = count * 17;
        count = count % 256;
    });
    count
}