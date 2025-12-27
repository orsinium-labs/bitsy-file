#[cfg(test)]
mod test {
    use crate::game::Game;
    use alloc::string::ToString;

    /// bitsy-parser will parse these games correctly
    /// but the output does not match the input, due to game data errors.
    const EXPECTED_FAILURES: &[&str] = &[
        // position out of bounds e.g. "5, -1"
        "CFE62F11", // "SweetPea Village",
        "013B3CDE", // "Sunset Shore",
        "65C2B499", // "==GIRLS OWN THE VOID=={br}a faux platformer",
        "74E0F6EF", // this one has no name. the files is huge so the test hangs, but it does parse ok apart from a bad item position.
        // extra tiles in room (this was an old editor bug)
        "07836D6F", // "I can't run anymore. They won't give up. I need water.",
        "12490381", // "Picnic at Castle Island",
        "14C48FA0", // "whatever you were doing ran later than expected and now it's",
        "26A717C8", // "Goodfishas",
        "2A1D0AF0", // "Dont Go South",
        "2A5FDAE9", // "stars",
        "5F836D20", // "We live in an increasingly connected world where we are incentivised to share every visual stimulus and emotional trigger that we encounter. ",
        "C5CF3FDA", // "No Snow",
        "D2A4D690", // "Cryptid Hunt feat. Mothman",
        "ED62FAC9", // "SCREEECH!                                           THUNK.",
        "F3E61FC1", // "Vitreous",
        "45EF9604",
        // extraneous character at end of file
        "D1FB278A", // "I Dream of Pixels",
        "755DA50E", // "You arrive at a party.",
        // extraneous whitespace
        "49F56F40", // "sun machine",
        "A5816C9C", // "THE BITSY MYSTERY DUNGEON{br}A game by everyone",
        "F31CA90B", // "THE UNSEEN LIGHT{br}For Gothic Novel Jam 2018",
        // extra pixels in image (another old editor bug)
        "36DB0432",
        "801AE85A", // "Write your game's title hevery blind (just without glasses but look my eyesight is very my eye sight is not good ok) (also i had a drink but am barely even tipsy but whatever)",
        "86CC164A", // "Dog Walking, Dog Running, and Dog Still.",
        "8831A581", // "startup sequence...",
        "9484FD34", // "A Bit of Reverie",
        "94943E5C", // "I am lost on this planet     Alone, yet crowded by thoughts",
        "C1DC0328", // "Four Corners",
        "F79D5368", // "the fox and the moon",
        // unexpected ordering of game data
        "C673EA1B", // "here are my cats",
        // NaN instead of pixels/etc.
        "DA88C287",
        "C2EF387A", // {clr2}{scramble}skeletal dos dreams skeletal dos dreams{scramble} skeletal dos dreams{clr2}
    ];

    fn str(s: &str, id: &str) {
        let result = Game::from(s);
        assert!(result.is_ok());
        let game = result.expect("failed to parse game");
        let actual = game.to_string();
        let actual = actual.trim_matches('\n');
        let expected = s.to_string();
        let expected = expected.trim_matches('\n');
        if EXPECTED_FAILURES.contains(&id) {
            assert!(actual != expected);
        } else {
            assert_eq!(actual, expected, "output does not match input");
        }
    }

    #[test]
    fn test_0053b32f() {
        str(
            include_str!("test-resources/omnibus/0053B32F.bitsy.txt"),
            "0053B32F",
        );
    }
    #[test]
    fn test_00e45dc5() {
        str(
            include_str!("test-resources/omnibus/00E45DC5.bitsy.txt"),
            "00E45DC5",
        );
    }
    #[test]
    fn test_010beb39() {
        str(
            include_str!("test-resources/omnibus/010BEB39.bitsy.txt"),
            "010BEB39",
        );
    }
    #[test]
    fn test_013b3cde() {
        str(
            include_str!("test-resources/omnibus/013B3CDE.bitsy.txt"),
            "013B3CDE",
        );
    }
    #[test]
    fn test_0145f050() {
        str(
            include_str!("test-resources/omnibus/0145F050.bitsy.txt"),
            "0145F050",
        );
    }
    #[test]
    fn test_031faa8a() {
        str(
            include_str!("test-resources/omnibus/031FAA8A.bitsy.txt"),
            "031FAA8A",
        );
    }
    #[test]
    fn test_0365960a() {
        str(
            include_str!("test-resources/omnibus/0365960A.bitsy.txt"),
            "0365960A",
        );
    }
    #[test]
    fn test_04716e2a() {
        str(
            include_str!("test-resources/omnibus/04716E2A.bitsy.txt"),
            "04716E2A",
        );
    }
    #[test]
    fn test_058b6cc2() {
        str(
            include_str!("test-resources/omnibus/058B6CC2.bitsy.txt"),
            "058B6CC2",
        );
    }
    #[test]
    fn test_0727ffab() {
        str(
            include_str!("test-resources/omnibus/0727FFAB.bitsy.txt"),
            "0727FFAB",
        );
    }
    #[test]
    fn test_07836d6f() {
        str(
            include_str!("test-resources/omnibus/07836D6F.bitsy.txt"),
            "07836D6F",
        );
    }
    #[test]
    fn test_0944c6fc() {
        str(
            include_str!("test-resources/omnibus/0944C6FC.bitsy.txt"),
            "0944C6FC",
        );
    }
    #[test]
    fn test_09463227() {
        str(
            include_str!("test-resources/omnibus/09463227.bitsy.txt"),
            "09463227",
        );
    }
    #[test]
    fn test_09b9f805() {
        str(
            include_str!("test-resources/omnibus/09B9F805.bitsy.txt"),
            "09B9F805",
        );
    }
    #[test]
    fn test_0a5b0213() {
        str(
            include_str!("test-resources/omnibus/0A5B0213.bitsy.txt"),
            "0A5B0213",
        );
    }
    #[test]
    fn test_0b05d95e() {
        str(
            include_str!("test-resources/omnibus/0B05D95E.bitsy.txt"),
            "0B05D95E",
        );
    }
    #[test]
    fn test_0bdf73f8() {
        str(
            include_str!("test-resources/omnibus/0BDF73F8.bitsy.txt"),
            "0BDF73F8",
        );
    }
    #[test]
    fn test_0ca8ccc9() {
        str(
            include_str!("test-resources/omnibus/0CA8CCC9.bitsy.txt"),
            "0CA8CCC9",
        );
    }
    #[test]
    fn test_0cbaa229() {
        str(
            include_str!("test-resources/omnibus/0CBAA229.bitsy.txt"),
            "0CBAA229",
        );
    }
    #[test]
    fn test_0d045f5f() {
        str(
            include_str!("test-resources/omnibus/0D045F5F.bitsy.txt"),
            "0D045F5F",
        );
    }
    #[test]
    fn test_0d5f9f11() {
        str(
            include_str!("test-resources/omnibus/0D5F9F11.bitsy.txt"),
            "0D5F9F11",
        );
    }
    #[test]
    fn test_0daebe2d() {
        str(
            include_str!("test-resources/omnibus/0DAEBE2D.bitsy.txt"),
            "0DAEBE2D",
        );
    }
    #[test]
    fn test_0dc609d8() {
        str(
            include_str!("test-resources/omnibus/0DC609D8.bitsy.txt"),
            "0DC609D8",
        );
    }
    #[test]
    fn test_0ddead71() {
        str(
            include_str!("test-resources/omnibus/0DDEAD71.bitsy.txt"),
            "0DDEAD71",
        );
    }
    #[test]
    fn test_0f3f62ba() {
        str(
            include_str!("test-resources/omnibus/0F3F62BA.bitsy.txt"),
            "0F3F62BA",
        );
    }
    #[test]
    fn test_0fabc625() {
        str(
            include_str!("test-resources/omnibus/0FABC625.bitsy.txt"),
            "0FABC625",
        );
    }
    #[test]
    fn test_0fba2397() {
        str(
            include_str!("test-resources/omnibus/0FBA2397.bitsy.txt"),
            "0FBA2397",
        );
    }
    #[test]
    fn test_0ff04b41() {
        str(
            include_str!("test-resources/omnibus/0FF04B41.bitsy.txt"),
            "0FF04B41",
        );
    }
    #[test]
    fn test_103cee9f() {
        str(
            include_str!("test-resources/omnibus/103CEE9F.bitsy.txt"),
            "103CEE9F",
        );
    }
    #[test]
    fn test_1057e4b3() {
        str(
            include_str!("test-resources/omnibus/1057E4B3.bitsy.txt"),
            "1057E4B3",
        );
    }
    #[test]
    fn test_107b236b() {
        str(
            include_str!("test-resources/omnibus/107B236B.bitsy.txt"),
            "107B236B",
        );
    }
    #[test]
    fn test_111183bd() {
        str(
            include_str!("test-resources/omnibus/111183BD.bitsy.txt"),
            "111183BD",
        );
    }
    #[test]
    fn test_12201174() {
        str(
            include_str!("test-resources/omnibus/12201174.bitsy.txt"),
            "12201174",
        );
    }
    #[test]
    fn test_12490381() {
        str(
            include_str!("test-resources/omnibus/12490381.bitsy.txt"),
            "12490381",
        );
    }
    #[test]
    fn test_127e4635() {
        str(
            include_str!("test-resources/omnibus/127E4635.bitsy.txt"),
            "127E4635",
        );
    }
    #[test]
    fn test_13882e3b() {
        str(
            include_str!("test-resources/omnibus/13882E3B.bitsy.txt"),
            "13882E3B",
        );
    }
    #[test]
    fn test_138a5257() {
        str(
            include_str!("test-resources/omnibus/138A5257.bitsy.txt"),
            "138A5257",
        );
    }
    #[test]
    fn test_13935f38() {
        str(
            include_str!("test-resources/omnibus/13935F38.bitsy.txt"),
            "13935F38",
        );
    }
    #[test]
    fn test_14926703() {
        str(
            include_str!("test-resources/omnibus/14926703.bitsy.txt"),
            "14926703",
        );
    }
    #[test]
    fn test_14c48fa0() {
        str(
            include_str!("test-resources/omnibus/14C48FA0.bitsy.txt"),
            "14C48FA0",
        );
    }
    #[test]
    fn test_163df269() {
        str(
            include_str!("test-resources/omnibus/163DF269.bitsy.txt"),
            "163DF269",
        );
    }
    #[test]
    fn test_1663a765() {
        str(
            include_str!("test-resources/omnibus/1663A765.bitsy.txt"),
            "1663A765",
        );
    }
    #[test]
    fn test_17c6de40() {
        str(
            include_str!("test-resources/omnibus/17C6DE40.bitsy.txt"),
            "17C6DE40",
        );
    }
    #[test]
    fn test_17f6799b() {
        str(
            include_str!("test-resources/omnibus/17F6799B.bitsy.txt"),
            "17F6799B",
        );
    }
    #[test]
    fn test_18647f85() {
        str(
            include_str!("test-resources/omnibus/18647F85.bitsy.txt"),
            "18647F85",
        );
    }
    #[test]
    fn test_18a82fc7() {
        str(
            include_str!("test-resources/omnibus/18A82FC7.bitsy.txt"),
            "18A82FC7",
        );
    }
    #[test]
    fn test_19217ac5() {
        str(
            include_str!("test-resources/omnibus/19217AC5.bitsy.txt"),
            "19217AC5",
        );
    }
    #[test]
    fn test_196c5cfe() {
        str(
            include_str!("test-resources/omnibus/196C5CFE.bitsy.txt"),
            "196C5CFE",
        );
    }
    #[test]
    fn test_1998508e() {
        str(
            include_str!("test-resources/omnibus/1998508E.bitsy.txt"),
            "1998508E",
        );
    }
    #[test]
    fn test_19adfd75() {
        str(
            include_str!("test-resources/omnibus/19ADFD75.bitsy.txt"),
            "19ADFD75",
        );
    }
    #[test]
    fn test_19f30e9d() {
        str(
            include_str!("test-resources/omnibus/19F30E9D.bitsy.txt"),
            "19F30E9D",
        );
    }
    #[test]
    fn test_1a2279eb() {
        str(
            include_str!("test-resources/omnibus/1A2279EB.bitsy.txt"),
            "1A2279EB",
        );
    }
    #[test]
    fn test_1a595207() {
        str(
            include_str!("test-resources/omnibus/1A595207.bitsy.txt"),
            "1A595207",
        );
    }
    #[test]
    fn test_1b10d2fe() {
        str(
            include_str!("test-resources/omnibus/1B10D2FE.bitsy.txt"),
            "1B10D2FE",
        );
    }
    #[test]
    fn test_1c54c16d() {
        str(
            include_str!("test-resources/omnibus/1C54C16D.bitsy.txt"),
            "1C54C16D",
        );
    }
    #[test]
    fn test_1e2742e0() {
        str(
            include_str!("test-resources/omnibus/1E2742E0.bitsy.txt"),
            "1E2742E0",
        );
    }
    #[test]
    fn test_1e339dba() {
        str(
            include_str!("test-resources/omnibus/1E339DBA.bitsy.txt"),
            "1E339DBA",
        );
    }
    #[test]
    fn test_1f6683a4() {
        str(
            include_str!("test-resources/omnibus/1F6683A4.bitsy.txt"),
            "1F6683A4",
        );
    }
    #[test]
    fn test_1f8db6b8() {
        str(
            include_str!("test-resources/omnibus/1F8DB6B8.bitsy.txt"),
            "1F8DB6B8",
        );
    }
    #[test]
    fn test_209b9517() {
        str(
            include_str!("test-resources/omnibus/209B9517.bitsy.txt"),
            "209B9517",
        );
    }
    #[test]
    fn test_20b08319() {
        str(
            include_str!("test-resources/omnibus/20B08319.bitsy.txt"),
            "20B08319",
        );
    }
    #[test]
    fn test_21e26b92() {
        str(
            include_str!("test-resources/omnibus/21E26B92.bitsy.txt"),
            "21E26B92",
        );
    }
    #[test]
    fn test_22ab996a() {
        str(
            include_str!("test-resources/omnibus/22AB996A.bitsy.txt"),
            "22AB996A",
        );
    }
    #[test]
    fn test_22cef657() {
        str(
            include_str!("test-resources/omnibus/22CEF657.bitsy.txt"),
            "22CEF657",
        );
    }
    #[test]
    fn test_23a22678() {
        str(
            include_str!("test-resources/omnibus/23A22678.bitsy.txt"),
            "23A22678",
        );
    }
    #[test]
    fn test_23c95b44() {
        str(
            include_str!("test-resources/omnibus/23C95B44.bitsy.txt"),
            "23C95B44",
        );
    }
    #[test]
    fn test_245ee1c6() {
        str(
            include_str!("test-resources/omnibus/245EE1C6.bitsy.txt"),
            "245EE1C6",
        );
    }
    #[test]
    fn test_24ac79ff() {
        str(
            include_str!("test-resources/omnibus/24AC79FF.bitsy.txt"),
            "24AC79FF",
        );
    }
    #[test]
    fn test_24c74f49() {
        str(
            include_str!("test-resources/omnibus/24C74F49.bitsy.txt"),
            "24C74F49",
        );
    }
    #[test]
    fn test_252182b5() {
        str(
            include_str!("test-resources/omnibus/252182B5.bitsy.txt"),
            "252182B5",
        );
    }
    #[test]
    fn test_26a20c22() {
        str(
            include_str!("test-resources/omnibus/26A20C22.bitsy.txt"),
            "26A20C22",
        );
    }
    #[test]
    fn test_26a717c8() {
        str(
            include_str!("test-resources/omnibus/26A717C8.bitsy.txt"),
            "26A717C8",
        );
    }
    #[test]
    fn test_27b5cddf() {
        str(
            include_str!("test-resources/omnibus/27B5CDDF.bitsy.txt"),
            "27B5CDDF",
        );
    }
    #[test]
    fn test_27dfc976() {
        str(
            include_str!("test-resources/omnibus/27DFC976.bitsy.txt"),
            "27DFC976",
        );
    }
    #[test]
    fn test_2805c4e9() {
        str(
            include_str!("test-resources/omnibus/2805C4E9.bitsy.txt"),
            "2805C4E9",
        );
    }
    #[test]
    fn test_282c85b4() {
        str(
            include_str!("test-resources/omnibus/282C85B4.bitsy.txt"),
            "282C85B4",
        );
    }
    #[test]
    fn test_2839d1ae() {
        str(
            include_str!("test-resources/omnibus/2839D1AE.bitsy.txt"),
            "2839D1AE",
        );
    }
    #[test]
    fn test_284796eb() {
        str(
            include_str!("test-resources/omnibus/284796EB.bitsy.txt"),
            "284796EB",
        );
    }
    #[test]
    fn test_284d2078() {
        str(
            include_str!("test-resources/omnibus/284D2078.bitsy.txt"),
            "284D2078",
        );
    }
    #[test]
    fn test_29e7379a() {
        str(
            include_str!("test-resources/omnibus/29E7379A.bitsy.txt"),
            "29E7379A",
        );
    }
    #[test]
    fn test_2a1d0af0() {
        str(
            include_str!("test-resources/omnibus/2A1D0AF0.bitsy.txt"),
            "2A1D0AF0",
        );
    }
    #[test]
    fn test_2a5fdae9() {
        str(
            include_str!("test-resources/omnibus/2A5FDAE9.bitsy.txt"),
            "2A5FDAE9",
        );
    }
    #[test]
    fn test_2a879a66() {
        str(
            include_str!("test-resources/omnibus/2A879A66.bitsy.txt"),
            "2A879A66",
        );
    }
    #[test]
    fn test_2ae3f2f7() {
        str(
            include_str!("test-resources/omnibus/2AE3F2F7.bitsy.txt"),
            "2AE3F2F7",
        );
    }
    #[test]
    fn test_2ae5b41f() {
        str(
            include_str!("test-resources/omnibus/2AE5B41F.bitsy.txt"),
            "2AE5B41F",
        );
    }
    #[test]
    fn test_2b22c193() {
        str(
            include_str!("test-resources/omnibus/2B22C193.bitsy.txt"),
            "2B22C193",
        );
    }
    #[test]
    fn test_2d2b56b6() {
        str(
            include_str!("test-resources/omnibus/2D2B56B6.bitsy.txt"),
            "2D2B56B6",
        );
    }
    #[test]
    fn test_2d533752() {
        str(
            include_str!("test-resources/omnibus/2D533752.bitsy.txt"),
            "2D533752",
        );
    }
    #[test]
    fn test_2d678f83() {
        str(
            include_str!("test-resources/omnibus/2D678F83.bitsy.txt"),
            "2D678F83",
        );
    }
    #[test]
    fn test_2dedac14() {
        str(
            include_str!("test-resources/omnibus/2DEDAC14.bitsy.txt"),
            "2DEDAC14",
        );
    }
    #[test]
    fn test_2e2987d0() {
        str(
            include_str!("test-resources/omnibus/2E2987D0.bitsy.txt"),
            "2E2987D0",
        );
    }
    #[test]
    fn test_2e60f4c3() {
        str(
            include_str!("test-resources/omnibus/2E60F4C3.bitsy.txt"),
            "2E60F4C3",
        );
    }
    #[test]
    fn test_30960393() {
        str(
            include_str!("test-resources/omnibus/30960393.bitsy.txt"),
            "30960393",
        );
    }
    #[test]
    fn test_31102002() {
        str(
            include_str!("test-resources/omnibus/31102002.bitsy.txt"),
            "31102002",
        );
    }
    /// todo fix this - a triple-quoted dialogue becomes empty
    // #[test]
    // fn test_313d1314() {
    //     str(
    //         include_str!("test-resources/omnibus/313D1314.bitsy.txt"),
    //         "313D1314",
    //     );
    // }
    #[test]
    fn test_317415f3() {
        str(
            include_str!("test-resources/omnibus/317415F3.bitsy.txt"),
            "317415F3",
        );
    }
    #[test]
    fn test_31c0d44b() {
        str(
            include_str!("test-resources/omnibus/31C0D44B.bitsy.txt"),
            "31C0D44B",
        );
    }
    #[test]
    fn test_32051452() {
        str(
            include_str!("test-resources/omnibus/32051452.bitsy.txt"),
            "32051452",
        );
    }
    #[test]
    fn test_333db34a() {
        str(
            include_str!("test-resources/omnibus/333DB34A.bitsy.txt"),
            "333DB34A",
        );
    }
    #[test]
    fn test_3388d883() {
        str(
            include_str!("test-resources/omnibus/3388D883.bitsy.txt"),
            "3388D883",
        );
    }
    #[test]
    fn test_34e2323e() {
        str(
            include_str!("test-resources/omnibus/34E2323E.bitsy.txt"),
            "34E2323E",
        );
    }
    #[test]
    fn test_3595b459() {
        str(
            include_str!("test-resources/omnibus/3595B459.bitsy.txt"),
            "3595B459",
        );
    }
    #[test]
    fn test_35c079d5() {
        str(
            include_str!("test-resources/omnibus/35C079D5.bitsy.txt"),
            "35C079D5",
        );
    }
    #[test]
    fn test_362c9f8e() {
        str(
            include_str!("test-resources/omnibus/362C9F8E.bitsy.txt"),
            "362C9F8E",
        );
    }
    #[test]
    fn test_362f1e1d() {
        str(
            include_str!("test-resources/omnibus/362F1E1D.bitsy.txt"),
            "362F1E1D",
        );
    }
    #[test]
    fn test_3664c1b9() {
        str(
            include_str!("test-resources/omnibus/3664C1B9.bitsy.txt"),
            "3664C1B9",
        );
    }
    #[test]
    fn test_36db0432() {
        str(
            include_str!("test-resources/omnibus/36DB0432.bitsy.txt"),
            "36DB0432",
        );
    }
    #[test]
    fn test_3895271d() {
        str(
            include_str!("test-resources/omnibus/3895271D.bitsy.txt"),
            "3895271D",
        );
    }
    #[test]
    fn test_38bedab0() {
        str(
            include_str!("test-resources/omnibus/38BEDAB0.bitsy.txt"),
            "38BEDAB0",
        );
    }
    #[test]
    fn test_38d19484() {
        str(
            include_str!("test-resources/omnibus/38D19484.bitsy.txt"),
            "38D19484",
        );
    }
    #[test]
    fn test_39eff3d0() {
        str(
            include_str!("test-resources/omnibus/39EFF3D0.bitsy.txt"),
            "39EFF3D0",
        );
    }
    #[test]
    fn test_3a3e8773() {
        str(
            include_str!("test-resources/omnibus/3A3E8773.bitsy.txt"),
            "3A3E8773",
        );
    }
    #[test]
    fn test_3a68df96() {
        str(
            include_str!("test-resources/omnibus/3A68DF96.bitsy.txt"),
            "3A68DF96",
        );
    }
    #[test]
    fn test_3a6a0fee() {
        str(
            include_str!("test-resources/omnibus/3A6A0FEE.bitsy.txt"),
            "3A6A0FEE",
        );
    }
    #[test]
    fn test_3ad39018() {
        str(
            include_str!("test-resources/omnibus/3AD39018.bitsy.txt"),
            "3AD39018",
        );
    }
    #[test]
    fn test_3adc3b2b() {
        str(
            include_str!("test-resources/omnibus/3ADC3B2B.bitsy.txt"),
            "3ADC3B2B",
        );
    }
    #[test]
    fn test_3b376b08() {
        str(
            include_str!("test-resources/omnibus/3B376B08.bitsy.txt"),
            "3B376B08",
        );
    }
    #[test]
    fn test_3bdc8a5e() {
        str(
            include_str!("test-resources/omnibus/3BDC8A5E.bitsy.txt"),
            "3BDC8A5E",
        );
    }
    #[test]
    fn test_3c2225e3() {
        str(
            include_str!("test-resources/omnibus/3C2225E3.bitsy.txt"),
            "3C2225E3",
        );
    }
    #[test]
    fn test_3c5ba8f0() {
        str(
            include_str!("test-resources/omnibus/3C5BA8F0.bitsy.txt"),
            "3C5BA8F0",
        );
    }
    #[test]
    fn test_3c712f1b() {
        str(
            include_str!("test-resources/omnibus/3C712F1B.bitsy.txt"),
            "3C712F1B",
        );
    }
    #[test]
    fn test_3c814196() {
        str(
            include_str!("test-resources/omnibus/3C814196.bitsy.txt"),
            "3C814196",
        );
    }
    #[test]
    fn test_3c8c19dd() {
        str(
            include_str!("test-resources/omnibus/3C8C19DD.bitsy.txt"),
            "3C8C19DD",
        );
    }
    #[test]
    fn test_3e8c3022() {
        str(
            include_str!("test-resources/omnibus/3E8C3022.bitsy.txt"),
            "3E8C3022",
        );
    }
    #[test]
    fn test_3f6eaaeb() {
        str(
            include_str!("test-resources/omnibus/3F6EAAEB.bitsy.txt"),
            "3F6EAAEB",
        );
    }
    #[test]
    fn test_3fc83ee6() {
        str(
            include_str!("test-resources/omnibus/3FC83EE6.bitsy.txt"),
            "3FC83EE6",
        );
    }
    #[test]
    fn test_400a3a88() {
        str(
            include_str!("test-resources/omnibus/400A3A88.bitsy.txt"),
            "400A3A88",
        );
    }
    #[test]
    fn test_40610fb3() {
        str(
            include_str!("test-resources/omnibus/40610FB3.bitsy.txt"),
            "40610FB3",
        );
    }
    #[test]
    fn test_40e58b03() {
        str(
            include_str!("test-resources/omnibus/40E58B03.bitsy.txt"),
            "40E58B03",
        );
    }
    #[test]
    fn test_41b01b3a() {
        str(
            include_str!("test-resources/omnibus/41B01B3A.bitsy.txt"),
            "41B01B3A",
        );
    }
    #[test]
    fn test_41c30128() {
        str(
            include_str!("test-resources/omnibus/41C30128.bitsy.txt"),
            "41C30128",
        );
    }
    #[test]
    fn test_42492af1() {
        str(
            include_str!("test-resources/omnibus/42492AF1.bitsy.txt"),
            "42492AF1",
        );
    }
    #[test]
    fn test_428d89c7() {
        str(
            include_str!("test-resources/omnibus/428D89C7.bitsy.txt"),
            "428D89C7",
        );
    }
    #[test]
    fn test_42c251ea() {
        str(
            include_str!("test-resources/omnibus/42C251EA.bitsy.txt"),
            "42C251EA",
        );
    }
    #[test]
    fn test_439ba1a5() {
        str(
            include_str!("test-resources/omnibus/439BA1A5.bitsy.txt"),
            "439BA1A5",
        );
    }
    #[test]
    fn test_43abaffd() {
        str(
            include_str!("test-resources/omnibus/43ABAFFD.bitsy.txt"),
            "43ABAFFD",
        );
    }
    #[test]
    fn test_43d24c5a() {
        str(
            include_str!("test-resources/omnibus/43D24C5A.bitsy.txt"),
            "43D24C5A",
        );
    }
    #[test]
    fn test_4483029c() {
        str(
            include_str!("test-resources/omnibus/4483029C.bitsy.txt"),
            "4483029C",
        );
    }
    #[test]
    fn test_4527a047() {
        str(
            include_str!("test-resources/omnibus/4527A047.bitsy.txt"),
            "4527A047",
        );
    }
    #[test]
    fn test_45ef9604() {
        str(
            include_str!("test-resources/omnibus/45EF9604.bitsy.txt"),
            "45EF9604",
        );
    }
    #[test]
    fn test_45f1225d() {
        str(
            include_str!("test-resources/omnibus/45F1225D.bitsy.txt"),
            "45F1225D",
        );
    }
    #[test]
    fn test_46062786() {
        str(
            include_str!("test-resources/omnibus/46062786.bitsy.txt"),
            "46062786",
        );
    }
    #[test]
    fn test_465efaa2() {
        str(
            include_str!("test-resources/omnibus/465EFAA2.bitsy.txt"),
            "465EFAA2",
        );
    }
    #[test]
    fn test_46a699a5() {
        str(
            include_str!("test-resources/omnibus/46A699A5.bitsy.txt"),
            "46A699A5",
        );
    }
    #[test]
    fn test_46eddcc6() {
        str(
            include_str!("test-resources/omnibus/46EDDCC6.bitsy.txt"),
            "46EDDCC6",
        );
    }
    #[test]
    fn test_4759957f() {
        str(
            include_str!("test-resources/omnibus/4759957F.bitsy.txt"),
            "4759957F",
        );
    }
    #[test]
    fn test_477a3731() {
        str(
            include_str!("test-resources/omnibus/477A3731.bitsy.txt"),
            "477A3731",
        );
    }
    #[test]
    fn test_4782f065() {
        str(
            include_str!("test-resources/omnibus/4782F065.bitsy.txt"),
            "4782F065",
        );
    }
    #[test]
    fn test_479de864() {
        str(
            include_str!("test-resources/omnibus/479DE864.bitsy.txt"),
            "479DE864",
        );
    }
    #[test]
    fn test_4941e947() {
        str(
            include_str!("test-resources/omnibus/4941E947.bitsy.txt"),
            "4941E947",
        );
    }
    #[test]
    fn test_49f56f40() {
        str(
            include_str!("test-resources/omnibus/49F56F40.bitsy.txt"),
            "49F56F40",
        );
    }
    #[test]
    fn test_4ab34220() {
        str(
            include_str!("test-resources/omnibus/4AB34220.bitsy.txt"),
            "4AB34220",
        );
    }
    #[test]
    fn test_4ac058fe() {
        str(
            include_str!("test-resources/omnibus/4AC058FE.bitsy.txt"),
            "4AC058FE",
        );
    }
    #[test]
    fn test_4b1e6a14() {
        str(
            include_str!("test-resources/omnibus/4B1E6A14.bitsy.txt"),
            "4B1E6A14",
        );
    }
    #[test]
    fn test_4c8ad705() {
        str(
            include_str!("test-resources/omnibus/4C8AD705.bitsy.txt"),
            "4C8AD705",
        );
    }
    #[test]
    fn test_4ddd9930() {
        str(
            include_str!("test-resources/omnibus/4DDD9930.bitsy.txt"),
            "4DDD9930",
        );
    }
    #[test]
    fn test_4ec3ae6f() {
        str(
            include_str!("test-resources/omnibus/4EC3AE6F.bitsy.txt"),
            "4EC3AE6F",
        );
    }
    #[test]
    fn test_50830651() {
        str(
            include_str!("test-resources/omnibus/50830651.bitsy.txt"),
            "50830651",
        );
    }
    #[test]
    fn test_512739f5() {
        str(
            include_str!("test-resources/omnibus/512739F5.bitsy.txt"),
            "512739F5",
        );
    }
    #[test]
    fn test_51dd0198() {
        str(
            include_str!("test-resources/omnibus/51DD0198.bitsy.txt"),
            "51DD0198",
        );
    }
    #[test]
    fn test_52629e0f() {
        str(
            include_str!("test-resources/omnibus/52629E0F.bitsy.txt"),
            "52629E0F",
        );
    }
    #[test]
    fn test_5295f9e7() {
        str(
            include_str!("test-resources/omnibus/5295F9E7.bitsy.txt"),
            "5295F9E7",
        );
    }
    #[test]
    fn test_52ad1de7() {
        str(
            include_str!("test-resources/omnibus/52AD1DE7.bitsy.txt"),
            "52AD1DE7",
        );
    }
    #[test]
    fn test_53b2a153() {
        str(
            include_str!("test-resources/omnibus/53B2A153.bitsy.txt"),
            "53B2A153",
        );
    }
    #[test]
    fn test_54bc2ac4() {
        str(
            include_str!("test-resources/omnibus/54BC2AC4.bitsy.txt"),
            "54BC2AC4",
        );
    }
    #[test]
    fn test_553ecb46() {
        str(
            include_str!("test-resources/omnibus/553ECB46.bitsy.txt"),
            "553ECB46",
        );
    }
    #[test]
    fn test_555e198c() {
        str(
            include_str!("test-resources/omnibus/555E198C.bitsy.txt"),
            "555E198C",
        );
    }
    #[test]
    fn test_55eb6535() {
        str(
            include_str!("test-resources/omnibus/55EB6535.bitsy.txt"),
            "55EB6535",
        );
    }
    #[test]
    fn test_5784dfef() {
        str(
            include_str!("test-resources/omnibus/5784DFEF.bitsy.txt"),
            "5784DFEF",
        );
    }
    #[test]
    fn test_5951f457() {
        str(
            include_str!("test-resources/omnibus/5951F457.bitsy.txt"),
            "5951F457",
        );
    }
    #[test]
    fn test_59b42152() {
        str(
            include_str!("test-resources/omnibus/59B42152.bitsy.txt"),
            "59B42152",
        );
    }
    #[test]
    fn test_59cfbdf2() {
        str(
            include_str!("test-resources/omnibus/59CFBDF2.bitsy.txt"),
            "59CFBDF2",
        );
    }
    #[test]
    fn test_59ea005d() {
        str(
            include_str!("test-resources/omnibus/59EA005D.bitsy.txt"),
            "59EA005D",
        );
    }
    #[test]
    fn test_5b337d40() {
        str(
            include_str!("test-resources/omnibus/5B337D40.bitsy.txt"),
            "5B337D40",
        );
    }
    #[test]
    fn test_5ba59ab1() {
        str(
            include_str!("test-resources/omnibus/5BA59AB1.bitsy.txt"),
            "5BA59AB1",
        );
    }
    #[test]
    fn test_5bcfd3b5() {
        str(
            include_str!("test-resources/omnibus/5BCFD3B5.bitsy.txt"),
            "5BCFD3B5",
        );
    }
    #[test]
    fn test_5d01e40d() {
        str(
            include_str!("test-resources/omnibus/5D01E40D.bitsy.txt"),
            "5D01E40D",
        );
    }
    #[test]
    fn test_5d0b4ec3() {
        str(
            include_str!("test-resources/omnibus/5D0B4EC3.bitsy.txt"),
            "5D0B4EC3",
        );
    }
    #[test]
    fn test_5d2b55a9() {
        str(
            include_str!("test-resources/omnibus/5D2B55A9.bitsy.txt"),
            "5D2B55A9",
        );
    }
    #[test]
    fn test_5d8aa0e5() {
        str(
            include_str!("test-resources/omnibus/5D8AA0E5.bitsy.txt"),
            "5D8AA0E5",
        );
    }
    #[test]
    fn test_5f836d20() {
        str(
            include_str!("test-resources/omnibus/5F836D20.bitsy.txt"),
            "5F836D20",
        );
    }
    #[test]
    fn test_5f977747() {
        str(
            include_str!("test-resources/omnibus/5F977747.bitsy.txt"),
            "5F977747",
        );
    }
    #[test]
    fn test_5fd6fa60() {
        str(
            include_str!("test-resources/omnibus/5FD6FA60.bitsy.txt"),
            "5FD6FA60",
        );
    }
    #[test]
    fn test_60289d2b() {
        str(
            include_str!("test-resources/omnibus/60289D2B.bitsy.txt"),
            "60289D2B",
        );
    }
    #[test]
    fn test_604ed83b() {
        str(
            include_str!("test-resources/omnibus/604ED83B.bitsy.txt"),
            "604ED83B",
        );
    }
    #[test]
    fn test_605e57d3() {
        str(
            include_str!("test-resources/omnibus/605E57D3.bitsy.txt"),
            "605E57D3",
        );
    }
    #[test]
    fn test_60a5c704() {
        str(
            include_str!("test-resources/omnibus/60A5C704.bitsy.txt"),
            "60A5C704",
        );
    }
    #[test]
    fn test_610930d7() {
        str(
            include_str!("test-resources/omnibus/610930D7.bitsy.txt"),
            "610930D7",
        );
    }
    #[test]
    fn test_61d4df26() {
        str(
            include_str!("test-resources/omnibus/61D4DF26.bitsy.txt"),
            "61D4DF26",
        );
    }
    #[test]
    fn test_6227c33b() {
        str(
            include_str!("test-resources/omnibus/6227C33B.bitsy.txt"),
            "6227C33B",
        );
    }
    #[test]
    fn test_629744da() {
        str(
            include_str!("test-resources/omnibus/629744DA.bitsy.txt"),
            "629744DA",
        );
    }
    #[test]
    fn test_640b5e37() {
        str(
            include_str!("test-resources/omnibus/640B5E37.bitsy.txt"),
            "640B5E37",
        );
    }
    #[test]
    fn test_643e26e7() {
        str(
            include_str!("test-resources/omnibus/643E26E7.bitsy.txt"),
            "643E26E7",
        );
    }
    #[test]
    fn test_657029d2() {
        str(
            include_str!("test-resources/omnibus/657029D2.bitsy.txt"),
            "657029D2",
        );
    }
    #[test]
    fn test_65821da2() {
        str(
            include_str!("test-resources/omnibus/65821DA2.bitsy.txt"),
            "65821DA2",
        );
    }
    #[test]
    fn test_65c2b499() {
        str(
            include_str!("test-resources/omnibus/65C2B499.bitsy.txt"),
            "65C2B499",
        );
    }
    #[test]
    fn test_66888415() {
        str(
            include_str!("test-resources/omnibus/66888415.bitsy.txt"),
            "66888415",
        );
    }
    #[test]
    fn test_68005325() {
        str(
            include_str!("test-resources/omnibus/68005325.bitsy.txt"),
            "68005325",
        );
    }
    #[test]
    fn test_682993ac() {
        str(
            include_str!("test-resources/omnibus/682993AC.bitsy.txt"),
            "682993AC",
        );
    }
    #[test]
    fn test_69e5adde() {
        str(
            include_str!("test-resources/omnibus/69E5ADDE.bitsy.txt"),
            "69E5ADDE",
        );
    }
    #[test]
    fn test_6a3083f8() {
        str(
            include_str!("test-resources/omnibus/6A3083F8.bitsy.txt"),
            "6A3083F8",
        );
    }
    #[test]
    fn test_6a934776() {
        str(
            include_str!("test-resources/omnibus/6A934776.bitsy.txt"),
            "6A934776",
        );
    }
    #[test]
    fn test_6c3d076c() {
        str(
            include_str!("test-resources/omnibus/6C3D076C.bitsy.txt"),
            "6C3D076C",
        );
    }
    #[test]
    fn test_6c491749() {
        str(
            include_str!("test-resources/omnibus/6C491749.bitsy.txt"),
            "6C491749",
        );
    }
    #[test]
    fn test_6cf290c7() {
        str(
            include_str!("test-resources/omnibus/6CF290C7.bitsy.txt"),
            "6CF290C7",
        );
    }
    #[test]
    fn test_6d9a411a() {
        str(
            include_str!("test-resources/omnibus/6D9A411A.bitsy.txt"),
            "6D9A411A",
        );
    }
    #[test]
    fn test_6e835b7d() {
        str(
            include_str!("test-resources/omnibus/6E835B7D.bitsy.txt"),
            "6E835B7D",
        );
    }
    #[test]
    fn test_6f136b0b() {
        str(
            include_str!("test-resources/omnibus/6F136B0B.bitsy.txt"),
            "6F136B0B",
        );
    }
    #[test]
    fn test_70e9483e() {
        str(
            include_str!("test-resources/omnibus/70E9483E.bitsy.txt"),
            "70E9483E",
        );
    }
    #[test]
    fn test_70ea1ca6() {
        str(
            include_str!("test-resources/omnibus/70EA1CA6.bitsy.txt"),
            "70EA1CA6",
        );
    }
    #[test]
    fn test_710f4731() {
        str(
            include_str!("test-resources/omnibus/710F4731.bitsy.txt"),
            "710F4731",
        );
    }
    #[test]
    fn test_7129037b() {
        str(
            include_str!("test-resources/omnibus/7129037B.bitsy.txt"),
            "7129037B",
        );
    }
    #[test]
    fn test_725b9251() {
        str(
            include_str!("test-resources/omnibus/725B9251.bitsy.txt"),
            "725B9251",
        );
    }
    #[test]
    fn test_73d4bbde() {
        str(
            include_str!("test-resources/omnibus/73D4BBDE.bitsy.txt"),
            "73D4BBDE",
        );
    }
    #[test]
    fn test_74685d79() {
        str(
            include_str!("test-resources/omnibus/74685D79.bitsy.txt"),
            "74685D79",
        );
    }
    #[test]
    fn test_74802fe3() {
        str(
            include_str!("test-resources/omnibus/74802FE3.bitsy.txt"),
            "74802FE3",
        );
    }
    #[test]
    fn test_748ea164() {
        str(
            include_str!("test-resources/omnibus/748EA164.bitsy.txt"),
            "748EA164",
        );
    }
    #[test]
    fn test_748f77b5() {
        str(
            include_str!("test-resources/omnibus/748F77B5.bitsy.txt"),
            "748F77B5",
        );
    }
    #[test]
    fn test_74e0f6ef() {
        str(
            include_str!("test-resources/omnibus/74E0F6EF.bitsy.txt"),
            "74E0F6EF",
        );
    }
    #[test]
    fn test_750d339d() {
        str(
            include_str!("test-resources/omnibus/750D339D.bitsy.txt"),
            "750D339D",
        );
    }
    #[test]
    fn test_755da50e() {
        str(
            include_str!("test-resources/omnibus/755DA50E.bitsy.txt"),
            "755DA50E",
        );
    }
    #[test]
    fn test_75b61d2b() {
        str(
            include_str!("test-resources/omnibus/75B61D2B.bitsy.txt"),
            "75B61D2B",
        );
    }
    #[test]
    fn test_75e04506() {
        str(
            include_str!("test-resources/omnibus/75E04506.bitsy.txt"),
            "75E04506",
        );
    }
    #[test]
    fn test_7698b32e() {
        str(
            include_str!("test-resources/omnibus/7698B32E.bitsy.txt"),
            "7698B32E",
        );
    }
    #[test]
    fn test_7769825b() {
        str(
            include_str!("test-resources/omnibus/7769825B.bitsy.txt"),
            "7769825B",
        );
    }
    #[test]
    fn test_77c2a64a() {
        str(
            include_str!("test-resources/omnibus/77C2A64A.bitsy.txt"),
            "77C2A64A",
        );
    }
    #[test]
    fn test_78a27fa0() {
        str(
            include_str!("test-resources/omnibus/78A27FA0.bitsy.txt"),
            "78A27FA0",
        );
    }
    #[test]
    fn test_79ff4e48() {
        str(
            include_str!("test-resources/omnibus/79FF4E48.bitsy.txt"),
            "79FF4E48",
        );
    }
    #[test]
    fn test_7ac4008f() {
        str(
            include_str!("test-resources/omnibus/7AC4008F.bitsy.txt"),
            "7AC4008F",
        );
    }
    #[test]
    fn test_7adf3924() {
        str(
            include_str!("test-resources/omnibus/7ADF3924.bitsy.txt"),
            "7ADF3924",
        );
    }
    #[test]
    fn test_7bc643cf() {
        str(
            include_str!("test-resources/omnibus/7BC643CF.bitsy.txt"),
            "7BC643CF",
        );
    }
    #[test]
    fn test_7bd4c1e0() {
        str(
            include_str!("test-resources/omnibus/7BD4C1E0.bitsy.txt"),
            "7BD4C1E0",
        );
    }
    #[test]
    fn test_7c0211e8() {
        str(
            include_str!("test-resources/omnibus/7C0211E8.bitsy.txt"),
            "7C0211E8",
        );
    }
    #[test]
    fn test_7c57e56e() {
        str(
            include_str!("test-resources/omnibus/7C57E56E.bitsy.txt"),
            "7C57E56E",
        );
    }
    #[test]
    fn test_7ca8eff9() {
        str(
            include_str!("test-resources/omnibus/7CA8EFF9.bitsy.txt"),
            "7CA8EFF9",
        );
    }
    #[test]
    fn test_7cb22bec() {
        str(
            include_str!("test-resources/omnibus/7CB22BEC.bitsy.txt"),
            "7CB22BEC",
        );
    }
    #[test]
    fn test_7cc82262() {
        str(
            include_str!("test-resources/omnibus/7CC82262.bitsy.txt"),
            "7CC82262",
        );
    }
    #[test]
    fn test_7dc64c62() {
        str(
            include_str!("test-resources/omnibus/7DC64C62.bitsy.txt"),
            "7DC64C62",
        );
    }
    #[test]
    fn test_7df36da4() {
        str(
            include_str!("test-resources/omnibus/7DF36DA4.bitsy.txt"),
            "7DF36DA4",
        );
    }
    #[test]
    fn test_7f1599d1() {
        str(
            include_str!("test-resources/omnibus/7F1599D1.bitsy.txt"),
            "7F1599D1",
        );
    }
    #[test]
    fn test_7f607cfa() {
        str(
            include_str!("test-resources/omnibus/7F607CFA.bitsy.txt"),
            "7F607CFA",
        );
    }
    #[test]
    fn test_7fbbc26e() {
        str(
            include_str!("test-resources/omnibus/7FBBC26E.bitsy.txt"),
            "7FBBC26E",
        );
    }
    #[test]
    fn test_7fcf6d96() {
        str(
            include_str!("test-resources/omnibus/7FCF6D96.bitsy.txt"),
            "7FCF6D96",
        );
    }
    #[test]
    fn test_801ae85a() {
        str(
            include_str!("test-resources/omnibus/801AE85A.bitsy.txt"),
            "801AE85A",
        );
    }
    #[test]
    fn test_8025521d() {
        str(
            include_str!("test-resources/omnibus/8025521D.bitsy.txt"),
            "8025521D",
        );
    }
    #[test]
    fn test_8059564a() {
        str(
            include_str!("test-resources/omnibus/8059564A.bitsy.txt"),
            "8059564A",
        );
    }
    #[test]
    fn test_807805cc() {
        str(
            include_str!("test-resources/omnibus/807805CC.bitsy.txt"),
            "807805CC",
        );
    }
    #[test]
    fn test_808ea54b() {
        str(
            include_str!("test-resources/omnibus/808EA54B.bitsy.txt"),
            "808EA54B",
        );
    }
    #[test]
    fn test_81875aec() {
        str(
            include_str!("test-resources/omnibus/81875AEC.bitsy.txt"),
            "81875AEC",
        );
    }
    #[test]
    fn test_82dde16f() {
        str(
            include_str!("test-resources/omnibus/82DDE16F.bitsy.txt"),
            "82DDE16F",
        );
    }
    #[test]
    fn test_83402ede() {
        str(
            include_str!("test-resources/omnibus/83402EDE.bitsy.txt"),
            "83402EDE",
        );
    }
    #[test]
    fn test_83db1fec() {
        str(
            include_str!("test-resources/omnibus/83DB1FEC.bitsy.txt"),
            "83DB1FEC",
        );
    }
    #[test]
    fn test_84ee182e() {
        str(
            include_str!("test-resources/omnibus/84EE182E.bitsy.txt"),
            "84EE182E",
        );
    }
    #[test]
    fn test_85319efc() {
        str(
            include_str!("test-resources/omnibus/85319EFC.bitsy.txt"),
            "85319EFC",
        );
    }
    #[test]
    fn test_85d7ae4c() {
        str(
            include_str!("test-resources/omnibus/85D7AE4C.bitsy.txt"),
            "85D7AE4C",
        );
    }
    #[test]
    fn test_865d97a2() {
        str(
            include_str!("test-resources/omnibus/865D97A2.bitsy.txt"),
            "865D97A2",
        );
    }
    #[test]
    fn test_86cc164a() {
        str(
            include_str!("test-resources/omnibus/86CC164A.bitsy.txt"),
            "86CC164A",
        );
    }
    #[test]
    fn test_8749dd6f() {
        str(
            include_str!("test-resources/omnibus/8749DD6F.bitsy.txt"),
            "8749DD6F",
        );
    }
    #[test]
    fn test_874b5bd3() {
        str(
            include_str!("test-resources/omnibus/874B5BD3.bitsy.txt"),
            "874B5BD3",
        );
    }
    #[test]
    fn test_87d1fe22() {
        str(
            include_str!("test-resources/omnibus/87D1FE22.bitsy.txt"),
            "87D1FE22",
        );
    }
    #[test]
    fn test_8831a581() {
        str(
            include_str!("test-resources/omnibus/8831A581.bitsy.txt"),
            "8831A581",
        );
    }
    #[test]
    fn test_88465670() {
        str(
            include_str!("test-resources/omnibus/88465670.bitsy.txt"),
            "88465670",
        );
    }
    #[test]
    fn test_88599170() {
        str(
            include_str!("test-resources/omnibus/88599170.bitsy.txt"),
            "88599170",
        );
    }
    #[test]
    fn test_89551e9a() {
        str(
            include_str!("test-resources/omnibus/89551E9A.bitsy.txt"),
            "89551E9A",
        );
    }
    #[test]
    fn test_89711e3e() {
        str(
            include_str!("test-resources/omnibus/89711E3E.bitsy.txt"),
            "89711E3E",
        );
    }
    #[test]
    fn test_89c6ed6a() {
        str(
            include_str!("test-resources/omnibus/89C6ED6A.bitsy.txt"),
            "89C6ED6A",
        );
    }
    #[test]
    fn test_8c057b81() {
        str(
            include_str!("test-resources/omnibus/8C057B81.bitsy.txt"),
            "8C057B81",
        );
    }
    #[test]
    fn test_8cabe4c9() {
        str(
            include_str!("test-resources/omnibus/8CABE4C9.bitsy.txt"),
            "8CABE4C9",
        );
    }
    #[test]
    fn test_8d020129() {
        str(
            include_str!("test-resources/omnibus/8D020129.bitsy.txt"),
            "8D020129",
        );
    }
    #[test]
    fn test_8da05131() {
        str(
            include_str!("test-resources/omnibus/8DA05131.bitsy.txt"),
            "8DA05131",
        );
    }
    #[test]
    fn test_8db7cf07() {
        str(
            include_str!("test-resources/omnibus/8DB7CF07.bitsy.txt"),
            "8DB7CF07",
        );
    }
    #[test]
    fn test_8e4187fd() {
        str(
            include_str!("test-resources/omnibus/8E4187FD.bitsy.txt"),
            "8E4187FD",
        );
    }
    #[test]
    fn test_8ef8319b() {
        str(
            include_str!("test-resources/omnibus/8EF8319B.bitsy.txt"),
            "8EF8319B",
        );
    }
    #[test]
    fn test_8f7a6fb5() {
        str(
            include_str!("test-resources/omnibus/8F7A6FB5.bitsy.txt"),
            "8F7A6FB5",
        );
    }
    #[test]
    fn test_8fedb06b() {
        str(
            include_str!("test-resources/omnibus/8FEDB06B.bitsy.txt"),
            "8FEDB06B",
        );
    }
    #[test]
    fn test_9074c1cd() {
        str(
            include_str!("test-resources/omnibus/9074C1CD.bitsy.txt"),
            "9074C1CD",
        );
    }
    #[test]
    fn test_924d038f() {
        str(
            include_str!("test-resources/omnibus/924D038F.bitsy.txt"),
            "924D038F",
        );
    }
    #[test]
    fn test_926a622e() {
        str(
            include_str!("test-resources/omnibus/926A622E.bitsy.txt"),
            "926A622E",
        );
    }
    #[test]
    fn test_92d589b9() {
        str(
            include_str!("test-resources/omnibus/92D589B9.bitsy.txt"),
            "92D589B9",
        );
    }
    #[test]
    fn test_9372ade8() {
        str(
            include_str!("test-resources/omnibus/9372ADE8.bitsy.txt"),
            "9372ADE8",
        );
    }
    #[test]
    fn test_938468ba() {
        str(
            include_str!("test-resources/omnibus/938468BA.bitsy.txt"),
            "938468BA",
        );
    }
    #[test]
    fn test_94612f67() {
        str(
            include_str!("test-resources/omnibus/94612F67.bitsy.txt"),
            "94612F67",
        );
    }
    #[test]
    fn test_946e85b3() {
        str(
            include_str!("test-resources/omnibus/946E85B3.bitsy.txt"),
            "946E85B3",
        );
    }
    #[test]
    fn test_947c7544() {
        str(
            include_str!("test-resources/omnibus/947C7544.bitsy.txt"),
            "947C7544",
        );
    }
    #[test]
    fn test_9484fd34() {
        str(
            include_str!("test-resources/omnibus/9484FD34.bitsy.txt"),
            "9484FD34",
        );
    }
    #[test]
    fn test_94943e5c() {
        str(
            include_str!("test-resources/omnibus/94943E5C.bitsy.txt"),
            "94943E5C",
        );
    }
    #[test]
    fn test_94fd6705() {
        str(
            include_str!("test-resources/omnibus/94FD6705.bitsy.txt"),
            "94FD6705",
        );
    }
    #[test]
    fn test_955b75c9() {
        str(
            include_str!("test-resources/omnibus/955B75C9.bitsy.txt"),
            "955B75C9",
        );
    }
    #[test]
    fn test_967dfe88() {
        str(
            include_str!("test-resources/omnibus/967DFE88.bitsy.txt"),
            "967DFE88",
        );
    }
    #[test]
    fn test_9724bf5e() {
        str(
            include_str!("test-resources/omnibus/9724BF5E.bitsy.txt"),
            "9724BF5E",
        );
    }
    #[test]
    fn test_974aa125() {
        str(
            include_str!("test-resources/omnibus/974AA125.bitsy.txt"),
            "974AA125",
        );
    }
    #[test]
    fn test_976e1d47() {
        str(
            include_str!("test-resources/omnibus/976E1D47.bitsy.txt"),
            "976E1D47",
        );
    }
    #[test]
    fn test_97812dac() {
        str(
            include_str!("test-resources/omnibus/97812DAC.bitsy.txt"),
            "97812DAC",
        );
    }
    #[test]
    fn test_98509df4() {
        str(
            include_str!("test-resources/omnibus/98509DF4.bitsy.txt"),
            "98509DF4",
        );
    }
    #[test]
    fn test_98b2b460() {
        str(
            include_str!("test-resources/omnibus/98B2B460.bitsy.txt"),
            "98B2B460",
        );
    }
    #[test]
    fn test_9927728d() {
        str(
            include_str!("test-resources/omnibus/9927728D.bitsy.txt"),
            "9927728D",
        );
    }
    #[test]
    fn test_99d3df43() {
        str(
            include_str!("test-resources/omnibus/99D3DF43.bitsy.txt"),
            "99D3DF43",
        );
    }
    #[test]
    fn test_99ea5769() {
        str(
            include_str!("test-resources/omnibus/99EA5769.bitsy.txt"),
            "99EA5769",
        );
    }
    #[test]
    fn test_9acccd68() {
        str(
            include_str!("test-resources/omnibus/9ACCCD68.bitsy.txt"),
            "9ACCCD68",
        );
    }
    #[test]
    fn test_9ad2a574() {
        str(
            include_str!("test-resources/omnibus/9AD2A574.bitsy.txt"),
            "9AD2A574",
        );
    }
    #[test]
    fn test_9b411e31() {
        str(
            include_str!("test-resources/omnibus/9B411E31.bitsy.txt"),
            "9B411E31",
        );
    }
    #[test]
    fn test_9c8ecf36() {
        str(
            include_str!("test-resources/omnibus/9C8ECF36.bitsy.txt"),
            "9C8ECF36",
        );
    }
    #[test]
    fn test_9da3b70d() {
        str(
            include_str!("test-resources/omnibus/9DA3B70D.bitsy.txt"),
            "9DA3B70D",
        );
    }
    #[test]
    fn test_9dad710d() {
        str(
            include_str!("test-resources/omnibus/9DAD710D.bitsy.txt"),
            "9DAD710D",
        );
    }
    #[test]
    fn test_9de4c3ba() {
        str(
            include_str!("test-resources/omnibus/9DE4C3BA.bitsy.txt"),
            "9DE4C3BA",
        );
    }
    #[test]
    fn test_9f39737d() {
        str(
            include_str!("test-resources/omnibus/9F39737D.bitsy.txt"),
            "9F39737D",
        );
    }
    #[test]
    fn test_9fcfddbc() {
        str(
            include_str!("test-resources/omnibus/9FCFDDBC.bitsy.txt"),
            "9FCFDDBC",
        );
    }
    #[test]
    fn test_a09d5907() {
        str(
            include_str!("test-resources/omnibus/A09D5907.bitsy.txt"),
            "A09D5907",
        );
    }
    #[test]
    fn test_a19f89f1() {
        str(
            include_str!("test-resources/omnibus/A19F89F1.bitsy.txt"),
            "A19F89F1",
        );
    }
    #[test]
    fn test_a1ae038d() {
        str(
            include_str!("test-resources/omnibus/A1AE038D.bitsy.txt"),
            "A1AE038D",
        );
    }
    #[test]
    fn test_a1b12872() {
        str(
            include_str!("test-resources/omnibus/A1B12872.bitsy.txt"),
            "A1B12872",
        );
    }
    #[test]
    fn test_a3b29805() {
        str(
            include_str!("test-resources/omnibus/A3B29805.bitsy.txt"),
            "A3B29805",
        );
    }
    #[test]
    fn test_a418d2ab() {
        str(
            include_str!("test-resources/omnibus/A418D2AB.bitsy.txt"),
            "A418D2AB",
        );
    }
    #[test]
    fn test_a5816c9c() {
        str(
            include_str!("test-resources/omnibus/A5816C9C.bitsy.txt"),
            "A5816C9C",
        );
    }
    #[test]
    fn test_a5cfeed6() {
        str(
            include_str!("test-resources/omnibus/A5CFEED6.bitsy.txt"),
            "A5CFEED6",
        );
    }
    #[test]
    fn test_a7183583() {
        str(
            include_str!("test-resources/omnibus/A7183583.bitsy.txt"),
            "A7183583",
        );
    }
    #[test]
    fn test_a7df882e() {
        str(
            include_str!("test-resources/omnibus/A7DF882E.bitsy.txt"),
            "A7DF882E",
        );
    }
    #[test]
    fn test_a7f51ae4() {
        str(
            include_str!("test-resources/omnibus/A7F51AE4.bitsy.txt"),
            "A7F51AE4",
        );
    }
    #[test]
    fn test_a82f4321() {
        str(
            include_str!("test-resources/omnibus/A82F4321.bitsy.txt"),
            "A82F4321",
        );
    }
    #[test]
    fn test_a8d107b2() {
        str(
            include_str!("test-resources/omnibus/A8D107B2.bitsy.txt"),
            "A8D107B2",
        );
    }
    #[test]
    fn test_aa007e54() {
        str(
            include_str!("test-resources/omnibus/AA007E54.bitsy.txt"),
            "AA007E54",
        );
    }
    #[test]
    fn test_ab368cec() {
        str(
            include_str!("test-resources/omnibus/AB368CEC.bitsy.txt"),
            "AB368CEC",
        );
    }
    #[test]
    fn test_ac7e9f07() {
        str(
            include_str!("test-resources/omnibus/AC7E9F07.bitsy.txt"),
            "AC7E9F07",
        );
    }
    #[test]
    fn test_ad97fbc3() {
        str(
            include_str!("test-resources/omnibus/AD97FBC3.bitsy.txt"),
            "AD97FBC3",
        );
    }
    #[test]
    fn test_adb7c913() {
        str(
            include_str!("test-resources/omnibus/ADB7C913.bitsy.txt"),
            "ADB7C913",
        );
    }
    #[test]
    fn test_afa24079() {
        str(
            include_str!("test-resources/omnibus/AFA24079.bitsy.txt"),
            "AFA24079",
        );
    }
    #[test]
    fn test_b16974f7() {
        str(
            include_str!("test-resources/omnibus/B16974F7.bitsy.txt"),
            "B16974F7",
        );
    }
    #[test]
    fn test_b34f07d5() {
        str(
            include_str!("test-resources/omnibus/B34F07D5.bitsy.txt"),
            "B34F07D5",
        );
    }
    #[test]
    fn test_b53d6e92() {
        str(
            include_str!("test-resources/omnibus/B53D6E92.bitsy.txt"),
            "B53D6E92",
        );
    }
    #[test]
    fn test_b545feac() {
        str(
            include_str!("test-resources/omnibus/B545FEAC.bitsy.txt"),
            "B545FEAC",
        );
    }
    #[test]
    fn test_b6370aff() {
        str(
            include_str!("test-resources/omnibus/B6370AFF.bitsy.txt"),
            "B6370AFF",
        );
    }
    #[test]
    fn test_b74f7f7a() {
        str(
            include_str!("test-resources/omnibus/B74F7F7A.bitsy.txt"),
            "B74F7F7A",
        );
    }
    #[test]
    fn test_b7b61117() {
        str(
            include_str!("test-resources/omnibus/B7B61117.bitsy.txt"),
            "B7B61117",
        );
    }
    #[test]
    fn test_b831fb24() {
        str(
            include_str!("test-resources/omnibus/B831FB24.bitsy.txt"),
            "B831FB24",
        );
    }
    #[test]
    fn test_b8e10418() {
        str(
            include_str!("test-resources/omnibus/B8E10418.bitsy.txt"),
            "B8E10418",
        );
    }
    #[test]
    fn test_b9e61bc6() {
        str(
            include_str!("test-resources/omnibus/B9E61BC6.bitsy.txt"),
            "B9E61BC6",
        );
    }
    #[test]
    fn test_ba102197() {
        str(
            include_str!("test-resources/omnibus/BA102197.bitsy.txt"),
            "BA102197",
        );
    }
    #[test]
    fn test_bb5d0d6c() {
        str(
            include_str!("test-resources/omnibus/BB5D0D6C.bitsy.txt"),
            "BB5D0D6C",
        );
    }
    #[test]
    fn test_bbb98596() {
        str(
            include_str!("test-resources/omnibus/BBB98596.bitsy.txt"),
            "BBB98596",
        );
    }
    #[test]
    fn test_bc0dce5c() {
        str(
            include_str!("test-resources/omnibus/BC0DCE5C.bitsy.txt"),
            "BC0DCE5C",
        );
    }
    #[test]
    fn test_bc8b3e64() {
        str(
            include_str!("test-resources/omnibus/BC8B3E64.bitsy.txt"),
            "BC8B3E64",
        );
    }
    #[test]
    fn test_bcc99eb5() {
        str(
            include_str!("test-resources/omnibus/BCC99EB5.bitsy.txt"),
            "BCC99EB5",
        );
    }
    #[test]
    fn test_bd621b6d() {
        str(
            include_str!("test-resources/omnibus/BD621B6D.bitsy.txt"),
            "BD621B6D",
        );
    }
    #[test]
    fn test_bdd57b7d() {
        str(
            include_str!("test-resources/omnibus/BDD57B7D.bitsy.txt"),
            "BDD57B7D",
        );
    }
    #[test]
    fn test_bebd5115() {
        str(
            include_str!("test-resources/omnibus/BEBD5115.bitsy.txt"),
            "BEBD5115",
        );
    }
    #[test]
    fn test_bf15da66() {
        str(
            include_str!("test-resources/omnibus/BF15DA66.bitsy.txt"),
            "BF15DA66",
        );
    }
    #[test]
    fn test_c03befc8() {
        str(
            include_str!("test-resources/omnibus/C03BEFC8.bitsy.txt"),
            "C03BEFC8",
        );
    }
    #[test]
    fn test_c0be7f65() {
        str(
            include_str!("test-resources/omnibus/C0BE7F65.bitsy.txt"),
            "C0BE7F65",
        );
    }
    #[test]
    fn test_c1dc0328() {
        str(
            include_str!("test-resources/omnibus/C1DC0328.bitsy.txt"),
            "C1DC0328",
        );
    }
    #[test]
    fn test_c2493877() {
        str(
            include_str!("test-resources/omnibus/C2493877.bitsy.txt"),
            "C2493877",
        );
    }
    #[test]
    fn test_c2ef387a() {
        str(
            include_str!("test-resources/omnibus/C2EF387A.bitsy.txt"),
            "C2EF387A",
        );
    }
    #[test]
    fn test_c353f7cf() {
        str(
            include_str!("test-resources/omnibus/C353F7CF.bitsy.txt"),
            "C353F7CF",
        );
    }
    #[test]
    fn test_c49c625e() {
        str(
            include_str!("test-resources/omnibus/C49C625E.bitsy.txt"),
            "C49C625E",
        );
    }
    #[test]
    fn test_c50eb781() {
        str(
            include_str!("test-resources/omnibus/C50EB781.bitsy.txt"),
            "C50EB781",
        );
    }
    #[test]
    fn test_c5cf3fda() {
        str(
            include_str!("test-resources/omnibus/C5CF3FDA.bitsy.txt"),
            "C5CF3FDA",
        );
    }
    #[test]
    fn test_c6617fa2() {
        str(
            include_str!("test-resources/omnibus/C6617FA2.bitsy.txt"),
            "C6617FA2",
        );
    }
    #[test]
    fn test_c673ea1b() {
        str(
            include_str!("test-resources/omnibus/C673EA1B.bitsy.txt"),
            "C673EA1B",
        );
    }
    #[test]
    fn test_c6e7831c() {
        str(
            include_str!("test-resources/omnibus/C6E7831C.bitsy.txt"),
            "C6E7831C",
        );
    }
    #[test]
    fn test_c8421383() {
        str(
            include_str!("test-resources/omnibus/C8421383.bitsy.txt"),
            "C8421383",
        );
    }
    #[test]
    fn test_c872d239() {
        str(
            include_str!("test-resources/omnibus/C872D239.bitsy.txt"),
            "C872D239",
        );
    }
    #[test]
    fn test_c87e0b9f() {
        str(
            include_str!("test-resources/omnibus/C87E0B9F.bitsy.txt"),
            "C87E0B9F",
        );
    }
    #[test]
    fn test_c8c0d6bd() {
        str(
            include_str!("test-resources/omnibus/C8C0D6BD.bitsy.txt"),
            "C8C0D6BD",
        );
    }
    #[test]
    fn test_c9ad2be0() {
        str(
            include_str!("test-resources/omnibus/C9AD2BE0.bitsy.txt"),
            "C9AD2BE0",
        );
    }
    #[test]
    fn test_cb70cded() {
        str(
            include_str!("test-resources/omnibus/CB70CDED.bitsy.txt"),
            "CB70CDED",
        );
    }
    #[test]
    fn test_cbb105d7() {
        str(
            include_str!("test-resources/omnibus/CBB105D7.bitsy.txt"),
            "CBB105D7",
        );
    }
    #[test]
    fn test_cbbfb107() {
        str(
            include_str!("test-resources/omnibus/CBBFB107.bitsy.txt"),
            "CBBFB107",
        );
    }
    #[test]
    fn test_cbf9a0c0() {
        str(
            include_str!("test-resources/omnibus/CBF9A0C0.bitsy.txt"),
            "CBF9A0C0",
        );
    }
    #[test]
    fn test_cc34457e() {
        str(
            include_str!("test-resources/omnibus/CC34457E.bitsy.txt"),
            "CC34457E",
        );
    }
    #[test]
    fn test_cd609e8e() {
        str(
            include_str!("test-resources/omnibus/CD609E8E.bitsy.txt"),
            "CD609E8E",
        );
    }
    #[test]
    fn test_ce0512b2() {
        str(
            include_str!("test-resources/omnibus/CE0512B2.bitsy.txt"),
            "CE0512B2",
        );
    }
    #[test]
    fn test_ce59a086() {
        str(
            include_str!("test-resources/omnibus/CE59A086.bitsy.txt"),
            "CE59A086",
        );
    }
    #[test]
    fn test_cf890145() {
        str(
            include_str!("test-resources/omnibus/CF890145.bitsy.txt"),
            "CF890145",
        );
    }
    #[test]
    fn test_cf89e555() {
        str(
            include_str!("test-resources/omnibus/CF89E555.bitsy.txt"),
            "CF89E555",
        );
    }
    #[test]
    fn test_cfbe5bdf() {
        str(
            include_str!("test-resources/omnibus/CFBE5BDF.bitsy.txt"),
            "CFBE5BDF",
        );
    }
    #[test]
    fn test_cfe62f11() {
        str(
            include_str!("test-resources/omnibus/CFE62F11.bitsy.txt"),
            "CFE62F11",
        );
    }
    #[test]
    fn test_cff98f1a() {
        str(
            include_str!("test-resources/omnibus/CFF98F1A.bitsy.txt"),
            "CFF98F1A",
        );
    }
    #[test]
    fn test_d07caf2a() {
        str(
            include_str!("test-resources/omnibus/D07CAF2A.bitsy.txt"),
            "D07CAF2A",
        );
    }
    #[test]
    fn test_d173c9c5() {
        str(
            include_str!("test-resources/omnibus/D173C9C5.bitsy.txt"),
            "D173C9C5",
        );
    }
    #[test]
    fn test_d1be479b() {
        str(
            include_str!("test-resources/omnibus/D1BE479B.bitsy.txt"),
            "D1BE479B",
        );
    }
    #[test]
    fn test_d1fb278a() {
        str(
            include_str!("test-resources/omnibus/D1FB278A.bitsy.txt"),
            "D1FB278A",
        );
    }
    #[test]
    fn test_d2a4d690() {
        str(
            include_str!("test-resources/omnibus/D2A4D690.bitsy.txt"),
            "D2A4D690",
        );
    }
    #[test]
    fn test_d336f626() {
        str(
            include_str!("test-resources/omnibus/D336F626.bitsy.txt"),
            "D336F626",
        );
    }
    #[test]
    fn test_d35b2e27() {
        str(
            include_str!("test-resources/omnibus/D35B2E27.bitsy.txt"),
            "D35B2E27",
        );
    }
    #[test]
    fn test_d373f018() {
        str(
            include_str!("test-resources/omnibus/D373F018.bitsy.txt"),
            "D373F018",
        );
    }
    #[test]
    fn test_d3a5c1d7() {
        str(
            include_str!("test-resources/omnibus/D3A5C1D7.bitsy.txt"),
            "D3A5C1D7",
        );
    }
    #[test]
    fn test_d498eefa() {
        str(
            include_str!("test-resources/omnibus/D498EEFA.bitsy.txt"),
            "D498EEFA",
        );
    }
    #[test]
    fn test_d53b7c03() {
        str(
            include_str!("test-resources/omnibus/D53B7C03.bitsy.txt"),
            "D53B7C03",
        );
    }
    #[test]
    fn test_d60e6d7f() {
        str(
            include_str!("test-resources/omnibus/D60E6D7F.bitsy.txt"),
            "D60E6D7F",
        );
    }
    #[test]
    fn test_d8480dc7() {
        str(
            include_str!("test-resources/omnibus/D8480DC7.bitsy.txt"),
            "D8480DC7",
        );
    }
    #[test]
    fn test_d84cbfff() {
        str(
            include_str!("test-resources/omnibus/D84CBFFF.bitsy.txt"),
            "D84CBFFF",
        );
    }
    #[test]
    fn test_d903b657() {
        str(
            include_str!("test-resources/omnibus/D903B657.bitsy.txt"),
            "D903B657",
        );
    }
    #[test]
    fn test_da70c62c() {
        str(
            include_str!("test-resources/omnibus/DA70C62C.bitsy.txt"),
            "DA70C62C",
        );
    }
    #[test]
    fn test_da88c287() {
        str(
            include_str!("test-resources/omnibus/DA88C287.bitsy.txt"),
            "DA88C287",
        );
    }
    #[test]
    fn test_dabc1b16() {
        str(
            include_str!("test-resources/omnibus/DABC1B16.bitsy.txt"),
            "DABC1B16",
        );
    }
    #[test]
    fn test_db59a848() {
        str(
            include_str!("test-resources/omnibus/DB59A848.bitsy.txt"),
            "DB59A848",
        );
    }
    #[test]
    fn test_db74abe2() {
        str(
            include_str!("test-resources/omnibus/DB74ABE2.bitsy.txt"),
            "DB74ABE2",
        );
    }
    #[test]
    fn test_dbd5d375() {
        str(
            include_str!("test-resources/omnibus/DBD5D375.bitsy.txt"),
            "DBD5D375",
        );
    }
    #[test]
    fn test_dc053b1a() {
        str(
            include_str!("test-resources/omnibus/DC053B1A.bitsy.txt"),
            "DC053B1A",
        );
    }
    #[test]
    fn test_dcdd7569() {
        str(
            include_str!("test-resources/omnibus/DCDD7569.bitsy.txt"),
            "DCDD7569",
        );
    }
    #[test]
    fn test_dd5be55b() {
        str(
            include_str!("test-resources/omnibus/DD5BE55B.bitsy.txt"),
            "DD5BE55B",
        );
    }
    #[test]
    fn test_de25b125() {
        str(
            include_str!("test-resources/omnibus/DE25B125.bitsy.txt"),
            "DE25B125",
        );
    }
    #[test]
    fn test_ded097eb() {
        str(
            include_str!("test-resources/omnibus/DED097EB.bitsy.txt"),
            "DED097EB",
        );
    }
    #[test]
    fn test_df0ba198() {
        str(
            include_str!("test-resources/omnibus/DF0BA198.bitsy.txt"),
            "DF0BA198",
        );
    }
    #[test]
    fn test_df7f0379() {
        str(
            include_str!("test-resources/omnibus/DF7F0379.bitsy.txt"),
            "DF7F0379",
        );
    }
    #[test]
    fn test_e058a61f() {
        str(
            include_str!("test-resources/omnibus/E058A61F.bitsy.txt"),
            "E058A61F",
        );
    }
    #[test]
    fn test_e1c8834b() {
        str(
            include_str!("test-resources/omnibus/E1C8834B.bitsy.txt"),
            "E1C8834B",
        );
    }
    #[test]
    fn test_e1cd8743() {
        str(
            include_str!("test-resources/omnibus/E1CD8743.bitsy.txt"),
            "E1CD8743",
        );
    }
    #[test]
    fn test_e1f19987() {
        str(
            include_str!("test-resources/omnibus/E1F19987.bitsy.txt"),
            "E1F19987",
        );
    }
    #[test]
    fn test_e2417e83() {
        str(
            include_str!("test-resources/omnibus/E2417E83.bitsy.txt"),
            "E2417E83",
        );
    }
    #[test]
    fn test_e363f335() {
        str(
            include_str!("test-resources/omnibus/E363F335.bitsy.txt"),
            "E363F335",
        );
    }
    #[test]
    fn test_e46023b3() {
        str(
            include_str!("test-resources/omnibus/E46023B3.bitsy.txt"),
            "E46023B3",
        );
    }
    #[test]
    fn test_e48dccdb() {
        str(
            include_str!("test-resources/omnibus/E48DCCDB.bitsy.txt"),
            "E48DCCDB",
        );
    }
    #[test]
    fn test_e4a85876() {
        str(
            include_str!("test-resources/omnibus/E4A85876.bitsy.txt"),
            "E4A85876",
        );
    }
    #[test]
    fn test_e5d4285e() {
        str(
            include_str!("test-resources/omnibus/E5D4285E.bitsy.txt"),
            "E5D4285E",
        );
    }
    #[test]
    fn test_e6030df0() {
        str(
            include_str!("test-resources/omnibus/E6030DF0.bitsy.txt"),
            "E6030DF0",
        );
    }
    #[test]
    fn test_e69239ef() {
        str(
            include_str!("test-resources/omnibus/E69239EF.bitsy.txt"),
            "E69239EF",
        );
    }
    #[test]
    fn test_e971b9fd() {
        str(
            include_str!("test-resources/omnibus/E971B9FD.bitsy.txt"),
            "E971B9FD",
        );
    }
    #[test]
    fn test_e9bd845a() {
        str(
            include_str!("test-resources/omnibus/E9BD845A.bitsy.txt"),
            "E9BD845A",
        );
    }
    #[test]
    fn test_eadd856d() {
        str(
            include_str!("test-resources/omnibus/EADD856D.bitsy.txt"),
            "EADD856D",
        );
    }
    #[test]
    fn test_eb09383e() {
        str(
            include_str!("test-resources/omnibus/EB09383E.bitsy.txt"),
            "EB09383E",
        );
    }
    #[test]
    fn test_ec269334() {
        str(
            include_str!("test-resources/omnibus/EC269334.bitsy.txt"),
            "EC269334",
        );
    }
    #[test]
    fn test_ec8e4594() {
        str(
            include_str!("test-resources/omnibus/EC8E4594.bitsy.txt"),
            "EC8E4594",
        );
    }
    #[test]
    fn test_ecd56a4f() {
        str(
            include_str!("test-resources/omnibus/ECD56A4F.bitsy.txt"),
            "ECD56A4F",
        );
    }
    #[test]
    fn test_ed47e5ed() {
        str(
            include_str!("test-resources/omnibus/ED47E5ED.bitsy.txt"),
            "ED47E5ED",
        );
    }
    #[test]
    fn test_ed62fac9() {
        str(
            include_str!("test-resources/omnibus/ED62FAC9.bitsy.txt"),
            "ED62FAC9",
        );
    }
    #[test]
    fn test_edb03d05() {
        str(
            include_str!("test-resources/omnibus/EDB03D05.bitsy.txt"),
            "EDB03D05",
        );
    }
    #[test]
    fn test_ee642f55() {
        str(
            include_str!("test-resources/omnibus/EE642F55.bitsy.txt"),
            "EE642F55",
        );
    }
    #[test]
    fn test_ef26697e() {
        str(
            include_str!("test-resources/omnibus/EF26697E.bitsy.txt"),
            "EF26697E",
        );
    }
    #[test]
    fn test_ef763825() {
        str(
            include_str!("test-resources/omnibus/EF763825.bitsy.txt"),
            "EF763825",
        );
    }
    #[test]
    fn test_f02bd788() {
        str(
            include_str!("test-resources/omnibus/F02BD788.bitsy.txt"),
            "F02BD788",
        );
    }
    #[test]
    fn test_f22c1fa2() {
        str(
            include_str!("test-resources/omnibus/F22C1FA2.bitsy.txt"),
            "F22C1FA2",
        );
    }
    #[test]
    fn test_f31ca90b() {
        str(
            include_str!("test-resources/omnibus/F31CA90B.bitsy.txt"),
            "F31CA90B",
        );
    }
    #[test]
    fn test_f38c5522() {
        str(
            include_str!("test-resources/omnibus/F38C5522.bitsy.txt"),
            "F38C5522",
        );
    }
    #[test]
    fn test_f3af62e7() {
        str(
            include_str!("test-resources/omnibus/F3AF62E7.bitsy.txt"),
            "F3AF62E7",
        );
    }
    #[test]
    fn test_f3e61fc1() {
        str(
            include_str!("test-resources/omnibus/F3E61FC1.bitsy.txt"),
            "F3E61FC1",
        );
    }
    #[test]
    fn test_f52b510f() {
        str(
            include_str!("test-resources/omnibus/F52B510F.bitsy.txt"),
            "F52B510F",
        );
    }
    #[test]
    fn test_f60a4b5f() {
        str(
            include_str!("test-resources/omnibus/F60A4B5F.bitsy.txt"),
            "F60A4B5F",
        );
    }
    #[test]
    fn test_f64d8936() {
        str(
            include_str!("test-resources/omnibus/F64D8936.bitsy.txt"),
            "F64D8936",
        );
    }
    #[test]
    fn test_f673def5() {
        str(
            include_str!("test-resources/omnibus/F673DEF5.bitsy.txt"),
            "F673DEF5",
        );
    }
    #[test]
    fn test_f6f5e3fb() {
        str(
            include_str!("test-resources/omnibus/F6F5E3FB.bitsy.txt"),
            "F6F5E3FB",
        );
    }
    #[test]
    fn test_f712f2f3() {
        str(
            include_str!("test-resources/omnibus/F712F2F3.bitsy.txt"),
            "F712F2F3",
        );
    }
    #[test]
    fn test_f79d5368() {
        str(
            include_str!("test-resources/omnibus/F79D5368.bitsy.txt"),
            "F79D5368",
        );
    }
    #[test]
    fn test_f8d7930c() {
        str(
            include_str!("test-resources/omnibus/F8D7930C.bitsy.txt"),
            "F8D7930C",
        );
    }
    #[test]
    fn test_f9b6e61f() {
        str(
            include_str!("test-resources/omnibus/F9B6E61F.bitsy.txt"),
            "F9B6E61F",
        );
    }
    #[test]
    fn test_fa4eb7c6() {
        str(
            include_str!("test-resources/omnibus/FA4EB7C6.bitsy.txt"),
            "FA4EB7C6",
        );
    }
    #[test]
    fn test_fc8a7441() {
        str(
            include_str!("test-resources/omnibus/FC8A7441.bitsy.txt"),
            "FC8A7441",
        );
    }
    #[test]
    fn test_fcd95029() {
        str(
            include_str!("test-resources/omnibus/FCD95029.bitsy.txt"),
            "FCD95029",
        );
    }
    #[test]
    fn test_fe53ef82() {
        str(
            include_str!("test-resources/omnibus/FE53EF82.bitsy.txt"),
            "FE53EF82",
        );
    }
    #[test]
    fn test_fe6547de() {
        str(
            include_str!("test-resources/omnibus/FE6547DE.bitsy.txt"),
            "FE6547DE",
        );
    }
    #[test]
    fn test_ff3857ae() {
        str(
            include_str!("test-resources/omnibus/FF3857AE.bitsy.txt"),
            "FF3857AE",
        );
    }
    #[test]
    fn test_ff7bcf9c() {
        str(
            include_str!("test-resources/omnibus/FF7BCF9C.bitsy.txt"),
            "FF7BCF9C",
        );
    }
}
