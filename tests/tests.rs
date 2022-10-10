#![feature(test)]

#[cfg(test)]
extern crate test;
#[cfg(test)]
extern crate speculate;

#[cfg(test)]
use speculate::speculate;

use wana_kana::to_hiragana::*;
use wana_kana::to_kana;
use wana_kana::to_kana::*;
use wana_kana::to_katakana::*;

use wana_kana::to_romaji;
use wana_kana::to_romaji::*;
use wana_kana::Options;

mod conversion_tables;
use conversion_tables::*;

speculate! {

describe "character_conversion" {

    describe "test every conversion table char" {
        it "to_kana()" {
            for &[romaji, hiragana, katakana] in ROMA_TO_HIRA_KATA.iter() {
                let lower = to_kana(romaji);
                let upper = to_kana(&romaji.to_uppercase());
                assert_eq!(lower, hiragana);
                assert_eq!(upper, katakana);
                        assert_eq!(hiragana.len(), katakana.len());
            }
        }

        it "to_hiragana()" {
            for &[romaji, hiragana, _katakana] in ROMA_TO_HIRA_KATA.iter() {
                let lower = to_hiragana(romaji);
                let upper = to_hiragana(&romaji.to_uppercase());
                assert_eq!(lower, hiragana);
                assert_eq!(upper, hiragana);
            }
        }


        it "Hiragana input to_romaji()" {
            for &[hiragana, _, romaji] in HIRA_KATA_TO_ROMA.iter() {

                if hiragana != ""{
                    assert_eq!(to_romaji(hiragana), romaji);
                }
            }
        }

        it "Katakana input to_romaji()" {
            for &[_, katakana, romaji] in HIRA_KATA_TO_ROMA.iter() {

                if katakana != ""{
                    assert_eq!(to_romaji(katakana), romaji);
                }
            }
        }
    }

    describe "converting_kana_to_kana" {
        it "k -> h" {
            assert_eq!(to_hiragana("バケル"), "ばける");
            assert_eq!(to_hiragana("バケル").len(), "ばける".len());
        }
        it "h -> k" {
            assert_eq!(to_katakana("ばける"), "バケル");
        }

        it "It survives only katakana to_katakana" {

            assert_eq!(to_katakana("スタイル"), "スタイル");

        }
        it "It survives only hiragana to_hiragana" {
            assert_eq!(to_hiragana("すたーいる"), "すたーいる");
            assert_eq!(to_hiragana("すたーいる").len(), "すたーいる".len());
        }
        it "Mixed kana converts every char k -> h" {
            assert_eq!(to_katakana("アメリカじん"), "アメリカジン");
        }
        it "Mixed kana converts every char h -> k" {
            assert_eq!(to_hiragana("アメリカじん"), "あめりかじん");
            assert_eq!(to_hiragana("アメリカじん").len(), "あめりかじん".len());
        }
    }
    describe "long_vowels" {
        it "Converts long vowels correctly from k -> h" {
          assert_eq!(to_hiragana("バツゴー"), "ばつごう");
          assert_eq!(to_hiragana("バツゴー").len(), "ばつごう".len());
        }
        it "Preserves long dash from h -> k" {
          assert_eq!(to_katakana("ばつゲーム"), "バツゲーム");
        }
        it "Preserves long dash from h -> h" {
          assert_eq!(to_hiragana("ばつげーむ"), "ばつげーむ");
          assert_eq!(to_hiragana("ばつげーむ").len(), "ばつげーむ".len());
        }
        it "Preserves long dash from k -> k" {
          assert_eq!(to_katakana("バツゲーム"), "バツゲーム");
        }
        it "Preserves long dash from mixed -> k 1" {
          assert_eq!(to_katakana("バツゲーム"), "バツゲーム");
        }
        it "Preserves long dash from mixed -> k 2" {
          assert_eq!(to_katakana("テスーと"), "テスート");
        }
        it "Preserves long dash from mixed -> h 1" {
          assert_eq!(to_hiragana("てすート"), "てすーと");
          assert_eq!(to_hiragana("てすート").len(), "てすーと".len());
        }
        it "Preserves long dash from mixed -> h 2" {
          assert_eq!(to_hiragana("てすー戸"), "てすー戸");
          assert_eq!(to_hiragana("てすー戸").len(), "てすー戸".len());
        }
        it "Preserves long dash from mixed -> h 3" {
          assert_eq!(to_hiragana("手巣ート"), "手巣ーと");
          assert_eq!(to_hiragana("手巣ート").len(), "手巣ーと".len());
        }
        it "Preserves long dash from mixed -> h 4" {
          assert_eq!(to_hiragana("tesート"), "てsーと");
          assert_eq!(to_hiragana("tesート").len(), "てsーと".len());
        }
        it "Preserves long dash from mixed -> h 5" {
          assert_eq!(to_hiragana("ートtesu"), "ーとてす");
          assert_eq!(to_hiragana("ートtesu").len(), "ーとてす".len());
        }
    }

    describe "mixed_syllabaries" {
        it "It passes non-katakana through when pass_romaji is true k -> h" {
          assert_eq!(to_hiragana_with_opt("座禅‘zazen’スタイル", Options{ pass_romaji: true, .. Default::default() }), "座禅‘zazen’すたいる");
        }
        it "It passes non-hiragana through when pass_romaji is true h -> k" {
          assert_eq!(to_katakana_with_opt("座禅‘zazen’すたいる", Options{ pass_romaji: true, .. Default::default() }), "座禅‘zazen’スタイル");
        }
        it "It converts non-katakana when pass_romaji is false k -> h" {
          assert_eq!(to_hiragana("座禅‘zazen’スタイル"), "座禅「ざぜん」すたいる");
          assert_eq!(to_hiragana("座禅‘zazen’スタイル").len(), "座禅「ざぜん」すたいる".len());
        }
        it "It converts non-hiragana when pass_romaji is false h -> k" {
          assert_eq!(to_katakana("座禅‘zazen’すたいる"), "座禅「ザゼン」スタイル");
        }
    }
}

describe "case_sensitivity" {
    it "cAse DoEsnT MatTER for to_hiragana()" {
        assert_eq!(to_hiragana("aiueo"), to_hiragana("AIUEO"));
        assert_eq!(to_hiragana("aiueo").len(), to_hiragana("AIUEO").len());
    }
    it "cAse DoEsnT MatTER for to_katakana()" {
        assert_eq!(to_katakana("aiueo"), to_katakana("AIUEO"));
    }
    it "Case DOES matter for to_kana()" {
        assert_ne!(to_kana("aiueo"), to_kana("AIUEO"));
    }
}



describe "n_edge_cases" {
    it "Solo N" {
        assert_eq!(to_kana("n"), "ん");
    }
    it "double N" {
        assert_eq!(to_kana("onn"), "おんん");
    }
    it "N followed by N* syllable" {
        assert_eq!(to_kana("onna"), "おんな");
    }
    it "Triple N" {
        assert_eq!(to_kana("nnn"), "んんん");
    }
    it "Triple N followed by N* syllable" {
        assert_eq!(to_kana("onnna"), "おんんな");
    }
    it "Quadruple N" {
        assert_eq!(to_kana("nnnn"), "んんんん");
    }
    it "nya -> にゃ" {
        assert_eq!(to_kana("nyan"), "にゃん");
    }
    it "nnya -> んにゃ" {
        assert_eq!(to_kana("nnyann"), "んにゃんん");
    }
    it "nnnya -> んにゃ" {
        assert_eq!(to_kana("nnnyannn"), "んんにゃんんん");
    }
    it "n'ya -> んや" {
        assert_eq!(to_kana("n'ya"), "んや");
    }
    it "kin'ya -> きんや" {
        assert_eq!(to_kana("kin'ya"), "きんや");
    }
    it "shin'ya -> しんや" {
        assert_eq!(to_kana("shin'ya"), "しんや");
    }
    it "kinyou -> きにょう" {
        assert_eq!(to_kana("kinyou"), "きにょう");
    }
    it "kin'you -> きんよう" {
        assert_eq!(to_kana("kin'you"), "きんよう");
    }
    it "kin'yu -> きんゆ" {
        assert_eq!(to_kana("kin'yu"), "きんゆ");
    }
    it "Properly add space after n[space]" {
        assert_eq!(to_kana("ichiban warui"), "いちばん わるい");
    }
}

describe "bogus_4_character_sequences" {
    it "Non bogus sequences work" {
        assert_eq!(to_kana("chya"), "ちゃ");
    }
    it "Bogus sequences do not work" {
        assert_eq!(to_kana("chyx"), "chyx");
    }
    it "Bogus sequences do not work 2" {
        assert_eq!(to_kana("shyp"), "shyp");
    }
    it "Bogus sequences do not work 3" {
        assert_eq!(to_kana("ltsb"), "ltsb");
    }
}


}

#[bench]
fn bench_kana_1(b: &mut test::Bencher) {
    b.iter(|| to_kana::to_kana("aiueosashisusesonaninunenokakikukeko"))
}

#[bench]
fn bench_kana_2(b: &mut test::Bencher) {
    b.iter(|| to_kana::to_kana("AIUEOSASHISUSESONANINUNENOKAKIKUKEKO"))
}

#[bench]
fn bench_romaji_to_hiragana(b: &mut test::Bencher) {
    b.iter(|| to_hiragana("aiueosashisusesonaninunenokakikukeko"))
}

#[bench]
fn bench_katakana_to_hiragana(b: &mut test::Bencher) {
    b.iter(|| to_hiragana("アイウエオサシスセソナニヌネノカキクケコ"))
}

#[bench]
fn bench_romaji_to_katakana(b: &mut test::Bencher) {
    b.iter(|| to_katakana("aiueosashisusesonaninunenokakikukeko"))
}

#[bench]
fn bench_katakana_to_katakana(b: &mut test::Bencher) {
    b.iter(|| to_katakana("あいうえおさしすせそなにぬねのかきくけこ"))
}

#[bench]
fn bench_hiragana_to_romaji(b: &mut test::Bencher) {
    b.iter(|| to_romaji::to_romaji("あいうえおさしすせそなにぬねのかきくけこ"))
}

#[bench]
fn bench_katakana_to_romaji(b: &mut test::Bencher) {
    b.iter(|| to_romaji::to_romaji("アイウエオサシスセソナニヌネノカキクケコ"))
}
