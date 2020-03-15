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



    enum Backness
    {
         Front , Central , Back , UnmarkedBackness
    }


    static BACKNESS_STATES: [Backness; 3] = [Front, Central, Back];

    enum Height
    { 
      Close , NearClose , CloseMid ,
      Mid , OpenMid , NearOpen , Open , UnmarkedHeight
    }

    static HEIGHT_STATES: [Height; 7] = 
      [Close, NearClose, CloseMid, Mid, OpenMid, NearOpen, Open];

    enum Rounding 
    {
        Rounded , Unrounded , UnmarkedRounding
    }

    static ROUNDING_STATES: [Rounding; 2] = [Rounded, Unrounded];


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

    enum Airstream
    {
        PulmonicEgressive , Click , Implosive , UnmarkedAirstream
    }

    static AIRSTREAM_STATES: [Airstream; 3]
       = [PulmonicEgressive, 
          Click, 
          Implosive
         ];

    enum VocalFolds
    {
        Voiced , Voiceless , VoicedAspirated , VoicelessAspirated , UnmarkedVocalFolds
    }

    static VOCAL_FOLD_STATES: [VocalFolds; 2] = 
      [Voiceless, Voiced];

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
            Consonant{vocal_folds: UnmarkedVocalFolds, place: x, manner: y, airstream: z} => Consonant {vocal_folds: UnmarkedVocalFolds, place: x, manner: y, airstream: z},
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


}
