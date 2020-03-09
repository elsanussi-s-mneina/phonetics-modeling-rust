fn main()
{
    println!("Hello, world!");
    welcome();
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
    println!("{}", MENU);
    print_prompt();
}

fn print_prompt()
{
  println!("{}", PROMPT);
}
