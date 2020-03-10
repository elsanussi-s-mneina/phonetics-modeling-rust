use std::io;

fn main()
{
    welcome();
    println!("{}", MENU);
    print_prompt();

    let mut selection = String::new();

    io::stdin().read_line(&mut selection)
        .expect("Failed to read user input.");
    
    selection = selection.trim().to_string();
    
    println!("The user selected: {}\n", selection);
    
    
    if selection == "1"
    {
        view_english_phoneme_inventory();
    }
    else if selection == "2"
    {
        prompt_for_phoneme_to_voice();
    }
    else if selection == "3"
    {
        prompt_for_phoneme_to_unvoice()
    }
    else
    {
       println!("User selection not handled");
    }
}

fn view_english_phoneme_inventory()
{
    println!("View English phoneme inventory not implemented!");
}

fn prompt_for_phoneme_to_voice()
{
    println!("Prompt for phoneme to voice not implemented!");
}

fn prompt_for_phoneme_to_unvoice()
{
    println!("Prompt for phoneme to unvoice not implemented!");
}

static MENU: &str = "What do you want to accomplish?

1) view the English phoneme inventory (as IPA graphemes).
2) make a phoneme voiced.
3) make a phoneme unvoiced.

Enter the number representing your selection below, after the prompt, and press enter/return.\n\n\n";

static PROMPT: &str = "(PROMPT:) ";

fn welcome()
{
    println!("Please read README.md file for instructions on how to use.");
}

fn print_prompt()
{
  println!("{}", PROMPT);
}
