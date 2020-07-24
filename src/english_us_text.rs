pub mod english_us_text
{
    pub(crate) static USER_INPUT_VIEW_ENGLISH_PHONEME_INVENTORY: &str = "1";
    pub(crate) static USER_INPUT_MAKE_A_PHONEME_VOICED: &str = "2";
    pub(crate) static USER_INPUT_MAKE_A_PHONEME_UNVOICED: &str = "3";

    pub(crate) static THE_USER_SELECTED: &str = "The user selected:";
    pub(crate) static USER_SELECTION_NOT_HANDLED: &str = "User selection not handled";

    pub(crate) static PROGRAM_TERMINATED_NORMALLY_MESSAGE: &str = "\nProgram terminated normally.\n\n";

    pub(crate) static VIEW_ENGLISH_PHONEME_INVENTORY_NOT_IMP: &str = "View English phoneme inventory not implemented!";
    pub(crate) static PHONEME_TO_DEVOICE_MESSAGE: &str = "Enter the phoneme you would like to voice:";
    pub(crate) static FAILED_TO_READ_USER_INPUT: &str = "Failed to read user input.";
    pub(crate) static PHONEME_TO_VOICE_MESSAGE: &str = "Enter the phoneme you would like to devoice:";
    pub(crate) static MENU: &str = "What do you want to accomplish?

1) view the English phoneme inventory (as IPA graphemes).
2) make a phoneme voiced.
3) make a phoneme unvoiced.

Enter the number representing your selection below, after the prompt, and press enter/return.\n\n\n";

    pub(crate) static PROMPT: &str = "(PROMPT:) ";
    pub(crate) static PLEASE_READ_README_MESSAGE: &str = "Please read README.md file for instructions on how to use.";

}