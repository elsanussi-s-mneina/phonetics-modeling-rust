mod grapheme;

use crate::grapheme::international_phonetic_alphabet::international_phonetic_alphabet::
    {voiced_transcription, devoiced_transcription};
mod english_us_text;
use crate::english_us_text::english_us_text::{USER_INPUT_VIEW_ENGLISH_PHONEME_INVENTORY,
                                              USER_INPUT_MAKE_A_PHONEME_VOICED,
                                              USER_INPUT_MAKE_A_PHONEME_UNVOICED,
                                              FAILED_TO_READ_USER_INPUT,
                                              MENU,
                                              THE_USER_SELECTED,
                                              USER_SELECTION_NOT_HANDLED,
                                              PROGRAM_TERMINATED_NORMALLY_MESSAGE,
                                              VIEW_ENGLISH_PHONEME_INVENTORY_NOT_IMP,
                                              PHONEME_TO_DEVOICE_MESSAGE,
                                              PHONEME_TO_VOICE_MESSAGE,
                                              PLEASE_READ_README_MESSAGE,
                                              PROMPT};
use std::io;




fn main()
{
    welcome();
    println!("{}", MENU);
    print_prompt();

    let mut selection = String::new();

    io::stdin().read_line(&mut selection)
        .expect(FAILED_TO_READ_USER_INPUT);
    
    selection = selection.trim().to_string();
    
    println!("{} {}\n", THE_USER_SELECTED, selection);


    if selection == USER_INPUT_VIEW_ENGLISH_PHONEME_INVENTORY
    {
        view_english_phoneme_inventory();
    }
    else if selection == USER_INPUT_MAKE_A_PHONEME_VOICED
    {
        prompt_for_phoneme_to_voice();
    }
    else if selection == USER_INPUT_MAKE_A_PHONEME_UNVOICED
    {
        prompt_for_phoneme_to_devoice()
    }
    else
    {
       println!("{}", USER_SELECTION_NOT_HANDLED);
    }
    
    println!("{}", PROGRAM_TERMINATED_NORMALLY_MESSAGE);
}


fn view_english_phoneme_inventory()
{
    println!("{}", VIEW_ENGLISH_PHONEME_INVENTORY_NOT_IMP);
}

fn prompt_for_phoneme_to_voice()
{
    println!("{}", PHONEME_TO_DEVOICE_MESSAGE);
    print_prompt();
    
    let mut phoneme = String::new();

    io::stdin().read_line(&mut phoneme)
        .expect(FAILED_TO_READ_USER_INPUT);
    
    phoneme = phoneme.trim().to_string();
    println!("{}", voiced_transcription(phoneme));
}

fn prompt_for_phoneme_to_devoice()
{
    println!("{}", PHONEME_TO_VOICE_MESSAGE);
    print_prompt();
    let mut phoneme = String::new();

    io::stdin().read_line(&mut phoneme)
        .expect(FAILED_TO_READ_USER_INPUT);
    
    phoneme = phoneme.trim().to_string();
    println!("{}", devoiced_transcription(phoneme));
}


fn welcome()
{
    println!("{}", PLEASE_READ_README_MESSAGE);
}

fn print_prompt()
{
  println!("{}", PROMPT);
}
