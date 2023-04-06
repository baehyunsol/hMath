use crate::{BigInt, Ratio};

// https://en.wikipedia.org/wiki/Bailey%E2%80%93Borwein%E2%80%93Plouffe_formula

/*
These values are memoized.

fn unit(k: i32) -> Ratio {
    let k2_bi = BigInt::from_i64(k as i64 * k as i64);
    let k3_bi = k2_bi.mul_i32(k);
    let k4_bi = k3_bi.mul_i32(k);

    Ratio::from_denom_and_numer(
        k4_bi.mul_i32(512).add_bi(&k3_bi.mul_i32(1024)).add_bi(&k2_bi.mul_i32(712)).add_i32(k * 194).add_i32(15),
        k2_bi.mul_i32(120).add_i32(k * 151).add_i32(47)
    )
}

fn large_unit(k: i32) -> Ratio {
    unit(k * 2).add_rat(&unit(k * 2 + 1).div_i32(16))
}
*/

/// It returns the approximate value of PI.
/// It gets more and more accurate as `k` gets bigger.
/// For now, `k` should be less than 256.
pub fn pi_iter(k: usize) -> Ratio {
    let data = spigot_cache();
    let mut coeff = Ratio::one();
    let mut result = Ratio::zero();

    for i in 0..k {
        let curr = Ratio::from_denom_and_numer_raw(
            BigInt::from_i128(data[i].1),
            BigInt::from_i128(data[i].0),
        );

        // TODO: result.add_rat_mut();
        result = result.add_rat(&curr.mul_rat(&coeff));

        // TODO: coeff.div_i32_mut();
        coeff = coeff.div_i32(256);
    }

    result
}

// data[i] = (large_unit(i).numer, large_unit(i).denom)
fn spigot_cache() -> Vec<(i128, i128)> {
    vec![
        (0x19201, 0x7ff8),           // 3.141...
        (0x1148f, 0x18d314),         // 3.141592...
        (0x201a5eb, 0x9c795af8),     // 3.141592653...
        (0x763182c5, 0x4c4119c21e),  // 3.14159265358...
        (0xda61b29, 0xf2ad518d8),       // 3.14159265358979...
        (0xd34689bd7, 0x167c5c389c3c),  // 3.14159265358979323...
        (0x2d63e6fd7, 0x6dd7dbd28b8),   // 3.1415926535897932384...
        (0x6e624b747, 0x16823328a485),  // 3.141592653589793238462...
        (0x31b575290f, 0xd24d9b3c9248),     // 3.141592653589793238462643...
        (0x435f6ef7cd, 0x166b6a644ea0c),    // 3.14159265358979323846264338...
        (0xe50db9dce99, 0x5daca9cc7cdde8),  // 3.14159265358979323846264338327...
        (0xffe527eb, 0x7e2805eb716),        // 3.141592653589793238462643383279...
        (0xd78d6419a5d, 0x7e1284e4b557f8),  // 3.1415926535897932384626433832795028...
        (0x2e7e463d9e3, 0x1fd45875633e9c),  // 3.1415926535897932384626433832795028841...
        (0x686727645fb, 0x52b4790823bb98),  // 3.14159265358979323846264338327950288419...
        (0xddfdd8d5e6, 0xc979ae8be1d39),    // 3.14159265358979323846264338327950288419716...
        (0x2d20b5eaf79, 0x2e84ed6efedeb8),  // 3.14159265358979323846264338327950288419716939...
        (0x3da56b52fa9, 0x47a0e934d9e42c),
        (0x13f09c24734d, 0x19f06f03eb92848),
        (0x1cab71772b8f, 0x2980f6e5029f212),
        (0x55e2e7e38d97, 0x899d25946e512a8),
        (0x6d96174a0d7, 0xc1646354a7dd9c),
        (0xc092a6243deb, 0x174a3d4752f5f8d8),
        (0x48d5a1010d7f, 0x99ea3947666b38f),
        (0x2b7d4b1a303, 0x63fdd1a1d5b728),
        (0x43d0a142474f, 0xa91102215f4ace4),
        (0x2fcbb5a92e7b, 0x80cc0e4199bd8b8),
        (0x174c8bd50a5c3, 0x43aaa3de57ab141e),
        (0x15a2a05c7a4787, 0x438997932482807e8),
        (0xace26361b1b, 0x2429e540f0ecedc),
        (0xd5102ed09dd3, 0x2fabf4c07426fbd8),
        (0x326422b1cdbc, 0xc088fb1901e46b5),
        (0x52a7894c5671, 0x1505de435e1d3978),
        (0x97b99f4282dbd, 0x290625ae8fa28fe3c),
        (0x80a7ec42a677, 0x24e9ab56607e24d8),
        (0x9c1d98507067, 0x2f72a7a072fd8b1a),
        (0x2e59e09804fb, 0xee61bdee1af1588),
        (0x37df78a2ba4e75, 0x12f7102a8dee3384b4),
        (0xe88cdaa2dbd35, 0x533c02530089868a8),
        (0x2e7aa5e30543, 0x118493ae085921a5),
        (0xb121422fe9d31b, 0x463552689857717fe8),
        (0x15dd3cc8e3bf9, 0x91a441b862b24b9c),
        (0x4ecb214583d1d3, 0x226a33368974cd09f8),
        (0x3fb1aaca9175, 0x1d274a2763c16aba),
        (0x14c0dbac06d199, 0x9f19aa2ceee7e1198),
        (0x3b44739a72693b, 0x1db24c1fd36a4fa03c),
        (0x35dd8bbcadd1, 0x1c326487f441c6c8),
        (0x663d7980baf8a, 0x37dc58bc249bedd85),
        (0xadbcf7e395cc71, 0x62fd166a84451d3ff8),
        (0xbc78a76dc2b07, 0x6fe233ed0b5ce4ed4),
        (0x48435f50476ab, 0x2ca8bf880faf03378),
        (0x27d2385a77c69, 0x1999889097a8232ce),
        (0x11743bd71a6fe65, 0xba9c10b041354137f8),
        (0x1655845d99f879, 0xf804aac7a2788e2e4),
        (0x45df02c8d6f9e3, 0x32559cc4e36fd22b98),
        (0x921502ca46f179, 0x6d276f0ea59e1ddead),
        (0x71d46794a5d7, 0x5829df058ddca688),
        (0x32089003401dd, 0x28249a79a2fa2f12c),
        (0x641e8836e0254e9, 0x53291782c6942b8bde8),
        (0x1d8d3c6b6dceaf, 0x1965892609b00c9c06),
        (0x5d4c883885f18b, 0x52e96cf2d75e6b7a48),
        (0x4cda26acae149, 0x469556c8d8318eb34),
        (0x319957dd3ca5757, 0x2f0dc04434b885e09f8),
        (0x6d183857896dc8, 0x6ad99b5c8e23d3ae8f),
        (0x52262777e9bbe1, 0x53064bc2162c530af8),
        (0xaf286433285caf, 0xb694c6678668691a14),
        (0x2e085e7b50dc43, 0x31776a33435c9e68b8),
        (0x86d628aa65f609, 0x954ddf8695c4a2cc56),
        (0x7cf1f5b6c24fbf, 0x8e7fb3dfef5ff1cae8),
        (0xb880abc021ab7, 0xd8a369f60b7d06d3c),
        (0x6610ee1ac22e33f, 0x7b54dc57ec235dd79f8),
        (0x144e997adae93, 0x193ded23006bb0121),
        (0x78b12da61cf22a9, 0x9a43c2cc64ba23cfff8),
        (0xc486a7333f02ee3, 0x10232ba3fa5a68f496b4),
        (0x454676decb97f, 0x5d8493de890578bd8),
        (0x2678acbc0d9a43f, 0x3557e400155dae5681e),
        (0x13fb3577f63466f, 0x1c72742c7cb24b2a428),
        (0x59fe65bda67e38b, 0x8381d248582fe8ab03c),
        (0x1bc4113b28649e1, 0x29a1bef5092e1a39848),
        (0xb1728850349a, 0x110e8c7e7a7586bdb),
        (0xe1fd6d2034bd611, 0x16465aebcc3b029abff8),
        (0x196d52c9e0e0d, 0x291b11df68f2c8e5c),
        (0x3c6a1e47cd83a15, 0x6416462758eb8892388),
        (0x17732f060195ea3, 0x27ccb5c382f20513a0a),
        (0x3c712c897e62401, 0x6910504003f80d17198),
        (0x457d8d55a9fe2fb, 0x7bad83370f349e678ac),
        (0xde865bb2792b67, 0x19565384e33ae9a48b8),
        (0x2e8fc397b814a27, 0x56ce156670ea5730b8f),
        (0x12fce9d05a836d5, 0x24373c368b8008b4618),
        (0x2aa70bbdc6dabf5, 0x5334c62a528ce5e120c),
        (0x2971fcdd20956a9, 0x52ac788b4f513f8e0e8),
        (0xe9bad851596c21, 0x1dca16960cbb974526a),
        (0x18bf9bfa7f7c4f9, 0x3394a51564a8c197e18),
        (0x11528a10569812b3, 0x24e3fcc1bc1c8488f83c),
        (0x1627d40e676ed361, 0x3033871f95001be992c8),
        (0xdd4a562b8ea14, 0x1ebb5a9de8a5b584dd),
        (0x1acb288e4e14e09, 0x3ccaf31afee33101eb8),
        (0x16444f7880f12e5d, 0x3393da23559aa0cf9e3c),
        (0x2f57b81167a1222b, 0x6fede82e85286387f9f8),
        (0x2a0b78e2b6155, 0x6570419dd16de1e2e),
        (0xa03707cc6bb8787f, 0x18a5ed3465068a71327e8),
        (0x8cae2dfabf9ff, 0x1613b3622dd44d821c),
        (0x895f99906f7b469, 0x15fc704534f05e3e8848),
        (0x17e370c23bf40273, 0x3e5ff4fe4ac01c4554ad),
        (0xd7f16fcfe518a75, 0x23ed345a02a65ac91998),
        (0x23b93791fd5f73f1, 0x60ed5a1311787a11023c),
        (0x6e9910c37956dd, 0x131cfeb78717a41e388),
        (0x13fd7c3d8a72abe7, 0x3851f695f7390faae01e),
        (0x6807c0a5fe103f1, 0x12a96490fdf6ba8541d8),
        (0x3d3c9972c4730bb, 0xb3064e500bf11fb2dbc),
        (0x1f6f6856892422ed, 0x5d9747d4c820502b78a8),
        (0x7f68e7da255836, 0x1823f6b4bc930f9aae7),
        (0x1ca3bffa07cbfa19, 0x5863fcde497a5f1928b8),
        (0x7e8d84e1c5eb413, 0x18d9289c44ec6d8180e4),
        (0x1757b2820141fdff, 0x4aa212bd497f979eab98),
        (0x36a5f55dc06af49, 0xb1ccbb85d0d275e60ae),
        (0xbd6954a711a7eb, 0x272ffe41c8e38a59588),
        (0x4424e0949b1187ef, 0xe57902fc1157c777543c),
        (0x1ae2c9b2b4d52ea0d, 0x5c16b4d7d39fdfa620de8),
        (0x141b53c07d932d5, 0x460a00aed9cfd4f1081),
        (0x16a586ba6b18addf, 0x50371a1abb62701b5248),
        (0xf23d26f97f2cb, 0x3685af857101c6e4b4),
        (0xaef81ea3be9760e3, 0x2808abc4e291dc89469f8),
        (0x98b1784b47c127, 0x2382d663bde55a08b96),
        (0x7b65f1414633415, 0x1d2a72156fe3830536b8),
        (0x652438f6bd995503, 0x184ab13a861683dc7083c),
        (0x87c52ec84172a2f, 0x2121b680d81ed52338b8),
        (0xbea012ac4f02830, 0x2f4204798d3bcde3f9ab),
        (0x4dafe2b5942de82b, 0x1390385c4f2154b522aa8),
        (0x186a36ce474ce489, 0x63e97d1083b095c7960c),
        (0xfa748431da91c31, 0x410da38574de19883a68),
        (0x2ad2ceec1229d2d, 0xb4b62b5f66e940abece),
        (0x118158833d0779875, 0x4b0042fe964f98797b7f8),
        (0x927fef02fe3db517, 0x27d34eeeb46303ba6dc3c),
        (0xfac21784a7d3b, 0x4531e97f726c2327d8),
        (0x280a53d1d74ab725, 0xb36cfff34ea081f39d8f),
        (0x282bd9d64380eaf3, 0xb6af60140c9705255c28),
        (0x853bb1a6edb867d, 0x266d3b87dea44def9c4c),
        (0x16d44840db5d1b8b3, 0x6ae3f5c782ef21b4f49f8),
        (0x39348ec89242e089, 0x10fbc31d8f52e9e0a6812),
        (0x18e0e90a23297d0dd, 0x77e19f6c2c9c6755157f8),
        (0x12fcc5e2f319745, 0x5ccdb48c1971fa26a84),
        (0x906bc9d36a5a0ecd, 0x2cbe9200a9429b9c538a8),
        (0x387a1f3524eaf69a, 0x11be9ed7fe6a5517cfc8f),
        (0xd7539a0788d6dfb, 0x449a155e5bb3688dc3a8),
        (0x42f1568be8e337dd, 0x159fd8583ee34b831f49c),
        (0x6d1e6c95472bdf1, 0x23bc512e9dcdf782c2e8),
        (0xa3ef739e3db54dd, 0x366cabbfaa71bcfc5116),
        (0x21f7f04ec2ab1dff, 0xb6e4c22f62a39995d468),
        (0xb8d526d070a0c17, 0x3f0a6dd8f7fb9af41d9c),
        (0x25928c3edabe6144f, 0xcfcb5e2d5842dfa4999f8),
        (0x726119c099f3e1, 0x2810660401b5904c321),
        (0x28aae52286fa462b9, 0xe6f0db53308505355bff8),
        (0x152581eff79e58469, 0x79ab3492294604c03fa3c),
        (0x1a622474098ab3eb5, 0x99c99f07a45251956b2c8),
        (0x8b4bf53e293d897, 0x33684eab3a9894d86d26),
        (0x2c375622d03603f, 0x108766e5559c6a8cbf28),
        (0x4a05426bcb92b55f9, 0x1c069b2dafe67bf01548b4),
        (0x7527fd9311e5f051, 0x2cec8bf244e72a0dff848),
        (0x154a523dd4adc7f4, 0x844732422c7a52b29083),
        (0x4402a88f2902a165, 0x1abe0a5e67526003389d8),
        (0x125a8d19a8e23285, 0x74ea6aedf1e56f19879c),
        (0x880aa35828c0127d, 0x36d63412532b2ad22c848),
        (0x2e4fb850ea0e32127, 0x12e60084eb070084138e5a),
        (0x4451683a8d97fb9b, 0x1c38ae8f767e744bd5088),
        (0x4be83175c1eed9a1, 0x1fbd49cba0ad4f037eae4),
        (0x8436f40a8fd6bce5, 0x37f473de396b5acb93a28),
        (0xcfa63240da60b0b, 0x58f0ba0a243a18f27b0d),
        (0x49f927f961e7ef789, 0x201078869c8381c91a7ff8),
        (0x5d6f23fe34e65e9, 0x28fb975b9c98213508dc),
        (0x4f65a66928bf67053, 0x233d12a857d73f945809f8),
        (0xd282c5e1fe947cf, 0x5e880b03d17c4b861cce),
        (0x247dfc2be58427d11, 0x10943bb3fb4b270d7c6ed8),
        (0x121516533622db8d, 0x84f92b7b8ea469fb4764),
        (0x3a652f5954d1551f, 0x1b2659ee1228f5aa298b8),
        (0x2368a59188f8913ca, 0x10a6f6a845dcb2f2da59ad),
        (0x8eea59da2fdae5f, 0x43faf4c002bdb7fe0d88),
        (0x328809044a83fdd25, 0x184f4b58345be9c6bb263c),
        (0x980986dc84be6d91, 0x49f85d1776285b44c70e8),
        (0x1c3c221bcae78bd, 0xde441acd7a49b4ed2e6),
        (0x6fbf951ea86ea14e5, 0x3798809c40d57bb63137f8),
        (0x6ee46e66aee55cb5, 0x37c8a600614c07efc88d4),
        (0x27cb4b8e5a006daa5, 0x143d6124891eb8fdd948a8),
        (0x233eda9812a5c905, 0x121fa3c036ceff654c239),
        (0x4c77b83c51ebed5cf, 0x27c0356b57cb0246b68cc8),
        (0x41d29154d1b06e0b9, 0x2296ee28669b0c545dca3c),
        (0xf4c90feca8ffc1, 0x8206e1a32633e1ca7a8),
        (0x2318ecfb790d9880b, 0x12d813555e1a6a04d8ec1e),
        (0x90f09ec2c0a04114d, 0x4ea72572afcd0d732457f8),
        (0x15c307cc2d65f3f5, 0xbef60debef2eb30aa3a4),
        (0x422d019c22edc0713, 0x24ad9990c66c15f58828d8),
        (0x43fbaa55a737a48, 0x2613b81d37384695584d),
        (0xa46400c7f2faa0bc1, 0x5d0a41c998554756897ff8),
        (0x24569d6dca7d8d321, 0x14c7eb914279b10ed3aaac),
        (0x22fb07da8bc60d9cf, 0x14364b03fab2a6ef430b98),
        (0x2d1764f43ea4bd4b5, 0x1a52bbeb051d8e2fcc121e),
        (0xd397903894d1955, 0x7cca6a58c0a552509ef8),
        (0x1f4d073393c6ae67, 0x12a623644bb4a437f8cfc),
        (0xc59d18d461fa751bf, 0x76ef45ff419696e7d679f8),
        (0x3c286e86946e2bb, 0x249293d53e2f4fd83449),
        (0x1524904710444ad, 0xcfba54231e9867520d8),
        (0x453053aa63dec009, 0x2ae95f702c692b736db9c),
        (0x29c30f107cfac197d9, 0x1a28c753a1ede13e26e5de8),
        (0x395a27750697452df, 0x2447f2d1000635b7c1481e),
        (0x6c0058be1894c6bf, 0x44fef02ee813f206f4ba8),
        (0x79a21fa18ce69accb, 0x4e77a6b1518e2d1d8af03c),
        (0xa04a33105e37e9bf, 0x686a5f1eba83fff49d8b8),
        (0x49a9574614fcd42e, 0x30736aeb2a24e79f79539),
        (0x31c122fd3eaa1935b3, 0x210ae8d63a2582d53aa3fe8),
        (0x91a4c4026191463b, 0x61a7bf27923bbbe2c2a04),
        (0x118fe37c4205dd24db, 0xbe36ad6242fc80a20699f8),
        (0x1cd597d513acc9f, 0x13b4802e5e17af528712),
        (0x16e048b7fe174de59, 0xfc81b658fbd20438ba1d8),
        (0x98f0e154669760adf, 0x6a81c5dce333f8849c043c),
        (0x8fce5161a1bd27a5, 0x651643296bae6d154a1a8),
        (0x286f2b00e0e165377, 0x1cb0582d3c6ac68cd1bb8f),
        (0xd4da1f36fd9d30e1, 0x986dddb29312e7521deb8),
        (0x200da19d4f32ba471b, 0x172ac232d88d82bd2acceb4),
        (0x10bc321f48038d677, 0xc350d1d43e7e6dd840818),
        (0x120f4c7ae1df17a57, 0xd4b62a98b0b33c2839406),
        (0x45988183e2a7315cc7, 0x33b356119c11b82381207e8),
        (0x116f54649c1321fd, 0xd11da2ee3513c90c8284),
        (0x239eff0a2103553c5, 0x1af1d44ba067663586c0e8),
        (0x10c578b4c5f8cee24, 0xcccfb4ba5c42135672385),
        (0x1089948e85051f889, 0xcbc496ba3c5d6a2261eb8),
        (0x2553ecc8f99dd167, 0x1d00d2663f3cd85ac2f74),
        (0x34529f916f18cce29, 0x290410aebdc43154697a28),
        (0x254edba2743c8921f, 0x1d814571b433765f746e0a),
        (0x961542889eabf665, 0x77bd66e8017f22c2e7178),
        (0x8d8c3e0983000ccd1, 0x71ec0e9732c07f36625424),
        (0x1e44a954d9572fdd5f, 0x1892e5e14b4842dfcbeb9f8),
        (0x27c32bd4f19d5349, 0x20901ac9d3ec94f01c3e7),
        (0x48dd1fb2908251aaf, 0x3c30299218ad0ef0311248),
        (0x17c9d745897858933, 0x13d1d9ac523f338a50c034),
        (0x6b62f566bc81cccb7, 0x5a3d62e566dd91a4f11b98),
        (0x3b04006c5ea2e7483, 0x32044c896c4fae6e725256),
        (0x7887fd6edfb3c917, 0x6705d44d084cb976bbce8),
        (0x121bc343779a25171b, 0xf9bfd233ca9c1e44d0003c),
        (0x8922f812743ff02e9, 0x7735989661c9723e125388),
        (0x22d211005f9e0972, 0x1e85f70c443d4b65eaea5),
        (0x270c7a651f2f10a631, 0x228435e1e19e924430a3ff8),
        (0xccf7b0a19981808d, 0xb6b06526403c52dcc4f9c),
        (0x5dccaba448b3822ed, 0x544c7dce80b303bbeee848),
        (0xa8404294b19535031, 0x9875e0695e0328a712461e),
        (0x259f71f2bce4d1589, 0x225f7021a9dd56a67c9358),
        (0x75cebfaeeb5f7c065, 0x6c839d1d8a9fb0ac55dc14),
        (0x975cae61ca7a037, 0x8c8f9cdf5689270dde38),
        (0x11648fd5dce9299b61, 0x10487c9d48183d31047f6ad),
        (0x2f8426bdfdaed78799, 0x2cd8643514aed4f3ab13ff8),
        (0x239a1e71cd78a877, 0x21df40edc802ad0fdf084),
        (0x109e6c39a8647a9521, 0xff03e8df0adefde94278a8),
        (0xa0e29e0f8d4701f, 0x9b88c72712dddf0c8a86),
        (0x344ac6a8a10fa58c8d, 0x32f4c70f3f76e444bd657f8),
        (0xb79696ed2b3821aaf, 0xb451c7a95232fe445a8cac),
        (0x3a7b28faf5a227a79, 0x39e4f8ab921df8d6405e88),
        (0xa34a2e0c997c4140, 0xa2ed0a4b525a9462b220d),
    ]
}

#[cfg(test)]
mod tests {
    use super::pi_iter;

    #[test]
    fn pi_test() {
        assert_eq!(
            pi_iter(2).to_approx_string(8),
            "3.141592"
        );
        assert_eq!(
            pi_iter(3).to_approx_string(11),
            "3.141592653"
        );
        assert_eq!(
            pi_iter(4).to_approx_string(13),
            "3.14159265358"
        );
        assert_eq!(
            pi_iter(5).to_approx_string(16),
            "3.14159265358979"
        );
        assert_eq!(
            pi_iter(6).to_approx_string(19),
            "3.14159265358979323"
        );
    }

}