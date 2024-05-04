use super::test_display;
use crate::snes::cartridge::Mapper;

use hex_literal::hex;

macro_rules! test_instr {
    ($testname:ident, $hash:expr) => {
        #[allow(non_snake_case)]
        #[test]
        fn $testname() {
            test_display(
                include_bytes!(concat!(
                    "../../../siena_tests/SNES/CHIP/GSU/GSUTest/",
                    stringify!($testname),
                    "/GSU",
                    stringify!($testname),
                    ".sfc"
                )),
                &hex!($hash),
                120000,
                true,
                Mapper::SuperFX1,
            );
        }
    };
}

macro_rules! test {
    ($testname:ident, $filename:expr, $hash:expr) => {
        #[allow(non_snake_case)]
        #[test]
        fn $testname() {
            test_display(
                include_bytes!(concat!("../../../siena_tests/SNES/CHIP/GSU/", $filename)),
                &hex!($hash),
                120000,
                true,
                Mapper::SuperFX1,
            );
        }
    };
}

test_instr!(
    ADC,
    "57357b1d4eef094e5e7c5a577e5af357f98e4ce7548afa6208b87e4d71d6d981"
);
test_instr!(
    ADD,
    "9464afd9a6e5b5b7e82820e3ec9a19d6b332f778ae91fbfe6d130bb94b29ab80"
);
test_instr!(
    AND,
    "bfaa13ea4f1fa94aa865217c7081ba68b7509c5d68a88164a5a8ae78eb753e00"
);
test_instr!(
    ASR,
    "9b95c4d0b4748b9bad1d25291eb8c134cde5b7f68b2d53a68b1a84f015a04093"
);
test_instr!(
    BIC,
    "8b63a6305c0720e36a3c243ab0343b59b1d64001a1174dbd574b412ae6791a2b"
);
test_instr!(
    CACHEINJECT,
    "223972bb8f80aa9c7cb1a8ed0888bc6a5b547aa6000eba3e5d98124b9e20be77"
);
test_instr!(
    CMP,
    "b956679b98b69b2a352cbd8c4aee148209cf862bfcd9b19aa18fa55762b92290"
);
test_instr!(
    DEC,
    "97de2ae05bc5c9fd33050e5b404d582a4776f0c158fcddd0d5de9a2787e2259e"
);
test_instr!(
    DIV2,
    "e2c85bf15bda0ba0a09b9a41c8d8051c20523744482651332a369c1cb6d88bca"
);
test_instr!(
    FMULT,
    "aa11adbb7311853a47641e3496e794ce408b6b46c40c01cba0e604b3ce4fa096"
);
test_instr!(
    HIB,
    "4ad2a4b0594f86095a23225fa053e6c82f9049e768d694cdd92b776ba832eb23"
);
test_instr!(
    IBT,
    "6b91f2ae7fe5baa507cac7301a574bcaeb3c620b54b6cbbc984f856c9fe2847b"
);
test_instr!(
    INC,
    "9135c00c86992b5d7c7f1ba44af065b9eac038b593f566aeac5629d61ee2283b"
);
test_instr!(
    IWT,
    "89186701e498c5da2a3a1aa870d579488efd8acf2da279b91ca9804ee58f13ee"
);
test_instr!(
    LMULT,
    "6f05945702e61311bd2182f71ac77e950bf4698a9033fc77cb442f5acdd7afa6"
);
test_instr!(
    LOB,
    "b6d4db11dc08e752ccd83fcde4ec1ed1ace45bdae25e3554a02907da5eb33a39"
);
test_instr!(
    LSR,
    "5e727d5ab2c2f315302983ecd8b273df32352a8f6089c21190bec6fd5e077df9"
);
test_instr!(
    MERGE,
    "750a9a687534ca1b1da80d761a9e49231fdce05c41136f836c96f83597164aff"
);
test_instr!(
    MOVE,
    "75ca16694291e5a6f0ec8411b99e6a400f891d528f50255085811fcdca6f117c"
);
test_instr!(
    MOVES,
    "006835e6e83201ae6226ecc94d3f21527b4add86f31d69c34201e6713783566a"
);
test_instr!(
    MULT,
    "8b109131643148461871c83f87fb5f523195e77732a4029fe9f90e657a5acfc3"
);
test_instr!(
    NOT,
    "760b0db4e974bbcb6831c860543cde95fa036e9f94c00d45c2438ffa7b79db74"
);
test_instr!(
    OR,
    "88b0aa43c98688bb16ff331ac9d565b6bb4dcf911ede5b88cd10c439858f403c"
);
test_instr!(
    ROL,
    "73f6c69f1d085ca757b9c1f28b38b0001260293a91577ff9181b653aaa81c0a8"
);
test_instr!(
    ROR,
    "b7aa09e3486e8a86c8ec0734f81133e975e0dcc56baf0df0a3fe0efec0e49aac"
);
test_instr!(
    SBC,
    "e279f33482cdab3170a262b8adde4039097bb0faa39ac5c6b18eca021e178d9f"
);
test_instr!(
    SEX,
    "51c05a209bd65a691ad82ec34182ab3e19eb1e495c7661b560ab89933f9f3a7d"
);
test_instr!(
    SUB,
    "1b1e74709ea984e55969a449b07021a3f17d8fa51e4df0d29c726022c24be37c"
);
test_instr!(
    SWAP,
    "134b19db5c6ecc653277fc3015fa1f9c33f33203542ab6d29d386c2fbf049d10"
);
test_instr!(
    UMULT,
    "e5f4802df3d82fd441654aacacb983cafa9db799dd108705d85df632c61892af"
);
test_instr!(
    XOR,
    "91add3c8db1a2ee96b046b6ab075cf8f031fc5907082b078d8fd7d8127c1f0fa"
);

test!(
    GSU2BPP256x128FillPoly,
    "2BPP/FillPoly/256x128/GSU2BPP256x128FillPoly.sfc",
    "58438705947af2c993df4948b195336da924be41cf491f7417b7f1305ec0eae0"
);
test!(
    GSU2BPP256x160FillPoly,
    "2BPP/FillPoly/256x160/GSU2BPP256x160FillPoly.sfc",
    "780cc8b72fe5ec66cd223191143d8ad2b098e9e7728c63f19615788982469832"
);
test!(
    GSU2BPP256x192FillPoly,
    "2BPP/FillPoly/256x192/GSU2BPP256x192FillPoly.sfc",
    "aff488c325ee30437df54b35d10a0deb05d07da840f7c791c1bb837a026c3f4f"
);
test!(
    GSU2BPP256x128PlotLine,
    "2BPP/PlotLine/256x128/GSU2BPP256x128PlotLine.sfc",
    "0ef2b646771caea308a07e12305bad503d9df6ba055f0a8bce1002c77ce513aa"
);
test!(
    GSU2BPP256x160PlotLine,
    "2BPP/PlotLine/256x160/GSU2BPP256x160PlotLine.sfc",
    "a071cae220ced16963fe8b11a07fdc85b7c2dfce47068b7c8d2edf8c34bc7bf3"
);
test!(
    GSU2BPP256x192PlotLine,
    "2BPP/PlotLine/256x192/GSU2BPP256x192PlotLine.sfc",
    "b0e13fa29e55f8925da490c0c4d43af28302e31667c539257f07d63899b6c35b"
);
test!(
    GSU2BPP256x128PlotPixel,
    "2BPP/PlotPixel/256x128/GSU2BPP256x128PlotPixel.sfc",
    "b772874b1c7486b0b1f5a37a4d0bf831a4cce3bfde21771cc9ee95aa3a60e2d6"
);
test!(
    GSU2BPP256x160PlotPixel,
    "2BPP/PlotPixel/256x160/GSU2BPP256x160PlotPixel.sfc",
    "2690a92601c03760544da8d3231c9ebf7a04540bf8bcbfb3c58f1681698d1a1a"
);
test!(
    GSU2BPP256x192PlotPixel,
    "2BPP/PlotPixel/256x192/GSU2BPP256x192PlotPixel.sfc",
    "3c81ad682940a530e87ac29e06ef505f9012f2e1fca6a46e67afb42a81ec21c3"
);
test!(
    GSU4BPP256x128FillPoly,
    "4BPP/FillPoly/256x128/GSU4BPP256x128FillPoly.sfc",
    "58438705947af2c993df4948b195336da924be41cf491f7417b7f1305ec0eae0"
);
test!(
    GSU4BPP256x160FillPoly,
    "4BPP/FillPoly/256x160/GSU4BPP256x160FillPoly.sfc",
    "780cc8b72fe5ec66cd223191143d8ad2b098e9e7728c63f19615788982469832"
);
test!(
    GSU4BPP256x192FillPoly,
    "4BPP/FillPoly/256x192/GSU4BPP256x192FillPoly.sfc",
    "aff488c325ee30437df54b35d10a0deb05d07da840f7c791c1bb837a026c3f4f"
);
test!(
    GSU4BPP256x128PlotLine,
    "4BPP/PlotLine/256x128/GSU4BPP256x128PlotLine.sfc",
    "0ef2b646771caea308a07e12305bad503d9df6ba055f0a8bce1002c77ce513aa"
);
test!(
    GSU4BPP256x160PlotLine,
    "4BPP/PlotLine/256x160/GSU4BPP256x160PlotLine.sfc",
    "a071cae220ced16963fe8b11a07fdc85b7c2dfce47068b7c8d2edf8c34bc7bf3"
);
test!(
    GSU4BPP256x192PlotLine,
    "4BPP/PlotLine/256x192/GSU4BPP256x192PlotLine.sfc",
    "b0e13fa29e55f8925da490c0c4d43af28302e31667c539257f07d63899b6c35b"
);
test!(
    GSU4BPP256x128PlotPixel,
    "4BPP/PlotPixel/256x128/GSU4BPP256x128PlotPixel.sfc",
    "b772874b1c7486b0b1f5a37a4d0bf831a4cce3bfde21771cc9ee95aa3a60e2d6"
);
test!(
    GSU4BPP256x160PlotPixel,
    "4BPP/PlotPixel/256x160/GSU4BPP256x160PlotPixel.sfc",
    "2690a92601c03760544da8d3231c9ebf7a04540bf8bcbfb3c58f1681698d1a1a"
);
test!(
    GSU4BPP256x192PlotPixel,
    "4BPP/PlotPixel/256x192/GSU4BPP256x192PlotPixel.sfc",
    "3c81ad682940a530e87ac29e06ef505f9012f2e1fca6a46e67afb42a81ec21c3"
);
test!(
    GSU8BPP256x128FillPoly,
    "8BPP/FillPoly/256x128/GSU8BPP256x128FillPoly.sfc",
    "58438705947af2c993df4948b195336da924be41cf491f7417b7f1305ec0eae0"
);
test!(
    GSU8BPP256x160FillPoly,
    "8BPP/FillPoly/256x160/GSU8BPP256x160FillPoly.sfc",
    "780cc8b72fe5ec66cd223191143d8ad2b098e9e7728c63f19615788982469832"
);
test!(
    GSU8BPP256x192FillPoly,
    "8BPP/FillPoly/256x192/GSU8BPP256x192FillPoly.sfc",
    "aff488c325ee30437df54b35d10a0deb05d07da840f7c791c1bb837a026c3f4f"
);
test!(
    GSU8BPP256x128PlotLine,
    "8BPP/PlotLine/256x128/GSU8BPP256x128PlotLine.sfc",
    "0ef2b646771caea308a07e12305bad503d9df6ba055f0a8bce1002c77ce513aa"
);
test!(
    GSU8BPP256x160PlotLine,
    "8BPP/PlotLine/256x160/GSU8BPP256x160PlotLine.sfc",
    "a071cae220ced16963fe8b11a07fdc85b7c2dfce47068b7c8d2edf8c34bc7bf3"
);
test!(
    GSU8BPP256x192PlotLine,
    "8BPP/PlotLine/256x192/GSU8BPP256x192PlotLine.sfc",
    "b0e13fa29e55f8925da490c0c4d43af28302e31667c539257f07d63899b6c35b"
);
test!(
    GSU8BPP256x128PlotPixel,
    "8BPP/PlotPixel/256x128/GSU8BPP256x128PlotPixel.sfc",
    "b772874b1c7486b0b1f5a37a4d0bf831a4cce3bfde21771cc9ee95aa3a60e2d6"
);
test!(
    GSU8BPP256x160PlotPixel,
    "8BPP/PlotPixel/256x160/GSU8BPP256x160PlotPixel.sfc",
    "2690a92601c03760544da8d3231c9ebf7a04540bf8bcbfb3c58f1681698d1a1a"
);
test!(
    GSU8BPP256x192PlotPixel,
    "8BPP/PlotPixel/256x192/GSU8BPP256x192PlotPixel.sfc",
    "3c81ad682940a530e87ac29e06ef505f9012f2e1fca6a46e67afb42a81ec21c3"
);
