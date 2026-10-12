
    // Ask for years of experience
    println!("Enter years of experience:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Input error");

    let input = input.trim();

    // Check that the input contains a valid whole number
    let mut valid = true;
    let mut years: u32 = 0;

    if input.is_empty() {
        valid = false;
    } else {
        for character in input.chars() {
            if !character.is_ascii_digit() {
                valid = false;
            }
        }
    }

    if valid == true {
        years = input.parse().unwrap_or(0);

        if years == 0 {
            valid = false;
        }
    }

    if valid == false {
        println!("Please enter a positive whole number.");
        return;
    }

    // Search the vectors for a matching level
    let mut found = false;

    for i in 0..levels.len() {
        if years >= min_years[i] && years <= max_years[i] {
            let role = if category_index == 0 {
                office_admin[i]
            } else if category_index == 1 {
                academic[i]
            } else if category_index == 2 {
                lawyer[i]
            } else {
                teacher[i]
            };

            println!("\n--- RESULT ---");
            println!("APS Level: {}", levels[i]);
            println!("Job Title: {}", role);

            found = true;
            break;
        }
    }

    if found == false {
        println!("No matching level found in the table.");
        println!("Please check the experience entered.");
    }
