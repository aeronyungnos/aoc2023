use std::collections::HashMap;

fn main()  {

    let input = include_str!("../input/15.txt");
    let input_list: Vec<&str> = input.split(",").collect();

    let mut boxes: HashMap<u32, Vec<(&str, u32)>> = HashMap::new();
    for i in 0..=256 {
        boxes.insert(i, Vec::new());
    }

    input_list.iter().for_each(|&element| {
        if element.contains("=") {

            let mut parts = element.split("=");
            let lens: &str = parts.next().unwrap_or_default();
            let focal: u32 = parts.next().unwrap_or_default().parse().unwrap_or(0);
            let lens_focal = (lens, focal);
            let key = calc(lens);
            if let Some(existing_vec) = boxes.get_mut(&key) {
                if let Some(existing_index) = existing_vec.iter().position(|(existing_lens, _)| *existing_lens == lens) {
                    existing_vec[existing_index] = lens_focal;
                } else {
                    existing_vec.push(lens_focal);
                }
            } else {
                boxes.insert(key, vec![lens_focal]);
            }
        } else {
            let lens = element.replace("-", "");
            let key: u32 = calc(&lens);
            boxes.entry(key)
                .and_modify(|existing_vec| {
                    existing_vec.retain(|&(name, _)| name != lens);
                });
        }
    });

    let total_result: i32 = boxes
        .iter()
        .flat_map(|(box_index, vec)| {
            vec.iter().enumerate().map(move |(element_index, (&_, focal))| {
                (*box_index, element_index, focal)
            })
        })
        .map(|(box_index, element_index, focal)| final_sum(&box_index, element_index, *focal))
        .sum();

    println!("{}", total_result)
    
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

fn final_sum(box_index: &u32, element_index: usize, focal: u32) -> i32 {
    ((box_index + 1) * (element_index as u32+1) * focal).try_into().unwrap()
}