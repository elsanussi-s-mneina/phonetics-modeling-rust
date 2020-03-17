mod grapheme;

use crate::grapheme::international_phonetic_alphabet::international_phonetic_alphabet::construct_IPA;
use crate::grapheme::international_phonetic_alphabet::international_phonetic_alphabet::analyze_IPA;

use std::io;

use phon_mod::lib::voiced_phonet;
use phon_mod::lib::devoiced_phonet;


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
        prompt_for_phoneme_to_devoice()
    }
    else
    {
       println!("User selection not handled");
    }
    
    println!("\nProgram terminated normally.\n\n");
}

fn view_english_phoneme_inventory()
{
    println!("View English phoneme inventory not implemented!");
}

fn prompt_for_phoneme_to_voice()
{
    println!("Enter the phoneme you would like to voice:");
    print_prompt();
    
    let mut phoneme = String::new();

    io::stdin().read_line(&mut phoneme)
        .expect("Failed to read user input.");
    
    phoneme = phoneme.trim().to_string();
    println!("{}", construct_IPA(voiced_phonet(analyze_IPA(phoneme))));
}

fn prompt_for_phoneme_to_devoice()
{
    println!("Enter the phoneme you would like to devoice:");
    print_prompt();
    let mut phoneme = String::new();

    io::stdin().read_line(&mut phoneme)
        .expect("Failed to read user input.");
    
    phoneme = phoneme.trim().to_string();
    println!("{}", construct_IPA(devoiced_phonet(analyze_IPA(phoneme))));
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
