#![allow(dead_code)]

mod lib
{
    use Phonet::*;
    use VocalFolds::*;
    use Place::*;
    use Manner::*;
    use Airstream::*;
    use Height::*;
    use Backness::*;
    use Rounding::*;
    use VocalFolds::*;

    #[derive(Eq, PartialEq, Debug)]
    enum Phonet
    {
        Consonant
        {
            vocal_folds: VocalFolds,
            place: Place,   // Place of articulation
            manner: Manner,  // Manner of articulation
            airstream: Airstream
        },
        
        Vowel
        {
            height: Height,
            backness: Backness,
            rounding: Rounding,
            vocal_folds: VocalFolds
        }
    }



    #[derive(Eq, PartialEq, Debug, Clone, Copy)]
    enum Backness
    {
         Front , Central , Back , UnmarkedBackness
    }


    static BACKNESS_STATES: [Backness; 3] = [Front, Central, Back];

    #[derive(Eq, PartialEq, Debug, Clone, Copy)]
    enum Height
    { 
      Close , NearClose , CloseMid ,
      Mid , OpenMid , NearOpen , Open , UnmarkedHeight
    }

    static HEIGHT_STATES: [Height; 7] = 
      [Close, NearClose, CloseMid, Mid, OpenMid, NearOpen, Open];

    #[derive(Eq, PartialEq, Debug, Clone, Copy)]
    enum Rounding 
    {
        Rounded , Unrounded , UnmarkedRounding
    }

    static ROUNDING_STATES: [Rounding; 2] = [Rounded, Unrounded];


    #[derive(Eq, PartialEq, Debug, Clone, Copy)]
    enum Place
    {
        Bilabial, LabioDental, Dental, Alveolar, PostAlveolar,
        Retroflex, Palatal, Velar, Uvular, Pharyngeal, Glottal, Epiglottal,
        
        // I am unsure if the following three should be counted
        // as 6 different places of articulation, or just 3
        
        LabialVelar, LabialPalatal, AlveoloPalatal,
        PalatoAlveolar,  // To do: investigate what the difference
                       // is between alveolopalatal, and palatoalveolar
        UnmarkedPlace
    }

    static PLACE_STATES: [Place; 16]
       = [ Bilabial, LabioDental, Dental, Alveolar, PostAlveolar
                  , Retroflex
                  , Palatal  , Velar  , Uvular , Pharyngeal , Glottal , Epiglottal
                  , LabialVelar , LabialPalatal , AlveoloPalatal
                  , PalatoAlveolar
                  ];
    
    fn retracted_place(place: Place) -> Place
    {
        match place
        {
            Bilabial => LabioDental,
            LabioDental => Dental,
            Dental => Alveolar,
            Alveolar => PostAlveolar,
            PostAlveolar => Retroflex,
            Retroflex => Palatal,
            Palatal => Velar,
            Velar => Uvular,
            Uvular => Pharyngeal,
            Pharyngeal => Glottal,
            same => same,
        }
    }

    #[derive(Eq, PartialEq, Debug, Clone, Copy)]
    enum Manner
    { Plosive , Nasal , Trill , TapOrFlap , Approximant , Fricative
                  , Affricate 
                  , LateralFricative
                  , LateralApproximant
                  , LateralFlap
                  , Lateral // we need this one for the lateral click.
                  , UnmarkedManner // There are very few IPA symbols for lateral flaps
    }

    static MANNER_STATES: [Manner; 11]
      = [ Plosive
        , Nasal
        , Trill
        , TapOrFlap
        , Approximant
        , Fricative
        , Affricate
        , LateralFricative
        , LateralApproximant
        , LateralFlap
        , Lateral
        ];

    #[derive(Eq, PartialEq, Debug, Clone, Copy)]
    enum Airstream
    {
        PulmonicEgressive , Click , Implosive , UnmarkedAirstream
    }

    static AIRSTREAM_STATES: [Airstream; 3]
       = [PulmonicEgressive, 
          Click, 
          Implosive
         ];

    #[derive(Eq, PartialEq, Debug, Clone, Copy)]
    enum VocalFolds
    {
        Voiced , Voiceless , VoicedAspirated , VoicelessAspirated , CreakyVoiced, UnmarkedVocalFolds
    }

    static VOCAL_FOLD_STATES: [VocalFolds; 5] =
      [Voiceless, Voiced, VoicedAspirated, VoicelessAspirated, CreakyVoiced];


    struct PhonetInventory([Phonet]);

    // A function that given an IPA symbol will convert it to the voiced equivalent.
    fn voiced_phonet(phonete: Phonet) -> Phonet
    {
        match phonete
        {
            Consonant{vocal_folds: VoicelessAspirated, place: x, manner: y, airstream: z} => Consonant{vocal_folds: VoicedAspirated, place: x, manner: y, airstream: z},
            Consonant{vocal_folds: Voiceless, place: x, manner: y, airstream: z} => Consonant{vocal_folds: Voiced, place: x, manner: y, airstream: z},
            Consonant{vocal_folds: Voiced, place: x, manner: y, airstream: z} => Consonant{vocal_folds: Voiced, place: x, manner: y, airstream: z},
            Consonant{vocal_folds: VoicedAspirated, place: x, manner: y, airstream: z} => Consonant {vocal_folds: VoicedAspirated, place: x, manner: y, airstream: z},
            Consonant{vocal_folds: _, place: x, manner: y, airstream: z} => Consonant {vocal_folds: Voiced, place: x, manner: y, airstream: z},

            Vowel{height: x, backness: y, rounding: z, vocal_folds: _} => Vowel{height: x, backness: y, rounding: z,    vocal_folds: Voiced},
        }
    }


    // A function that given an IPA symbol will convert it to the voiceless equivalent.
    fn devoiced_phonet(phonet: Phonet) -> Phonet
    {
        match phonet
        {
            Consonant   {vocal_folds: Voiced            , place: x, manner: y, airstream: z} => Consonant   {vocal_folds: Voiceless         , place: x, manner: y, airstream: z},
            Consonant   {vocal_folds: Voiceless         , place: x, manner: y, airstream: z} => Consonant   {vocal_folds: Voiceless         , place: x, manner: y, airstream: z},
            Consonant   {vocal_folds: VoicedAspirated   , place: x, manner: y, airstream: z} => Consonant   {vocal_folds: VoicelessAspirated, place: x, manner: y, airstream: z},
            Consonant   {vocal_folds: VoicelessAspirated, place: x, manner: y, airstream: z} => Consonant   {vocal_folds: VoicelessAspirated, place: x, manner: y, airstream: z},
            Vowel {height: x, backness: y, rounding: z, vocal_folds: _                         } => Vowel {height: x, backness: y, rounding: z, vocal_folds: Voiceless},
            other => other,
        }
    }


    fn spirantized_phonet(phonet: Phonet) -> Phonet
    {
        match phonet
        {
            Consonant {vocal_folds: x, place: Alveolar, manner: Plosive, airstream: z} => Consonant {vocal_folds: x, place: Dental, manner: Fricative, airstream: z},
            Consonant {vocal_folds: x, place: a_place   , manner: Plosive, airstream: z} => Consonant {vocal_folds: x, place: a_place , manner: Fricative, airstream: z},
            other => other
        }
    }

    fn unmark_differences(phonet1: Phonet, phonet2: Phonet) -> Phonet
    {
        match (phonet1, phonet2)
        {
            (Consonant{vocal_folds: voice1, place: place1, manner: manner1, airstream: airstream1}, 
             Consonant{vocal_folds: voice2, place: place2, manner: manner2, airstream: airstream2}) 
            =>
            {
                let voice3     = if voice1     == voice2     { voice1   } else { UnmarkedVocalFolds } ;
                let place3     = if place1     == place2     { place1   } else { UnmarkedPlace      };
                let manner3    = if manner1    == manner2    {manner1   } else { UnmarkedManner     };
                let airstream3 = if airstream1 == airstream2 {airstream1} else { UnmarkedAirstream  };
                Consonant{vocal_folds:voice3, place: place3, manner: manner3, airstream: airstream3}
            },
            (Vowel{height: height1, backness: backness1, rounding: rounding1, vocal_folds: voice1}, 
             Vowel{height: height2, backness: backness2, rounding: rounding2, vocal_folds: voice2})
            =>
            {
                let voice3    = if voice1    == voice2    { voice1    } else { UnmarkedVocalFolds };
                let height3   = if height1   == height2   { height1   } else { UnmarkedHeight     };
                let backness3 = if backness1 == backness2 { backness1 } else { UnmarkedBackness   };
                let rounding3 = if rounding1 == rounding2 { rounding1 } else { UnmarkedRounding   };
                Vowel{height: height3, backness: backness3, rounding: rounding3, vocal_folds: voice3}
            }
            (Vowel{height: _, backness: _, rounding: _, vocal_folds: voice1},
             Consonant{vocal_folds: voice2, place: _, manner: _, airstream: _})
            =>
            {
                let voice3 = if voice1 == voice2 { voice1 } else { UnmarkedVocalFolds };
                Vowel{height: UnmarkedHeight, backness: UnmarkedBackness, rounding: UnmarkedRounding, vocal_folds: voice3}
            },
            (c @ Consonant{vocal_folds: _, place: _, manner: _, airstream: _},
             v @ Vowel{height: _, backness: _, rounding: _, vocal_folds: _}) 
            => unmark_differences(v, c) // Change the order of arguments
        }
    }
    

        


    fn generate_from_unmarked(phonet: Phonet) -> Vec<Phonet>
    {
        match phonet
        {
            Consonant{vocal_folds: voice1, place: place1, manner: manner1, airstream: airstream1}
            =>
            {
                let mut voice2: Vec<VocalFolds>;
                if voice1  == UnmarkedVocalFolds
                {
                    voice2 = Vec::new();
                    
                    for v in VOCAL_FOLD_STATES.iter()
                    {
                        voice2.push(*v);
                    }
                }
                else
                {
                    voice2 = Vec::new();
                    voice2.push(voice1);
                }
                
                let mut place2: Vec<Place>;
                if place1  == UnmarkedPlace
                {
                    place2 = Vec::new();
                    
                    for p in PLACE_STATES.iter()
                    {
                        place2.push(*p);
                    }
                }
                else
                {
                    place2 = Vec::new();
                    place2.push(place1);
                }
                


                let mut manner2: Vec<Manner>;
                if manner1  == UnmarkedManner
                {
                    manner2 = Vec::new();
                    
                    for m in MANNER_STATES.iter()
                    {
                        manner2.push(*m);
                    }
                }
                else
                {
                    manner2 = Vec::new();
                    manner2.push(manner1);
                }




                let mut airstream2: Vec<Airstream>;
                if airstream1  == UnmarkedAirstream
                {
                    airstream2 = Vec::new();
                    
                    for a in AIRSTREAM_STATES.iter()
                    {
                        airstream2.push(*a);
                    }
                }
                else
                {
                    airstream2 = Vec::new();
                    airstream2.push(airstream1  );
                }

                
                let mut possibilities: Vec<Phonet> = Vec::new();

                for p in place2.iter()
                {
                    for v in voice2.iter()
                    {
                        for m in manner2.iter()
                        {
                            for a in airstream2.iter()
                            {
                                possibilities.push(Consonant{vocal_folds: *v, place: *p, manner: *m, airstream: *a});
                            }
                        }
                    }
                }
                possibilities
            },
           Vowel{height: height1, backness: backness1, rounding: rounding1, vocal_folds: voice1}
            =>
            {
                let mut voice2: Vec<VocalFolds>;
                if voice1  == UnmarkedVocalFolds
                {
                    voice2 = Vec::new();
                    
                    for v in VOCAL_FOLD_STATES.iter()
                    {
                        voice2.push(*v);
                    }
                }
                else
                {
                    voice2 = Vec::new();
                    voice2.push(voice1);
                }
                
                let mut height2: Vec<Height>;
                if height1   == UnmarkedHeight
                {
                    height2 = Vec::new();
                    
                    for h in HEIGHT_STATES.iter()
                    {
                        height2.push(*h);
                    }
                }
                else
                {
                    height2 = Vec::new();
                    height2.push(height1);
                }

                let mut backness2: Vec<Backness>;
                if backness1 == UnmarkedBackness
                {
                    backness2 = Vec::new();
                    
                    for b in BACKNESS_STATES.iter()
                    {
                        backness2.push(*b);
                    }
                }
                else
                {
                    backness2 = Vec::new();
                    backness2.push(backness1);
                }



                let mut rounding2: Vec<Rounding>;
                if rounding1 == UnmarkedRounding
                {
                    rounding2 = Vec::new();
                    
                    for r in ROUNDING_STATES.iter()
                    {
                        rounding2.push(*r);
                    }
                }
                else
                {
                    rounding2 = Vec::new();
                    rounding2.push(rounding1);
                }

                
                let mut possibilities: Vec<Phonet> = Vec::new();
                
                for h in height2.iter()
                {
                    for b in backness2.iter()
                    {
                        for r in rounding2.iter()
                        {
                            for v in voice2.iter()
                            {
                                possibilities.push(Vowel{height: *h, backness: (*b).clone(), rounding: (*r).clone(), vocal_folds: (*v).clone()});
                            }
                        }
                    }
                }
                possibilities
            },
        }
    }
    
    
}
