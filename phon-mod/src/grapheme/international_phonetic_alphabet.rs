pub mod international_phonetic_alphabet
{

    pub struct IPAText(String);
      // For storing text meant to be interpreted as International phonetic alphabet


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
        ASCENDERS.into_iter().any(|&curr| curr == character)
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
        DESCENDERS.into_iter().any(|&curr| curr == character)
    }

    // See: https://www.internationalphoneticassociation.org/sites/default/files/IPA_Kiel_2015.pdf
    // For the source of this information..

    // CONSONANTS (PULMONIC)
    static CONSONANTS_PULMONIC: [char; 59] =
      [ 'p', 'b',                     't', 'd',           'ʈ', 'ɖ', 'c', 'ɟ', 'k', 'g', 'q', 'ɢ',           'ʔ'      // Plosive
      ,      'm',      'ɱ',                'n',                'ɳ',      'ɲ',      'ŋ',      'ɴ'                     // Nasal
      ,      'ʙ',                          'r',                                              'ʀ'                     // Trill
      ,                'ⱱ',                'ɾ',                'ɽ'                                                   // Tap or Flap
      , 'ɸ', 'β', 'f', 'v', 'θ', 'ð', 's', 'z', 'ʃ', 'ʒ', 'ʂ', 'ʐ', 'ç', 'ʝ', 'x', 'ɣ', 'χ', 'ʁ', 'ħ', 'ʕ', 'h', 'ɦ'  // Fricative
      ,                     'ɬ', 'ɮ'                                                                                 // Lateral fricative
      ,                'ʋ',      'ɹ',      'ɻ',                          'j',       'ɰ'                              // Approximant
      ,                          'l',      'ɭ',                          'ʎ',       'ʟ'                              // Lateral approximant
      ];


    static CONSONANTS_PULMONIC_TABLE: [[char; 22]; 8] =
     [[ 'p', 'b', ' ', ' ', ' ', ' ', 't', 'd', ' ', ' ', 'ʈ', 'ɖ', 'c', 'ɟ', 'k', 'g', 'q', 'ɢ', ' ', ' ', 'ʔ', ' '] // Plosive
     ,[ ' ', 'm', ' ', 'ɱ', ' ', ' ', ' ', 'n', ' ', ' ', ' ', 'ɳ', ' ', 'ɲ', ' ', 'ŋ', ' ', 'ɴ', ' ', ' ', ' ', ' '] // Nasal
     ,[ ' ', 'ʙ', ' ', ' ', ' ', ' ', ' ', 'r', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', 'ʀ', ' ', ' ', ' ', ' '] // Trill
     ,[ ' ', ' ', ' ', 'ⱱ', ' ', ' ', ' ', 'ɾ', ' ', ' ', ' ', 'ɽ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '] // Tap or Flap
     ,[ 'ɸ', 'β', 'f', 'v', 'θ', 'ð', 's', 'z', 'ʃ', 'ʒ', 'ʂ', 'ʐ', 'ç', 'ʝ', 'x', 'ɣ', 'χ', 'ʁ', 'ħ', 'ʕ', 'h', 'ɦ']  // Fricative
     ,[ ' ', ' ', ' ', ' ', ' ', ' ', 'ɬ', 'ɮ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '] // Lateral fricative
     ,[ ' ', ' ', ' ', 'ʋ', ' ', ' ', ' ', 'ɹ', ' ', ' ', ' ', 'ɻ', ' ', 'j', ' ', 'ɰ', ' ', ' ', ' ', ' ', ' ', ' '] // Approximant
     ,[ ' ', ' ', ' ', ' ', ' ', ' ', ' ', 'l', ' ', ' ', ' ', 'ɭ', ' ', 'ʎ', ' ', 'ʟ', ' ', ' ', ' ', ' ', ' ', ' '] // Lateral approximant
     ];

    static CONSONANTS_NON_PULMONIC: [char; 10] =
    // Clicks   Voiced implosives
     [ 'ʘ',     'ɓ' // Bilabial
     , 'ǀ', /* Dental */     'ɗ' // Dental/alveolar
     , 'ǃ', /*  (Post)alveolar */  'ʄ'
     , 'ǂ',  'ɠ'
     , 'ǁ',  'ʛ'
     ];

    static OTHER_SYMBOLS: [char; 10] =
      ['ʍ',  'ɕ'
      ,'w',  'ʑ'
      ,'ɥ',  'ɺ'
      ,'ʜ',  'ɧ'
      ,'ʢ'
      ,'ʡ'
      ];

    static VOWELS: [char; 28] =
      ['i', 'y',   'ɨ', 'ʉ',   'ɯ', 'u'   // Close
      ,'ɪ', 'ʏ',            'ʊ'
      ,'e', 'ø',   'ɘ', 'ɵ',   'ɤ', 'o'   // Close-mid
      ,               'ə'
      ,'ɛ', 'œ',   'ɜ', 'ɞ',   'ʌ', 'ɔ'   // Open-mid
      , 'æ',           'ɐ'
      , 'a', 'ɶ',              'ɑ', 'ɒ'  // Open
      ];

    static SUPRASEGMENTALS: [char; 9] =
      [ 'ˈ'   // Primary stress
      , 'ˌ'   // Secondary stress
      , 'ː'   // Long
      , 'ˑ'   // Half long

      , '̆'    // Extra short
      , '|'   // Minor (foot) group
      , '‖'   // Major (intonation) group
      , '.'   // Syllable break
      , '‿'   // Linking (absence of a break
      ];


    static TONE_AND_WORD_ACCENTS: [char; 19] =
      // Level
      [ '˥', '̋'  // Extra high
      , '˦', '́'  // High
      , '˧', '̄'  // Mid
      , '˨', '̀'  // Low
      , '˩', '̏'  // Extra low
      ,      'ꜜ'  // Downstep
      ,      'ꜛ'  // Upstep

      // Countour
      , '̌' // Rising
      , '̂' // Falling
      , '᷄' // High rising
      , '᷅' // Low rising
      , '᷈' // Rising-falling
      , '↗' // Global rise
      , '↘' // Global fall
      ];

    static DIACRITICS_AND_SUPERSEGMENTALS: [char; 28] =
      [ 'ʰ'  // Aspirated
      , 'ʷ'  // Labialised
      , 'ʲ'  // Palatalised
      , 'ˠ'  // Velarised
      , 'ˤ'  // Pharyngealised
      , 'ⁿ'  // Pre/post nasalised
      , 'ˡ'  // Lateral release

      , '˞'  // Rhoticity
      , 'ʼ'  // Ejective
      , '̚'   // No audible release

      , '̩'   // Syllabic
      , '̯'   // Non-syllabic
      , '̰'   // Creaky voiced
      , '̥'   // Voiceless
      , '̬'   // Voiced
      , '̤'   // Breathy voiced
      , '̊'   // Voiceless (diacritic placed above symbol with descender)
      , '̍'   // Syllabic (diacritic placed above)
      , '̪'   // Dental
      , '̺'   // Apical
      , '̻'   // Laminal
      , '̼'   // Linguolabial
      , '.'  // Closer variety/Fricative
      , '̃'   // Nasalised
      , '̈'   // Centralised
      , '̽'   // Mid centralised
      , '̆'   // Extra short
      , '̇'    // Palatalization/Centralization
      ];
}
