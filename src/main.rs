use std::fs;

struct Elf {
    m_id: u32,
    m_foot_items: Vec<u32>,
    m_calories: u32
}

fn main() {
    let input = fs::read_to_string("caloriesInput.txt").unwrap();

    let mut elfs: Vec<Elf> = Vec::new();
    let mut index = 0;
    let mut food_items:Vec<u32> = Vec::new();
    let mut calories: u32 = 0;
    let mut max_cals:u32 = 0;
    let mut max_carrying_elf_id = 0;
    let mut found_whitespaces = 0;

    let mut number:String = "".to_string();

    for element in input.chars() {

        if element.is_whitespace() {
            found_whitespaces = found_whitespaces + 1;

            match found_whitespaces {
                // Found new number
                1 => {
                    food_items.push(number.parse::<u32>().unwrap());
                    calories = calories + number.parse::<u32>().unwrap();
                    number.clear();
                },
                // Skip this step
                2|3 => {
                    continue;
                },
                // Create new elf
                4 => {
                    index = index + 1;
                    let elf = Elf {
                        m_id: index,
                        m_foot_items: food_items.clone(),
                        m_calories: calories,
                    };
    
                    if elf.m_calories > max_cals {
                        max_cals = elf.m_calories;
                        max_carrying_elf_id = elf.m_id;
                    }
        
                    elfs.push(elf);
        
                    food_items.clear();
                    calories = 0;
                },
                // Should not happen
                _ => {
                    print!("Should not happen.")
                }

            }
        }

        else {
            number.push(element);
            found_whitespaces = 0;
        }
    }

    print!("Max calories: {}", max_cals);
}