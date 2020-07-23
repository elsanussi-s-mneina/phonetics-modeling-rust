pub mod international_phonetic_alphabet
{
    use phon_mod::lib::Phonet;
    use phon_mod::lib::Phonet::*;
    use phon_mod::lib::VocalFolds;
    use phon_mod::lib::VocalFolds::*;
    use phon_mod::lib::Place;
    use phon_mod::lib::Place::*;
    use phon_mod::lib::Manner;
    use phon_mod::lib::Manner::*;
    use phon_mod::lib::Airstream::*;
    use phon_mod::lib::Height::*;
    use phon_mod::lib::Backness::*;
    use phon_mod::lib::Rounding::*;
    use phon_mod::lib::voiced_phonet;
    use phon_mod::lib::devoiced_phonet;
    use phon_mod::lib::spirantized_phonet;


    // See: https://www.internationalphoneticassociation.org/sites/default/files/IPA_Kiel_2015.pdf
    // For the source of this information..

    // CONSONANTS (PULMONIC)
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

    fn analyze_manner_transcription(x: char) -> (Manner, usize)
    {
        if CONSONANTS_PULMONIC_TABLE[0].iter().any(|&elem| elem == x) == true
        {
            return (Plosive, 0);
        }
        else if CONSONANTS_PULMONIC_TABLE[1].iter().any(|&elem| elem == x) == true
        {
            return (Nasal, 1);
        }
        else if CONSONANTS_PULMONIC_TABLE[2].iter().any(|&elem| elem == x) == true
        {
            return (Trill, 2);
        }
        else if CONSONANTS_PULMONIC_TABLE[3].iter().any(|&elem| elem == x) == true
        {
            return (TapOrFlap, 3);
        }
        else if CONSONANTS_PULMONIC_TABLE[4].iter().any(|&elem| elem == x) == true
        {
            return (Fricative, 4);
        }
        else if CONSONANTS_PULMONIC_TABLE[5].iter().any(|&elem| elem == x) == true
        {
            return (LateralFricative, 5);
        }
        else if CONSONANTS_PULMONIC_TABLE[6].iter().any(|&elem| elem == x) == true
        {
            return (Approximant, 6);
        }
        else if CONSONANTS_PULMONIC_TABLE[7].iter().any(|&elem| elem == x) == true
        {
            return (LateralApproximant, 7);
        }
        else
        {
            return (LateralApproximant, 7); // Not right, but will have to work for now. // TODO: Fix this.
        }
    }
    
    fn analyze_place_transcription(col_index: usize) -> Place
    {
        let col_names = [Bilabial, LabioDental, Dental, Alveolar, PostAlveolar, Retroflex, Palatal, Velar, Uvular, Pharyngeal, Glottal];
        col_names[col_index / 2]
    }


    fn analyze_transcriptionv2(x: char) -> Phonet
    {
        let (manner1, row_index) = analyze_manner_transcription(x);
        let col_index = CONSONANTS_PULMONIC_TABLE[row_index].iter().position(|&elem| elem == x).unwrap();
        let voicing   = col_index_to_voicing(col_index);
        let place1    = analyze_place_transcription(col_index);
        Consonant {vocal_folds: voicing, place: place1, manner: manner1, airstream: PulmonicEgressive}
    }


    fn col_index_to_voicing(col_index: usize) -> VocalFolds
    {
        if col_index % 2 == 0
        {
            Voiceless
        }
        else
        {
            Voiced
        }
    }
    
    fn voicing_to_col_index_offset(vocal_folds: VocalFolds) -> usize
    {
        match vocal_folds
        {
            Voiceless => 0,
            Voiced    => 1,
            VoicelessAspirated => 0,
            VoicedAspirated => 1,
            CreakyVoiced => 1,
            UnmarkedVocalFolds => 0,
        }
    }
    
    fn manner_to_row_index(manner: Manner) -> usize
    {
        let row_names: [Manner; 8] = [Plosive, Nasal, Trill, TapOrFlap, Fricative, LateralFricative, Approximant, LateralApproximant];
        row_names.iter().position(|&elem| elem == manner).unwrap()
    }
    

    fn place_to_half_col_index(place: Place) -> usize
    {
      let col_names: [Place; 11] = [Bilabial, LabioDental, Dental, Alveolar, PostAlveolar, Retroflex, Palatal, Velar, Uvular, Pharyngeal, Glottal];
      col_names.iter().position(|&elem| elem == place).unwrap()
    }
    
    fn voicing_and_place_to_col_index(voicing: VocalFolds, place: Place) -> usize
    {
      (2 * place_to_half_col_index(place)) + voicing_to_col_index_offset(voicing)
    }


    // | This function will allow us to convert an IPA symbol
    // | to its analyzed form (its phonetic features)
    // Currently, only the consonants (pulmonic) in the 2005 IPA chart are included.
    pub fn analyze_transcription(text: String) -> Phonet
    {
        match text.as_str()
        {
            // Affricates
            "t͡ʃ" => Consonant {vocal_folds: Voiceless, place: PostAlveolar, manner: Affricate, airstream: PulmonicEgressive},
            "d͡ʒ" => Consonant {vocal_folds: Voiced   , place: PostAlveolar, manner: Affricate, airstream: PulmonicEgressive},
            // We should probably enforce use of the tie-bar underneath, otherwise
            // it would not be deterministic to determine whether two graphemes here
            // represent affricates or a plosive followed by a fricative.


            // Other Consonants:
            
            "w" => Consonant{vocal_folds: Voiced            , place: LabialVelar   , manner: Approximant   , airstream: PulmonicEgressive},
            "ʍ" => Consonant{vocal_folds: Voiceless         , place: LabialVelar   , manner: Fricative     , airstream: PulmonicEgressive},
            "ɥ" => Consonant{vocal_folds: Voiced            , place: LabialPalatal , manner: Approximant   , airstream: PulmonicEgressive},
            "ʜ" => Consonant{vocal_folds: Voiceless         , place: Epiglottal    , manner: Fricative     , airstream: PulmonicEgressive},
            "ʢ" => Consonant{vocal_folds: Voiced            , place: Epiglottal    , manner: Fricative     , airstream: PulmonicEgressive},
            
            // Under the Other Symbols part of the IPA chart:
            // Is the epiglottal plosive voiceless? The IPA chart does not specify.

            "ʡ" => Consonant{vocal_folds: Voiceless         , place: Epiglottal    , manner: Plosive       , airstream: PulmonicEgressive},
            "ɕ" => Consonant{vocal_folds: Voiceless         , place: AlveoloPalatal, manner: Fricative     , airstream: PulmonicEgressive},
            "ʑ" => Consonant{vocal_folds: Voiced            , place: AlveoloPalatal, manner: Fricative     , airstream: PulmonicEgressive},
            "ɺ" => Consonant{vocal_folds: Voiced            , place: Alveolar      , manner: LateralFlap   , airstream: PulmonicEgressive},
            
            // We cannot handle the ɧ (simultaneous ʃ and x) because
            // we did not define our data types to handle it yet.
            // In any case, here is some pseudocode for it:
            // analyzeIPA "ɧ" = simultaneous (analyzeIPA "ʃ") (analyzeIPA "x")

            "ʘ" => Consonant{vocal_folds: UnmarkedVocalFolds, place: Bilabial      , manner: UnmarkedManner, airstream: Click            },
            "ǀ" => Consonant{vocal_folds: UnmarkedVocalFolds, place: Dental        , manner: UnmarkedManner, airstream: Click            },
            "ǃ" => Consonant{vocal_folds: UnmarkedVocalFolds, place: Alveolar      , manner: UnmarkedManner, airstream: Click            },   // Or it could be PostAlveolar.
            "ǂ" => Consonant{vocal_folds: UnmarkedVocalFolds, place: PalatoAlveolar, manner: UnmarkedManner, airstream: Click            },
            "ǁ" => Consonant{vocal_folds: UnmarkedVocalFolds, place: Alveolar      , manner: Lateral       , airstream: Click            },
            "ɓ" => Consonant{vocal_folds: Voiced            , place: Bilabial      , manner: UnmarkedManner, airstream: Implosive        },
            "ɗ" => Consonant{vocal_folds: Voiced            , place: Dental        , manner: UnmarkedManner, airstream: Implosive        },  // Or Alveolar
            "ʄ" => Consonant{vocal_folds: Voiced            , place: Palatal       , manner: UnmarkedManner, airstream: Implosive        },
            "ɠ" => Consonant{vocal_folds: Voiced            , place: Velar         , manner: UnmarkedManner, airstream: Implosive        },
            "ʛ" => Consonant{vocal_folds: Voiced            , place: Uvular        , manner: UnmarkedManner, airstream: Implosive        },

            // Close Vowels:
            "i"  => Vowel {height: Close    , backness: Front  , rounding: Unrounded       , vocal_folds:  Voiced},
            "y"  => Vowel {height: Close    , backness: Front  , rounding: Rounded         , vocal_folds:  Voiced},
            "ɨ"  => Vowel {height: Close    , backness: Central, rounding: Unrounded       , vocal_folds:  Voiced},
            "ʉ"  => Vowel {height: Close    , backness: Central, rounding: Rounded         , vocal_folds:  Voiced},
            "ɯ"  => Vowel {height: Close    , backness: Back   , rounding: Unrounded       , vocal_folds:  Voiced},
            "u"  => Vowel {height: Close    , backness: Back   , rounding: Rounded         , vocal_folds:  Voiced},
            // Near-close Vowels:
            "ɪ"  => Vowel {height: NearClose, backness: Front  , rounding: Unrounded       , vocal_folds:  Voiced},
            "ʏ"  => Vowel {height: NearClose, backness: Front  , rounding: Rounded         , vocal_folds:  Voiced},
            "ʊ"  => Vowel {height: NearClose, backness: Back   , rounding: Rounded         , vocal_folds:  Voiced},
            // Close-mid Vowels:
            "e"  => Vowel {height: CloseMid , backness: Front  , rounding: Unrounded       , vocal_folds:  Voiced},
            "ø"  => Vowel {height: CloseMid , backness: Front  , rounding: Rounded         , vocal_folds:  Voiced},
            "ɘ"  => Vowel {height: CloseMid , backness: Central, rounding: Unrounded       , vocal_folds:  Voiced},
            "ɵ"  => Vowel {height: CloseMid , backness: Central, rounding: Rounded         , vocal_folds:  Voiced},
            "ɤ"  => Vowel {height: CloseMid , backness: Back   , rounding: Unrounded       , vocal_folds:  Voiced},
            "o"  => Vowel {height: CloseMid , backness: Back   , rounding: Rounded         , vocal_folds:  Voiced},
            // Mid Vowels:
            "ə"  => Vowel {height: Mid      , backness: Central, rounding: UnmarkedRounding, vocal_folds:  Voiced},
            // Open-mid Vowels:
            "ɛ"  => Vowel {height: OpenMid  , backness: Front  , rounding: Unrounded       , vocal_folds:  Voiced},
            "œ"  => Vowel {height: OpenMid  , backness: Front  , rounding: Rounded         , vocal_folds:  Voiced},
            "ɜ"  => Vowel {height: OpenMid  , backness: Central, rounding: Unrounded       , vocal_folds:  Voiced},
            "ɞ"  => Vowel {height: OpenMid  , backness: Central, rounding: Rounded         , vocal_folds:  Voiced},
            "ʌ"  => Vowel {height: OpenMid  , backness: Back   , rounding: Unrounded       , vocal_folds:  Voiced},
            "ɔ"  => Vowel {height: OpenMid  , backness: Back   , rounding: Rounded         , vocal_folds:  Voiced},
            // Near-open
            "æ"  => Vowel {height: NearOpen , backness: Front  , rounding: Unrounded       , vocal_folds:  Voiced},
            "ɐ"  => Vowel {height: NearOpen , backness: Central, rounding: UnmarkedRounding, vocal_folds:  Voiced},
            // Open Vowels:
            "a"  => Vowel {height: Open     , backness: Front  , rounding: Unrounded       , vocal_folds:  Voiced},
            "ɶ"  => Vowel {height: Open     , backness: Front  , rounding: Rounded         , vocal_folds:  Voiced},
            "ɑ"  => Vowel {height: Open     , backness: Back   , rounding: Unrounded       , vocal_folds:  Voiced},
            "ɒ"  => Vowel {height: Open     , backness: Back   , rounding: Rounded         , vocal_folds:  Voiced},


            x if x.len() == 1  => analyze_transcriptionv2(x[0..1].to_string().chars().next().unwrap()),
            x =>
            {
                // Handle Diacritics:

                if x[1..2] == "̥".to_string()
                {
                    let full_grapheme = analyze_transcription(x[0..1].to_string());
                    
                    match full_grapheme
                    {
                        Consonant {vocal_folds: _, place: place1, manner: manner1, airstream: airstream1}  => Consonant {vocal_folds: Voiceless, place: place1, manner: manner1, airstream: airstream1},
                        Vowel {height: height1, backness: backness1, rounding: rounding1, vocal_folds: _}  => Vowel {height: height1, backness: backness1, rounding: rounding1, vocal_folds: Voiceless},
                    }

                }
                else if x[1..2] == "̬".to_string()
                {
                    let full_grapheme = analyze_transcription(x[0..1].to_string());
                    
                    match full_grapheme
                    {
                            Consonant {vocal_folds: _, place: place1, manner: manner1, airstream: airstream1} => Consonant {vocal_folds: Voiced, place: place1, manner: manner1, airstream: airstream1 },
                            Vowel {height: height1, backness: backness1, rounding: rounding1, vocal_folds: _}  => Vowel {height: height1, backness: backness1, rounding: rounding1, vocal_folds: Voiced },
                    }

                }
                else if x[1..2] == "ʰ".to_string()
                {
                    let full_grapheme = analyze_transcription(String::from(x[0..1].to_string()));
                    match full_grapheme
                    {
                            Consonant {vocal_folds: Voiced   , place: place1, manner: manner1, airstream: airstream1 } => Consonant {vocal_folds: VoicedAspirated   , place: place1, manner: manner1, airstream: airstream1 },
                            Consonant {vocal_folds: Voiceless, place: place1, manner: manner1, airstream: airstream1 } => Consonant {vocal_folds: VoicelessAspirated, place: place1, manner: manner1, airstream: airstream1 },
                            Vowel     {height: height1, backness: backness1, rounding: rounding1, vocal_folds: voice } => Vowel {height: height1, backness: backness1, rounding: rounding1, vocal_folds: voice },
                            // (About the preceding line:) It is strange but we will just do nothing if they give us an aspirated vowel.
                            // since we have no way to represent it in the type system. to do: determine
                            // if the idea of an aspirated vowel makes sense
                            _ => Consonant {vocal_folds: UnmarkedVocalFolds, place: UnmarkedPlace, manner: UnmarkedManner, airstream: UnmarkedAirstream}

                    }
                }
                else
                {
                    Consonant {vocal_folds: UnmarkedVocalFolds, place: UnmarkedPlace, manner: UnmarkedManner, airstream: UnmarkedAirstream}
                    // Not recognized.
                }
            },
        }
    }


    fn construct_unaspirated_pulmonic_egressive(phone_description: Phonet) -> String
    {
        match phone_description
        {
            Consonant{vocal_folds: voicing1, place: place1, manner: manner1, airstream: _} =>
            {
                let row_index = manner_to_row_index(manner1);
                let col_index = voicing_and_place_to_col_index(voicing1, place1);
                (CONSONANTS_PULMONIC_TABLE[row_index][col_index]).to_string()
            },
            _ => String::from(""),
        }
    }


    fn deaspirate(phone_description: Phonet) -> Phonet
    {
        match phone_description
        {
            Consonant {vocal_folds: VoicedAspirated   , place: place1, manner: manner1, airstream: airstream1} => Consonant {vocal_folds: Voiced   , place: place1, manner: manner1, airstream: airstream1},
            Consonant {vocal_folds: VoicelessAspirated, place: place1, manner: manner1, airstream: airstream1} => Consonant {vocal_folds: Voiceless, place: place1, manner: manner1, airstream: airstream1},
            x => x,
        }
    }



    pub fn construct_transcription(phone_description: Phonet) -> String
    {
        match phone_description
        {
            // Affricates
            Consonant { vocal_folds: Voiceless, place: PostAlveolar,  manner: Affricate,  airstream: PulmonicEgressive} => String::from("t͡ʃ"),
            Consonant { vocal_folds: Voiced   , place: PostAlveolar,  manner: Affricate,  airstream: PulmonicEgressive} => String::from("d͡ʒ"),
            Consonant { vocal_folds: Voiceless, place: Bilabial    ,  manner: Affricate,  airstream: PulmonicEgressive} => String::from("p͡ɸ"),
            Consonant { vocal_folds: Voiceless, place: Alveolar    ,  manner: Affricate,  airstream: PulmonicEgressive} => String::from("t͜s"),
            Consonant { vocal_folds: Voiced   , place: Alveolar    ,  manner: Affricate,  airstream: PulmonicEgressive} => String::from("d͡z"),
            Consonant { vocal_folds: Voiceless, place: Velar       ,  manner: Affricate,  airstream: PulmonicEgressive} => String::from("k͡x"),
            Consonant { vocal_folds: Voiceless, place: Uvular      ,  manner: Affricate,  airstream: PulmonicEgressive} => String::from("q͡χ"),
            // The following two lines are commented out, because I am unsure about their place of articulation:
            // Consonant { vocal_folds: Voiceless, place: LabialVelar?                , manner: Affricate, airstream: PulmonicEgressive} => String::from("k͡p"),
            // Consonant { vocal_folds: Voiceless, place: Palatal (or AlveolaPalatal?), manner: Affricate, airstream: PulmonicEgressive} => String::from("c͡ɕ"),
            _ =>
            {
                // If it can represent it as a single character it will
                // return the single character result (i.e. without diacritics),
                // otherwise
                // it will try to represent it in IPA with more than
                // one character
                let simple_result = construct_transcription1(phone_description.clone());
                if simple_result == " "
                {
                    construct_transcription2(phone_description.clone())
                }
                else
                {
                    simple_result
                }
            },
        }
    }


    // Note to Software Developer: the reason there are three
    // functions for constructing the IPA is to prevent
    // infinite recursion. The reason is that
    // if we only had one function, it would // for some
    // cases never halt if it could not find a character
    // to place a diacritic on.

    // | Given an analysis construct an IPA symbol
    // | This function will allow us to convert an analyzed form
    // | to its IPA symbol.
    // | Note this only returns one character without diacritics.
    fn construct_transcription1(phone_description: Phonet) -> String
    {

    // Under the Other Symbols part of the IPA chart:
        match phone_description
        {
            Consonant {vocal_folds: Voiced   , place: LabialVelar  , manner: Approximant, airstream: PulmonicEgressive}              => String::from("w"),
            Consonant {vocal_folds: Voiceless, place: LabialVelar  , manner: Fricative  , airstream: PulmonicEgressive}              => String::from("ʍ"),
            Consonant {vocal_folds: Voiced   , place: LabialPalatal, manner: Approximant, airstream: PulmonicEgressive}              => String::from("ɥ"),
            Consonant {vocal_folds: Voiceless, place: Epiglottal   , manner: Fricative  , airstream: PulmonicEgressive}              => String::from("ʜ"),
            Consonant {vocal_folds: Voiced   , place: Epiglottal   , manner: Fricative  , airstream: PulmonicEgressive}              => String::from("ʢ"),
            Consonant {vocal_folds: Voiceless, place: Epiglottal   , manner: Plosive    , airstream: PulmonicEgressive}              => String::from("ʡ"),
            // Is the epiglottal plosive voiceless? The IPA chart does not specify.

            Consonant {vocal_folds: Voiceless         , place: AlveoloPalatal, manner: Fricative     , airstream: PulmonicEgressive} => String::from("ɕ"),
            Consonant {vocal_folds: Voiced            , place: AlveoloPalatal, manner: Fricative     , airstream: PulmonicEgressive} => String::from("ʑ"),
            Consonant {vocal_folds: Voiced            , place: Alveolar      , manner: LateralFlap   , airstream: PulmonicEgressive} => String::from("ɺ"),

            // We cannot handle the ɧ (simultaneous ʃ and x) because
            // we did not define our data types to handle it yet.
            // constructIPA (simultaneous (analyzeIPA "ʃ") (analyzeIPA "x")) = "ɧ"

            // Other Consonants:
            Consonant {vocal_folds: UnmarkedVocalFolds, place: Bilabial      , manner: UnmarkedManner, airstream: Click            }  => String::from("ʘ"),
            Consonant {vocal_folds: UnmarkedVocalFolds, place: Dental        , manner: UnmarkedManner, airstream: Click            }  => String::from("ǀ"),
            Consonant {vocal_folds: UnmarkedVocalFolds, place: Alveolar      , manner: UnmarkedManner, airstream: Click            }  => String::from("ǃ"), // Or it could be PostAlveolar
            Consonant {vocal_folds: UnmarkedVocalFolds, place: PalatoAlveolar, manner: UnmarkedManner, airstream: Click            }  => String::from("ǂ"),
            Consonant {vocal_folds: UnmarkedVocalFolds, place: Alveolar      , manner: Lateral       , airstream: Click            }  => String::from("ǁ"),
            Consonant {vocal_folds: Voiced            , place: Bilabial      , manner: UnmarkedManner, airstream: Implosive        }  => String::from("ɓ"),
            Consonant {vocal_folds: Voiced            , place: Dental        , manner: UnmarkedManner, airstream: Implosive        }  => String::from("ɗ"),  // Or Alveolar
            Consonant {vocal_folds: Voiced            , place: Palatal       , manner: UnmarkedManner, airstream: Implosive        }  => String::from("ʄ"),
            Consonant {vocal_folds: Voiced            , place: Velar         , manner: UnmarkedManner, airstream: Implosive        }  => String::from("ɠ"),
            Consonant {vocal_folds: Voiced            , place: Uvular        , manner: UnmarkedManner, airstream: Implosive        }  => String::from("ʛ"),

            c @ Consonant {vocal_folds: Voiced, place: _, manner: _, airstream: PulmonicEgressive} => construct_unaspirated_pulmonic_egressive(c),

            c @ Consonant {vocal_folds: VoicedAspirated,  place: _, manner: _, airstream: PulmonicEgressive} => construct_unaspirated_pulmonic_egressive(deaspirate(c)) + "ʰ",
            c @ Consonant {vocal_folds: Voiceless, place: _, manner: _, airstream: PulmonicEgressive} => construct_unaspirated_pulmonic_egressive(c),

            c @ Consonant {vocal_folds: VoicelessAspirated, place: _, manner: _, airstream: PulmonicEgressive} => construct_unaspirated_pulmonic_egressive(deaspirate(c)) + "ʰ",

           // Close Vowels:
           Vowel { height: Close    , backness: Front  , rounding: Unrounded       , vocal_folds: Voiced} => String::from("i"),
           Vowel { height: Close    , backness: Front  , rounding: Rounded         , vocal_folds: Voiced} => String::from("y"),
           Vowel { height: Close    , backness: Central, rounding: Unrounded       , vocal_folds: Voiced} => String::from("ɨ"),
           Vowel { height: Close    , backness: Central, rounding: Rounded         , vocal_folds: Voiced} => String::from("ʉ"),
           Vowel { height: Close    , backness: Back   , rounding: Unrounded       , vocal_folds: Voiced} => String::from("ɯ"),
           Vowel { height: Close    , backness: Back   , rounding: Rounded         , vocal_folds: Voiced} => String::from("u"),

           // Near-close Vowels:
           Vowel { height: NearClose, backness: Front  , rounding: Unrounded       , vocal_folds: Voiced} => String::from("ɪ"),
           Vowel { height: NearClose, backness: Front  , rounding: Rounded         , vocal_folds: Voiced} => String::from("ʏ"),
           Vowel { height: NearClose, backness: Back   , rounding: Rounded         , vocal_folds: Voiced} => String::from("ʊ"), // Close-mid Vowels:
           Vowel { height: CloseMid , backness: Front  , rounding: Unrounded       , vocal_folds: Voiced} => String::from("e"),
           Vowel { height: CloseMid , backness: Front  , rounding: Rounded         , vocal_folds: Voiced} => String::from("ø"),
           Vowel { height: CloseMid , backness: Central, rounding: Unrounded       , vocal_folds: Voiced} => String::from("ɘ"),
           Vowel { height: CloseMid , backness: Central, rounding: Rounded         , vocal_folds: Voiced} => String::from("ɵ"),
           Vowel { height: CloseMid , backness: Back   , rounding: Unrounded       , vocal_folds: Voiced} => String::from("ɤ"),
           Vowel { height: CloseMid , backness: Back   , rounding: Rounded         , vocal_folds: Voiced} => String::from("o"), // Mid Vowels:
           Vowel { height: Mid      , backness: Central, rounding: UnmarkedRounding, vocal_folds: Voiced} => String::from("ə"), // Open-mid Vowels:
           Vowel { height: OpenMid  , backness: Front  , rounding: Unrounded       , vocal_folds: Voiced} => String::from("ɛ"),
           Vowel { height: OpenMid  , backness: Front  , rounding: Rounded         , vocal_folds: Voiced} => String::from("œ"),
           Vowel { height: OpenMid  , backness: Central, rounding: Unrounded       , vocal_folds: Voiced} => String::from("ɜ"),
           Vowel { height: OpenMid  , backness: Central, rounding: Rounded         , vocal_folds: Voiced} => String::from("ɞ"),
           Vowel { height: OpenMid  , backness: Back   , rounding: Unrounded       , vocal_folds: Voiced} => String::from("ʌ"),
           Vowel { height: OpenMid  , backness: Back   , rounding: Rounded         , vocal_folds: Voiced} => String::from("ɔ"), // Near-open
           Vowel { height: NearOpen , backness: Front  , rounding: Unrounded       , vocal_folds: Voiced} => String::from("æ"),
           Vowel { height: NearOpen , backness: Central, rounding: UnmarkedRounding, vocal_folds: Voiced} => String::from("ɐ"), // Open Vowels:
           Vowel { height: Open     , backness: Front  , rounding: Unrounded       , vocal_folds: Voiced} => String::from("a"),
           Vowel { height: Open     , backness: Front  , rounding: Rounded         , vocal_folds: Voiced} => String::from("ɶ"),
           Vowel { height: Open     , backness: Back   , rounding: Unrounded       , vocal_folds: Voiced} => String::from("ɑ"),
           Vowel { height: Open     , backness: Back   , rounding: Rounded         , vocal_folds: Voiced} => String::from("ɒ"),

           _                                                                                              => String::from(" "),
        }

    }

    
    fn construct_transcription2(phonet: Phonet) -> String
    {
        match phonet
        {

            Consonant {vocal_folds: x, place: PostAlveolar, manner: y, airstream: z} =>
              construct_transcription1(Consonant {vocal_folds: x, place: Alveolar, manner: y, airstream: z}) + "̠",  // Add the diacritic for "retracted"


            // If there isn't a symbol, and the consonant we want is voiceless,
            // Just take the symbol for a voiced consonant,
            // and then put that diacritic that means voiceless after.
            // (The following two definitions are intended to implement that)
            // Add the small circle diacritic to consonants to make them voiceless.
            Consonant {vocal_folds: Voiceless, place: x, manner: y, airstream: z} =>
              construct_transcription1(Consonant {vocal_folds: Voiced, place: x, manner: y, airstream: z}) + "̥", // add diacritic for voiceless

            // Add the small circle diacritic to vowels to make them voiceless.
            Vowel {height: x, backness: y, rounding: z, vocal_folds: Voiceless} =>
              construct_transcription1(Vowel {height: x, backness: y, rounding: z, vocal_folds: Voiced}) + "̥",

            // If there is no way to express a voiced consonant in a single
            // grapheme add a diacritic to the grapheme that represents
            // the voiceless counterpart.
            Consonant {vocal_folds: Voiced, place: x, manner: y, airstream: z} =>
              construct_transcription1(Consonant {vocal_folds: Voiceless, place: x, manner: y, airstream: z}) + "̬",

            Vowel {height: x, backness: y, rounding: z, vocal_folds: Voiced} =>
              construct_transcription1(Vowel {height: x, backness: y, rounding: z, vocal_folds: Voiceless}) + "̬",

            _                                                                => String::from("∅"), // This return value ( a symbol representing the empty set)
            // is not a full answer. It really means we don't have an answer.
            // We are only using it here so that we can ignore values we have not programmed
            // yet. We just want it to show that we do not have it.
        }
    }


    pub fn voiced_transcription(x: String) -> String
    {
        construct_transcription(voiced_phonet(analyze_transcription(x)))
    }

    pub fn devoiced_transcription(x: String) -> String
    {
        construct_transcription(devoiced_phonet(analyze_transcription(x)))
    }

    pub fn spirantized_transcription(x: String) -> String
    {
        construct_transcription(spirantized_phonet(analyze_transcription(x)))
    }


    #[cfg(test)]
    mod international_phonetic_alphabet_tests
    {
        use super::*;

        #[test]
        fn voiced_transcription_test()
        {
            assert_eq!(voiced_transcription(String::from("s")), "z");
        }

        #[test]
        fn spirantized_transcription_test_voiceless_alveolar_spirantized()
        {
            assert_eq!(spirantized_transcription(String::from("t")), "θ");
        }

    }

}


