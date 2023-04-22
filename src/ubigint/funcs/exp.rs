use crate::UBigInt;

impl UBigInt {

    /// It returns 2^n.
    pub fn exp2(n: u64) -> Self {
        UBigInt::from_u32(1 << ((n % 32) as u32)).shift_left((n / 32) as usize)
    }

    /// It's the inverse function of `log2_accurate`. It returns `truncate(2^(n/4294967296))`. It may have a small error when `n` is large.
    pub fn exp2_accurate(n: &UBigInt) -> Self {
        let trunc = n.shift_right(1);

        // 0 ~ 2^32-1
        let mut frac = n.sub_ubi(&trunc.shift_left(1)).to_u64().unwrap() as usize;

        // It uses linear approximation
        // f1 + (f2 - f1) * (frac % 2^23) / (2^23)
        let mut f1 = UBigInt::from_u64(EXP_TABLE[frac >> 23]);
        let mut f2 = UBigInt::from_u64(EXP_TABLE[(frac >> 23) + 1]);
        frac %= 1 << 23;

        for _ in 0..5 {
            let mid = f1.mul_ubi(&f2).sqrt();

            if frac > (1 << 22) {
                f1 = mid;
                frac -= 1 << 22;
                frac *= 2;
            }

            else {
                f2 = mid;
                frac *= 2;
            }

        }

        f1.add_ubi_mut(&f2.sub_ubi(&f1).mul_u32(frac as u32).div_u32(1 << 23));

        let result_pow_30 = UBigInt::exp2(trunc.to_u64().unwrap()).mul_ubi(&f1).shift_right(1);
        let result = result_pow_30.div_u32(1 << 30);

        if result_pow_30.rem_pow2(1 << 30).0[0] > (1 << 29) {
            result.add_u32(1)
        }

        else {
            result
        }

    }

}

#[cfg(test)]
mod tests {
    use crate::UBigInt;

    #[test]
    fn exp2_test() {

        for n in [0, 1, 2, 3, 4, 5, 17, 1728, 196, 159, 160, 161, 162, 9982, 230716] {
            assert_eq!(UBigInt::from_u32(2).pow_u32(n), UBigInt::exp2(n as u64));
        }

        for n in [1, 2, 3, 29, 1721, 4, 5, 6, 7, 8, 9, 9999, 1048576, 16777216, 230524, 16227766, 12345678, 31415926, 87654321, 987654321, 9876543210] {
            assert_eq!(UBigInt::from_u64(n), UBigInt::exp2_accurate(&UBigInt::from_u64(n).log2_accurate()));
        }

    }

}

const EXP_TABLE: [u64; 513] = [
    4611686018427387904,  // 2^(62 + 0/512)
    4617933561212708775,  // 2^(62 + 1/512)
    4624189567668517719,  // 2^(62 + 2/512)
    4630454049260717504,  // 2^(62 + 3/512)
    4636727017470743989,  // 2^(62 + 4/512)
    4643008483795587158,  // 2^(62 + 5/512)
    4649298459747812201,  // 2^(62 + 6/512)
    4655596956855580605,  // 2^(62 + 7/512)
    4661903986662671289,  // 2^(62 + 8/512)
    4668219560728501757,  // 2^(62 + 9/512)
    4674543690628149290,  // 2^(62 + 10/512)
    4680876387952372150,  // 2^(62 + 11/512)
    4687217664307630837,  // 2^(62 + 12/512)
    4693567531316109347,  // 2^(62 + 13/512)
    4699926000615736485,  // 2^(62 + 14/512)
    4706293083860207187,  // 2^(62 + 15/512)
    4712668792719003883,  // 2^(62 + 16/512)
    4719053138877417880,  // 2^(62 + 17/512)
    4725446134036570786,  // 2^(62 + 18/512)
    4731847789913435947,  // 2^(62 + 19/512)
    4738258118240859930,  // 2^(62 + 20/512)
    4744677130767584018,  // 2^(62 + 21/512)
    4751104839258265751,  // 2^(62 + 22/512)
    4757541255493500484,  // 2^(62 + 23/512)
    4763986391269842978,  // 2^(62 + 24/512)
    4770440258399829020,  // 2^(62 + 25/512)
    4776902868711997078,  // 2^(62 + 26/512)
    4783374234050909974,  // 2^(62 + 27/512)
    4789854366277176595,  // 2^(62 + 28/512)
    4796343277267473630,  // 2^(62 + 29/512)
    4802840978914567341,  // 2^(62 + 30/512)
    4809347483127335354,  // 2^(62 + 31/512)
    4815862801830788490,  // 2^(62 + 32/512)
    4822386946966092617,  // 2^(62 + 33/512)
    4828919930490590542,  // 2^(62 + 34/512)
    4835461764377823920,  // 2^(62 + 35/512)
    4842012460617555201,  // 2^(62 + 36/512)
    4848572031215789603,  // 2^(62 + 37/512)
    4855140488194797122,  // 2^(62 + 38/512)
    4861717843593134559,  // 2^(62 + 39/512)
    4868304109465667591,  // 2^(62 + 40/512)
    4874899297883592854,  // 2^(62 + 41/512)
    4881503420934460084,  // 2^(62 + 42/512)
    4888116490722194253,  // 2^(62 + 43/512)
    4894738519367117767,  // 2^(62 + 44/512)
    4901369519005972667,  // 2^(62 + 45/512)
    4908009501791942888,  // 2^(62 + 46/512)
    4914658479894676518,  // 2^(62 + 47/512)
    4921316465500308115,  // 2^(62 + 48/512)
    4927983470811481028,  // 2^(62 + 49/512)
    4934659508047369780,  // 2^(62 + 50/512)
    4941344589443702445,  // 2^(62 + 51/512)
    4948038727252783085,  // 2^(62 + 52/512)
    4954741933743514199,  // 2^(62 + 53/512)
    4961454221201419216,  // 2^(62 + 54/512)
    4968175601928665005,  // 2^(62 + 55/512)
    4974906088244084428,  // 2^(62 + 56/512)
    4981645692483198910,  // 2^(62 + 57/512)
    4988394426998241058,  // 2^(62 + 58/512)
    4995152304158177290,  // 2^(62 + 59/512)
    5001919336348730513,  // 2^(62 + 60/512)
    5008695535972402813,  // 2^(62 + 61/512)
    5015480915448498200,  // 2^(62 + 62/512)
    5022275487213145355,  // 2^(62 + 63/512)
    5029079263719320435,  // 2^(62 + 64/512)
    5035892257436869887,  // 2^(62 + 65/512)
    5042714480852533313,  // 2^(62 + 66/512)
    5049545946469966344,  // 2^(62 + 67/512)
    5056386666809763566,  // 2^(62 + 68/512)
    5063236654409481459,  // 2^(62 + 69/512)
    5070095921823661385,  // 2^(62 + 70/512)
    5076964481623852590,  // 2^(62 + 71/512)
    5083842346398635250,  // 2^(62 + 72/512)
    5090729528753643536,  // 2^(62 + 73/512)
    5097626041311588730,  // 2^(62 + 74/512)
    5104531896712282346,  // 2^(62 + 75/512)
    5111447107612659305,  // 2^(62 + 76/512)
    5118371686686801127,  // 2^(62 + 77/512)
    5125305646625959167,  // 2^(62 + 78/512)
    5132249000138577865,  // 2^(62 + 79/512)
    5139201759950318047,  // 2^(62 + 80/512)
    5146163938804080243,  // 2^(62 + 81/512)
    5153135549460028046,  // 2^(62 + 82/512)
    5160116604695611495,  // 2^(62 + 83/512)
    5167107117305590496,  // 2^(62 + 84/512)
    5174107100102058268,  // 2^(62 + 85/512)
    5181116565914464831,  // 2^(62 + 86/512)
    5188135527589640513,  // 2^(62 + 87/512)
    5195163997991819501,  // 2^(62 + 88/512)
    5202201990002663414,  // 2^(62 + 89/512)
    5209249516521284916,  // 2^(62 + 90/512)
    5216306590464271354,  // 2^(62 + 91/512)
    5223373224765708436,  // 2^(62 + 92/512)
    5230449432377203927,  // 2^(62 + 93/512)
    5237535226267911400,  // 2^(62 + 94/512)
    5244630619424553992,  // 2^(62 + 95/512)
    5251735624851448218,  // 2^(62 + 96/512)
    5258850255570527792,  // 2^(62 + 97/512)
    5265974524621367508,  // 2^(62 + 98/512)
    5273108445061207127,  // 2^(62 + 99/512)
    5280252029964975316,  // 2^(62 + 100/512)
    5287405292425313602,  // 2^(62 + 101/512)
    5294568245552600380,  // 2^(62 + 102/512)
    5301740902474974931,  // 2^(62 + 103/512)
    5308923276338361492,  // 2^(62 + 104/512)
    5316115380306493339,  // 2^(62 + 105/512)
    5323317227560936926,  // 2^(62 + 106/512)
    5330528831301116032,  // 2^(62 + 107/512)
    5337750204744335961,  // 2^(62 + 108/512)
    5344981361125807761,  // 2^(62 + 109/512)
    5352222313698672487,  // 2^(62 + 110/512)
    5359473075734025485,  // 2^(62 + 111/512)
    5366733660520940719,  // 2^(62 + 112/512)
    5374004081366495124,  // 2^(62 + 113/512)
    5381284351595793001,  // 2^(62 + 114/512)
    5388574484551990430,  // 2^(62 + 115/512)
    5395874493596319734,  // 2^(62 + 116/512)
    5403184392108113960,  // 2^(62 + 117/512)
    5410504193484831406,  // 2^(62 + 118/512)
    5417833911142080172,  // 2^(62 + 119/512)
    5425173558513642751,  // 2^(62 + 120/512)
    5432523149051500644,  // 2^(62 + 121/512)
    5439882696225859024,  // 2^(62 + 122/512)
    5447252213525171415,  // 2^(62 + 123/512)
    5454631714456164422,  // 2^(62 + 124/512)
    5462021212543862474,  // 2^(62 + 125/512)
    5469420721331612628,  // 2^(62 + 126/512)
    5476830254381109376,  // 2^(62 + 127/512)
    5484249825272419511,  // 2^(62 + 128/512)
    5491679447604007009,  // 2^(62 + 129/512)
    5499119134992757961,  // 2^(62 + 130/512)
    5506568901074005520,  // 2^(62 + 131/512)
    5514028759501554899,  // 2^(62 + 132/512)
    5521498723947708389,  // 2^(62 + 133/512)
    5528978808103290425,  // 2^(62 + 134/512)
    5536469025677672671,  // 2^(62 + 135/512)
    5543969390398799153,  // 2^(62 + 136/512)
    5551479916013211409,  // 2^(62 + 137/512)
    5559000616286073699,  // 2^(62 + 138/512)
    5566531505001198218,  // 2^(62 + 139/512)
    5574072595961070371,  // 2^(62 + 140/512)
    5581623902986874059,  // 2^(62 + 141/512)
    5589185439918517020,  // 2^(62 + 142/512)
    5596757220614656189,  // 2^(62 + 143/512)
    5604339258952723099,  // 2^(62 + 144/512)
    5611931568828949313,  // 2^(62 + 145/512)
    5619534164158391902,  // 2^(62 + 146/512)
    5627147058874958935,  // 2^(62 + 147/512)
    5634770266931435028,  // 2^(62 + 148/512)
    5642403802299506907,  // 2^(62 + 149/512)
    5650047678969789026,  // 2^(62 + 150/512)
    5657701910951849198,  // 2^(62 + 151/512)
    5665366512274234278,  // 2^(62 + 152/512)
    5673041496984495871,  // 2^(62 + 153/512)
    5680726879149216082,  // 2^(62 + 154/512)
    5688422672854033293,  // 2^(62 + 155/512)
    5696128892203667981,  // 2^(62 + 156/512)
    5703845551321948568,  // 2^(62 + 157/512)
    5711572664351837309,  // 2^(62 + 158/512)
    5719310245455456208,  // 2^(62 + 159/512)
    5727058308814112981,  // 2^(62 + 160/512)
    5734816868628327039,  // 2^(62 + 161/512)
    5742585939117855525,  // 2^(62 + 162/512)
    5750365534521719366,  // 2^(62 + 163/512)
    5758155669098229376,  // 2^(62 + 164/512)
    5765956357125012382,  // 2^(62 + 165/512)
    5773767612899037403,  // 2^(62 + 166/512)
    5781589450736641840,  // 2^(62 + 167/512)
    5789421884973557726,  // 2^(62 + 168/512)
    5797264929964937989,  // 2^(62 + 169/512)
    5805118600085382776,  // 2^(62 + 170/512)
    5812982909728965785,  // 2^(62 + 171/512)
    5820857873309260655,  // 2^(62 + 172/512)
    5828743505259367377,  // 2^(62 + 173/512)
    5836639820031938753,  // 2^(62 + 174/512)
    5844546832099206879,  // 2^(62 + 175/512)
    5852464555953009674,  // 2^(62 + 176/512)
    5860393006104817433,  // 2^(62 + 177/512)
    5868332197085759436,  // 2^(62 + 178/512)
    5876282143446650567,  // 2^(62 + 179/512)
    5884242859758017991,  // 2^(62 + 180/512)
    5892214360610127853,  // 2^(62 + 181/512)
    5900196660613012027,  // 2^(62 + 182/512)
    5908189774396494883,  // 2^(62 + 183/512)
    5916193716610220109,  // 2^(62 + 184/512)
    5924208501923677551,  // 2^(62 + 185/512)
    5932234145026230112,  // 2^(62 + 186/512)
    5940270660627140661,  // 2^(62 + 187/512)
    5948318063455599003,  // 2^(62 + 188/512)
    5956376368260748864,  // 2^(62 + 189/512)
    5964445589811714933,  // 2^(62 + 190/512)
    5972525742897629924,  // 2^(62 + 191/512)
    5980616842327661684,  // 2^(62 + 192/512)
    5988718902931040332,  // 2^(62 + 193/512)
    5996831939557085444,  // 2^(62 + 194/512)
    6004955967075233261,  // 2^(62 + 195/512)
    6013091000375063949,  // 2^(62 + 196/512)
    6021237054366328878,  // 2^(62 + 197/512)
    6029394143978977963,  // 2^(62 + 198/512)
    6037562284163187012,  // 2^(62 + 199/512)
    6045741489889385139,  // 2^(62 + 200/512)
    6053931776148282190,  // 2^(62 + 201/512)
    6062133157950896230,  // 2^(62 + 202/512)
    6070345650328581045,  // 2^(62 + 203/512)
    6078569268333053698,  // 2^(62 + 204/512)
    6086804027036422108,  // 2^(62 + 205/512)
    6095049941531212683,  // 2^(62 + 206/512)
    6103307026930397974,  // 2^(62 + 207/512)
    6111575298367424379,  // 2^(62 + 208/512)
    6119854770996239872,  // 2^(62 + 209/512)
    6128145459991321787,  // 2^(62 + 210/512)
    6136447380547704622,  // 2^(62 + 211/512)
    6144760547881007891,  // 2^(62 + 212/512)
    6153084977227464009,  // 2^(62 + 213/512)
    6161420683843946222,  // 2^(62 + 214/512)
    6169767683007996561,  // 2^(62 + 215/512)
    6178125990017853850,  // 2^(62 + 216/512)
    6186495620192481739,  // 2^(62 + 217/512)
    6194876588871596786,  // 2^(62 + 218/512)
    6203268911415696566,  // 2^(62 + 219/512)
    6211672603206087827,  // 2^(62 + 220/512)
    6220087679644914676,  // 2^(62 + 221/512)
    6228514156155186816,  // 2^(62 + 222/512)
    6236952048180807804,  // 2^(62 + 223/512)
    6245401371186603362,  // 2^(62 + 224/512)
    6253862140658349717,  // 2^(62 + 225/512)
    6262334372102801990,  // 2^(62 + 226/512)
    6270818081047722607,  // 2^(62 + 227/512)
    6279313283041909765,  // 2^(62 + 228/512)
    6287819993655225924,  // 2^(62 + 229/512)
    6296338228478626352,  // 2^(62 + 230/512)
    6304868003124187689,  // 2^(62 + 231/512)
    6313409333225136569,  // 2^(62 + 232/512)
    6321962234435878265,  // 2^(62 + 233/512)
    6330526722432025391,  // 2^(62 + 234/512)
    6339102812910426618,  // 2^(62 + 235/512)
    6347690521589195456,  // 2^(62 + 236/512)
    6356289864207739052,  // 2^(62 + 237/512)
    6364900856526787043,  // 2^(62 + 238/512)
    6373523514328420439,  // 2^(62 + 239/512)
    6382157853416100551,  // 2^(62 + 240/512)
    6390803889614697949,  // 2^(62 + 241/512)
    6399461638770521475,  // 2^(62 + 242/512)
    6408131116751347278,  // 2^(62 + 243/512)
    6416812339446447901,  // 2^(62 + 244/512)
    6425505322766621397,  // 2^(62 + 245/512)
    6434210082644220497,  // 2^(62 + 246/512)
    6442926635033181805,  // 2^(62 + 247/512)
    6451654995909055044,  // 2^(62 + 248/512)
    6460395181269032326,  // 2^(62 + 249/512)
    6469147207131977483,  // 2^(62 + 250/512)
    6477911089538455417,  // 2^(62 + 251/512)
    6486686844550761504,  // 2^(62 + 252/512)
    6495474488252951029,  // 2^(62 + 253/512)
    6504274036750868671,  // 2^(62 + 254/512)
    6513085506172178013,  // 2^(62 + 255/512)
    6521908912666391106,  // 2^(62 + 256/512)
    6530744272404898065,  // 2^(62 + 257/512)
    6539591601580996712,  // 2^(62 + 258/512)
    6548450916409922249,  // 2^(62 + 259/512)
    6557322233128876981,  // 2^(62 + 260/512)
    6566205567997060072,  // 2^(62 + 261/512)
    6575100937295697350,  // 2^(62 + 262/512)
    6584008357328071140,  // 2^(62 + 263/512)
    6592927844419550151,  // 2^(62 + 264/512)
    6601859414917619389,  // 2^(62 + 265/512)
    6610803085191910130,  // 2^(62 + 266/512)
    6619758871634229909,  // 2^(62 + 267/512)
    6628726790658592572,  // 2^(62 + 268/512)
    6637706858701248354,  // 2^(62 + 269/512)
    6646699092220714007,  // 2^(62 + 270/512)
    6655703507697802963,  // 2^(62 + 271/512)
    6664720121635655539,  // 2^(62 + 272/512)
    6673748950559769182,  // 2^(62 + 273/512)
    6682790011018028765,  // 2^(62 + 274/512)
    6691843319580736904,  // 2^(62 + 275/512)
    6700908892840644339,  // 2^(62 + 276/512)
    6709986747412980333,  // 2^(62 + 277/512)
    6719076899935483138,  // 2^(62 + 278/512)
    6728179367068430474,  // 2^(62 + 279/512)
    6737294165494670076,  // 2^(62 + 280/512)
    6746421311919650260,  // 2^(62 + 281/512)
    6755560823071450549,  // 2^(62 + 282/512)
    6764712715700812327,  // 2^(62 + 283/512)
    6773877006581169540,  // 2^(62 + 284/512)
    6783053712508679438,  // 2^(62 + 285/512)
    6792242850302253362,  // 2^(62 + 286/512)
    6801444436803587564,  // 2^(62 + 287/512)
    6810658488877194078,  // 2^(62 + 288/512)
    6819885023410431626,  // 2^(62 + 289/512)
    6829124057313536575,  // 2^(62 + 290/512)
    6838375607519653922,  // 2^(62 + 291/512)
    6847639690984868335,  // 2^(62 + 292/512)
    6856916324688235224,  // 2^(62 + 293/512)
    6866205525631811866,  // 2^(62 + 294/512)
    6875507310840688561,  // 2^(62 + 295/512)
    6884821697363019840,  // 2^(62 + 296/512)
    6894148702270055704,  // 2^(62 + 297/512)
    6903488342656172922,  // 2^(62 + 298/512)
    6912840635638906349,  // 2^(62 + 299/512)
    6922205598358980310,  // 2^(62 + 300/512)
    6931583247980340006,  // 2^(62 + 301/512)
    6940973601690182980,  // 2^(62 + 302/512)
    6950376676698990610,  // 2^(62 + 303/512)
    6959792490240559658,  // 2^(62 + 304/512)
    6969221059572033849,  // 2^(62 + 305/512)
    6978662401973935511,  // 2^(62 + 306/512)
    6988116534750197234,  // 2^(62 + 307/512)
    6997583475228193591,  // 2^(62 + 308/512)
    7007063240758772893,  // 2^(62 + 309/512)
    7016555848716288994,  // 2^(62 + 310/512)
    7026061316498633126,  // 2^(62 + 311/512)
    7035579661527265794,  // 2^(62 + 312/512)
    7045110901247248699,  // 2^(62 + 313/512)
    7054655053127276717,  // 2^(62 + 314/512)
    7064212134659709910,  // 2^(62 + 315/512)
    7073782163360605592,  // 2^(62 + 316/512)
    7083365156769750422,  // 2^(62 + 317/512)
    7092961132450692564,  // 2^(62 + 318/512)
    7102570107990773864,  // 2^(62 + 319/512)
    7112192101001162094,  // 2^(62 + 320/512)
    7121827129116883221,  // 2^(62 + 321/512)
    7131475209996853739,  // 2^(62 + 322/512)
    7141136361323913021,  // 2^(62 + 323/512)
    7150810600804855739,  // 2^(62 + 324/512)
    7160497946170464307,  // 2^(62 + 325/512)
    7170198415175541389,  // 2^(62 + 326/512)
    7179912025598942427,  // 2^(62 + 327/512)
    7189638795243608237,  // 2^(62 + 328/512)
    7199378741936597628,  // 2^(62 + 329/512)
    7209131883529120087,  // 2^(62 + 330/512)
    7218898237896568483,  // 2^(62 + 331/512)
    7228677822938551841,  // 2^(62 + 332/512)
    7238470656578928136,  // 2^(62 + 333/512)
    7248276756765837156,  // 2^(62 + 334/512)
    7258096141471733387,  // 2^(62 + 335/512)
    7267928828693418960,  // 2^(62 + 336/512)
    7277774836452076627,  // 2^(62 + 337/512)
    7287634182793302800,  // 2^(62 + 338/512)
    7297506885787140616,  // 2^(62 + 339/512)
    7307392963528113061,  // 2^(62 + 340/512)
    7317292434135256128,  // 2^(62 + 341/512)
    7327205315752152034,  // 2^(62 + 342/512)
    7337131626546962462,  // 2^(62 + 343/512)
    7347071384712461869,  // 2^(62 + 344/512)
    7357024608466070822,  // 2^(62 + 345/512)
    7366991316049889393,  // 2^(62 + 346/512)
    7376971525730730588,  // 2^(62 + 347/512)
    7386965255800153831,  // 2^(62 + 348/512)
    7396972524574498480,  // 2^(62 + 349/512)
    7406993350394917409,  // 2^(62 + 350/512)
    7417027751627410610,  // 2^(62 + 351/512)
    7427075746662858865,  // 2^(62 + 352/512)
    7437137353917057445,  // 2^(62 + 353/512)
    7447212591830749869,  // 2^(62 + 354/512)
    7457301478869661694,  // 2^(62 + 355/512)
    7467404033524534367,  // 2^(62 + 356/512)
    7477520274311159105,  // 2^(62 + 357/512)
    7487650219770410841,  // 2^(62 + 358/512)
    7497793888468282196,  // 2^(62 + 359/512)
    7507951298995917513,  // 2^(62 + 360/512)
    7518122469969646926,  // 2^(62 + 361/512)
    7528307420031020484,  // 2^(62 + 362/512)
    7538506167846842314,  // 2^(62 + 363/512)
    7548718732109204833,  // 2^(62 + 364/512)
    7558945131535523007,  // 2^(62 + 365/512)
    7569185384868568660,  // 2^(62 + 366/512)
    7579439510876504816,  // 2^(62 + 367/512)
    7589707528352920108,  // 2^(62 + 368/512)
    7599989456116863213,  // 2^(62 + 369/512)
    7610285313012877352,  // 2^(62 + 370/512)
    7620595117911034821,  // 2^(62 + 371/512)
    7630918889706971579,  // 2^(62 + 372/512)
    7641256647321921877,  // 2^(62 + 373/512)
    7651608409702752943,  // 2^(62 + 374/512)
    7661974195821999697,  // 2^(62 + 375/512)
    7672354024677899535,  // 2^(62 + 376/512)
    7682747915294427138,  // 2^(62 + 377/512)
    7693155886721329348,  // 2^(62 + 378/512)
    7703577958034160076,  // 2^(62 + 379/512)
    7714014148334315267,  // 2^(62 + 380/512)
    7724464476749067903,  // 2^(62 + 381/512)
    7734928962431603070,  // 2^(62 + 382/512)
    7745407624561053048,  // 2^(62 + 383/512)
    7755900482342532474,  // 2^(62 + 384/512)
    7766407555007173531,  // 2^(62 + 385/512)
    7776928861812161204,  // 2^(62 + 386/512)
    7787464422040768567,  // 2^(62 + 387/512)
    7798014255002392129,  // 2^(62 + 388/512)
    7808578380032587220,  // 2^(62 + 389/512)
    7819156816493103436,  // 2^(62 + 390/512)
    7829749583771920116,  // 2^(62 + 391/512)
    7840356701283281882,  // 2^(62 + 392/512)
    7850978188467734218,  // 2^(62 + 393/512)
    7861614064792159105,  // 2^(62 + 394/512)
    7872264349749810692,  // 2^(62 + 395/512)
    7882929062860351032,  // 2^(62 + 396/512)
    7893608223669885846,  // 2^(62 + 397/512)
    7904301851751000361,  // 2^(62 + 398/512)
    7915009966702795168,  // 2^(62 + 399/512)
    7925732588150922154,  // 2^(62 + 400/512)
    7936469735747620461,  // 2^(62 + 401/512)
    7947221429171752519,  // 2^(62 + 402/512)
    7957987688128840097,  // 2^(62 + 403/512)
    7968768532351100431,  // 2^(62 + 404/512)
    7979563981597482379,  // 2^(62 + 405/512)
    7990374055653702647,  // 2^(62 + 406/512)
    8001198774332282038,  // 2^(62 + 407/512)
    8012038157472581777,  // 2^(62 + 408/512)
    8022892224940839861,  // 2^(62 + 409/512)
    8033760996630207479,  // 2^(62 + 410/512)
    8044644492460785466,  // 2^(62 + 411/512)
    8055542732379660815,  // 2^(62 + 412/512)
    8066455736360943230,  // 2^(62 + 413/512)
    8077383524405801746,  // 2^(62 + 414/512)
    8088326116542501374,  // 2^(62 + 415/512)
    8099283532826439816,  // 2^(62 + 416/512)
    8110255793340184218,  // 2^(62 + 417/512)
    8121242918193507984,  // 2^(62 + 418/512)
    8132244927523427624,  // 2^(62 + 419/512)
    8143261841494239667,  // 2^(62 + 420/512)
    8154293680297557612,  // 2^(62 + 421/512)
    8165340464152348945,  // 2^(62 + 422/512)
    8176402213304972183,  // 2^(62 + 423/512)
    8187478948029213991,  // 2^(62 + 424/512)
    8198570688626326334,  // 2^(62 + 425/512)
    8209677455425063690,  // 2^(62 + 426/512)
    8220799268781720302,  // 2^(62 + 427/512)
    8231936149080167493,  // 2^(62 + 428/512)
    8243088116731891017,  // 2^(62 + 429/512)
    8254255192176028479,  // 2^(62 + 430/512)
    8265437395879406786,  // 2^(62 + 431/512)
    8276634748336579667,  // 2^(62 + 432/512)
    8287847270069865225,  // 2^(62 + 433/512)
    8299074981629383562,  // 2^(62 + 434/512)
    8310317903593094431,  // 2^(62 + 435/512)
    8321576056566834960,  // 2^(62 + 436/512)
    8332849461184357411,  // 2^(62 + 437/512)
    8344138138107367006,  // 2^(62 + 438/512)
    8355442108025559784,  // 2^(62 + 439/512)
    8366761391656660531,  // 2^(62 + 440/512)
    8378096009746460744,  // 2^(62 + 441/512)
    8389445983068856660,  // 2^(62 + 442/512)
    8400811332425887322,  // 2^(62 + 443/512)
    8412192078647772714,  // 2^(62 + 444/512)
    8423588242592951928,  // 2^(62 + 445/512)
    8434999845148121404,  // 2^(62 + 446/512)
    8446426907228273200,  // 2^(62 + 447/512)
    8457869449776733335,  // 2^(62 + 448/512)
    8469327493765200161,  // 2^(62 + 449/512)
    8480801060193782815,  // 2^(62 + 450/512)
    8492290170091039694,  // 2^(62 + 451/512)
    8503794844514017005,  // 2^(62 + 452/512)
    8515315104548287351,  // 2^(62 + 453/512)
    8526850971307988383,  // 2^(62 + 454/512)
    8538402465935861491,  // 2^(62 + 455/512)
    8549969609603290561,  // 2^(62 + 456/512)
    8561552423510340769,  // 2^(62 + 457/512)
    8573150928885797447,  // 2^(62 + 458/512)
    8584765146987204979,  // 2^(62 + 459/512)
    8596395099100905772,  // 2^(62 + 460/512)
    8608040806542079260,  // 2^(62 + 461/512)
    8619702290654780980,  // 2^(62 + 462/512)
    8631379572811981680,  // 2^(62 + 463/512)
    8643072674415606501,  // 2^(62 + 464/512)
    8654781616896574193,  // 2^(62 + 465/512)
    8666506421714836404,  // 2^(62 + 466/512)
    8678247110359417000,  // 2^(62 + 467/512)
    8690003704348451459,  // 2^(62 + 468/512)
    8701776225229226302,  // 2^(62 + 469/512)
    8713564694578218591,  // 2^(62 + 470/512)
    8725369134001135467,  // 2^(62 + 471/512)
    8737189565132953756,  // 2^(62 + 472/512)
    8749026009637959612,  // 2^(62 + 473/512)
    8760878489209788236,  // 2^(62 + 474/512)
    8772747025571463622,  // 2^(62 + 475/512)
    8784631640475438380,  // 2^(62 + 476/512)
    8796532355703633597,  // 2^(62 + 477/512)
    8808449193067478766,  // 2^(62 + 478/512)
    8820382174407951754,  // 2^(62 + 479/512)
    8832331321595618838,  // 2^(62 + 480/512)
    8844296656530674781,  // 2^(62 + 481/512)
    8856278201142982983,  // 2^(62 + 482/512)
    8868275977392115660,  // 2^(62 + 483/512)
    8880290007267394102,  // 2^(62 + 484/512)
    8892320312787928964,  // 2^(62 + 485/512)
    8904366916002660635,  // 2^(62 + 486/512)
    8916429838990399637,  // 2^(62 + 487/512)
    8928509103859867100,  // 2^(62 + 488/512)
    8940604732749735272,  // 2^(62 + 489/512)
    8952716747828668109,  // 2^(62 + 490/512)
    8964845171295361889,  // 2^(62 + 491/512)
    8976990025378585913,  // 2^(62 + 492/512)
    8989151332337223231,  // 2^(62 + 493/512)
    9001329114460311449,  // 2^(62 + 494/512)
    9013523394067083575,  // 2^(62 + 495/512)
    9025734193507008925,  // 2^(62 + 496/512)
    9037961535159834084,  // 2^(62 + 497/512)
    9050205441435623929,  // 2^(62 + 498/512)
    9062465934774802695,  // 2^(62 + 499/512)
    9074743037648195108,  // 2^(62 + 500/512)
    9087036772557067563,  // 2^(62 + 501/512)
    9099347162033169373,  // 2^(62 + 502/512)
    9111674228638774057,  // 2^(62 + 503/512)
    9124017994966720698,  // 2^(62 + 504/512)
    9136378483640455342,  // 2^(62 + 505/512)
    9148755717314072473,  // 2^(62 + 506/512)
    9161149718672356525,  // 2^(62 + 507/512)
    9173560510430823461,  // 2^(62 + 508/512)
    9185988115335762404,  // 2^(62 + 509/512)
    9198432556164277330,  // 2^(62 + 510/512)
    9210893855724328808,  // 2^(62 + 511/512)
    9223372036854775808,  // 2^(62 + 512/512)
];