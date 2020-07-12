pub mod grapheme_grammar
{

    static EXPONENTIALS: [char; 7]
      = ['ʰ' , 'ʷ' , 'ʲ' , 'ˠ' , 'ˤ' , 'ⁿ' , 'ˡ'];


    /// Whether an IPA character is written above the base line
    /// and to the right of the previous character,
    /// like how exponents of a power are written
    /// in mathematical notation.
    fn is_exponential(character: char) -> bool
    {
        for curr in EXPONENTIALS.iter()
        {
            if *curr == character
            {
                return true;
            }
        }
        false
    }

    /// Whether a diacritic goes above
    /// the character it is placed on.
    fn is_diacritic_above(character: char) -> bool
    {
        character == '̊'
    }

    /// Whether a diacritic goes below
    /// the character which it is placed on.
    fn is_diacritic_below(character: char) -> bool
    {
        character == '̥'
    }


    /// Whether a character (but not a diacritic)
    /// takes up space
    /// below the imaginary horizontal line
    /// on which it is written.
       
    /// This could be useful later for determining
    /// where to put diacritics so that
    /// they are readable.

    static ASCENDERS: [char; 37] =
      ['b', 't', 'd', 'k', 'ʔ', 'f', 'θ', 'ð', 'ħ', 'ʕ', 'h', 'ɦ', 'ɬ', 'l', 'ʎ',
      'ʘ', 'ɓ', 'ǀ', 'ɗ', 'ǃ', 'ǂ', 'ɠ', 'ʄ', 'ǁ', 'ʛ', 'ɺ', 'ʢ', 'ʡ', 'ɤ', 'ʈ', 'ɖ',
      'ɸ', 'β', 'ʃ', 'ɮ', 'ɭ', 'ɧ']
      ;


    fn is_ascender(character: char) -> bool
    {
        ASCENDERS.iter().any(|&curr| curr == character)
    }
    
    static DESCENDERS: [char; 28] =
      ['p', 'ɟ', 'g', 'q', 'ɱ', 'ɽ', 'ʒ', 'ʂ', 'ʐ', 'ç', 'ʝ', 'ɣ', 'χ', 'ɻ', 'j',
       'ɰ', 'ɥ', 'y', 'ɳ', 'ɲ', 'ʈ', 'ɖ', 'ɸ', 'β', 'ʃ', 'ɮ', 'ɭ', 'ɧ'];


    /// Whether a character (but not a diacritic)
    /// takes up space
    /// below the imaginary horizontal line
    /// on which it is written.
 
    /// This could be useful later for determining
    /// where to put diacritics so that
    /// they are readable.
    fn is_descender(character: char) -> bool
    {
        DESCENDERS.iter().any(|&curr| curr == character)
    }


}