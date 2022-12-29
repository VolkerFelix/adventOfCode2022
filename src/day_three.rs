use std::fs;
use std::collections::HashMap;
use std::path::PrefixComponent;

static ASCII: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 
    'z', 'A', 'B', 'C', 'D',
    'E', 'F', 'G', 'H', 'I', 
    'J', 'K', 'L', 'M', 'N', 
    'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X',
    'Y', 'Z',
];

#[derive(Default)]
struct Rucksack {
    m_compartments: [Vec<char>; 2]
}

pub fn dayThree() {

    let mut priorities: HashMap<char, usize> = HashMap::new();

    for (i, element) in ASCII.iter().enumerate() {
        priorities.insert(*element, i+1);
    }

    let input = fs::read_to_string("rucksackInput.txt").unwrap();

    let mut input_items = Vec::new();
    //let mut rucksacks = Vec::new();
    let mut found_white_spaces = 0;
    let mut prio_sum = 0;

    for element in input.chars() {
        // Warning: This is OS dependent
        if element.is_whitespace() {
            found_white_spaces = found_white_spaces + 1;
            match found_white_spaces {
                // It's me --> skip
                1 => {
                    continue;
                },
                // New rucksack
                2 => {
                    // Devide found items in half and place into compartments
                    let compartment1 = input_items[0..input_items.len()/2].as_ref();
                    let compartment2 = input_items[input_items.len()/2..input_items.len()].as_ref();
                    /*
                    let new_rucksack = Rucksack {
                        m_compartments: [compartment1.to_vec(), compartment2.to_vec()]
                    };
                    rucksacks.push(new_rucksack);
                    let mut same_items = Vec::new();
                    */
                    // Search for same items and calc prio sum
                    for item1 in compartment1 {
                        for item2 in compartment2 {
                            if item1 == item2 {
                                prio_sum = prio_sum + priorities.get(item1).unwrap();
                                break;
                                //same_items.push(*item1);

                            }
                        }
                    }
                    input_items.clear();
                },
                _ => {
                    panic!("Should not happen!")
                }
            }
        }

        else {
            found_white_spaces = 0;
            input_items.push(element);
        }
    }

    print!("Total prio sum: {}", prio_sum);

}