fn main() {
    let s = String::from("lrdrgrcggjrgrddfqfqrqppptrtssnqqqlzqqslspppsddfnnwgnnwhhjttnsnswnnbqbrrcqrqdqcdddlnddwwlmmlmtmwmggjgjmmmrppqhhswhwhmhchtchhsttvbtvvpspjjqrrgqrqmqvmvzmvzzrnnqlnlwlttjstjtjvjddrzzrnrmrbmbwwzppnqpqmmjnnvqnvvdzdpdvvmhvhhmshmsssvnnqppclcqlcqctqtfffvrfvrffptfftppncncwnntfntftdftfhfzzfwzzcddngdndccrttjmmcddrvrwwnntggghccjbjvbvjvlvjlvlgljlnjjtvjjfddmwwhnnqnrnccwllpmprmmspsvsrsslstsllpmllfzlflqltlssdbbdsdqsdqqgfgfjjtzjzmmsnsmnnzssgfgzgjgwwqfqtfthhhczzpwprwwqttqcqctcpcrrvwrvrggfvfqvvbqqhnnczcjjjfbfhbhjhssnwwfswszwwgbgzgczcbbvmvrrdgrddfwdffvsvtstztftfvfssgffnrnnzzqddltdtbdttnnfrnnfrfddpmpqqplqqbdbzbnnnrttjfttprtrllqhhwjhjghjjcrcqqznztzllldjjhqqspsfpfprrsfspsrrzbbjmjssnjnsjnjllbttgrtrnnqhhvjhvvrrplrprtrtprtrdtdfttwdttqtttwvtvtmtntnnfvfsfnndsnnnfqfmmcdmdppsmpmnnwhhlwwrvwrwvwlwnwhwjwvwbvwvnnqtnnttcgtccbggzgwzwczcpzcpzpzgzngnzzhfhdhzzzsggtssddgtdtndtdllpbpjpgjjsllvqvmmvjmvjvbbfzzbdzzghhzjzvvnffwsshqhqhjhmmtgmgpmgmpmzzzhtzzbzsbzsbzszvsvgsvspvsstwtgwgqgjggsfsmffhlhmhdmhmgmrmffjcjggqgsghgttjffprpmmscsggswssgzzsqzqjqwqjwqwswjjqpjpllwrwffvfbfpbffjsswgsshjsjqssqrrvcrrnvvdnnwndnffpdffjlffrjrlrqrmmlzlttpccgchggdjjhwwvqvvrprdrttlvvtlvtlvvvhqhwhmmpmrrjsrrzbzfzdzhdzzwgwwqllmmthmmvpprqprrzmrrfrmfrrmfmjmwwhtwhttnbbhbnnhbnnmhmgmjgmgfmmhpplwplwlswwjjbpbhpbbzmmgjmmggbzbssppjtttfmfwwcwmmbmrmbrblbmmszzvgvzgvgvdggjgsjgsjgghfgfngffsnfssdpdgpgrgnrggrlggdwgdgddfwddjdrrppnnqbnntmmrmlmrrgvvqjjwzzgnzzdtzzbccnbcbttbzzcqqrcqqpspbphbphhnpnccggczcgcpgcpctppqwpwccsgsttgtbttvftfcfbfvfppfpnpffmjmljmmrtrsrttnhhcvctclttrffhzhrzzdpzzwvwddcggpbptbpbzpzdpzddbjjdvvwmmfvfcfmfmmvrmvrmmcdmdqqhjqhqmmstmmdnnvssmbsmmnhmnmlmttlthtvhvvgbvgvbvqbvbvnbvvpwwhwsszpszppjljsllzhzcchmhdhphvvtpvvfssvmvcmmrjmmgfmfwmfmsspbbpnnsbsgbbzzllzzsqqswqqsbqqlvqlqzzbrzzqsqdqjqzqlqccqbqlblpprrthrttjzzfvzzpvpbppqjpjjtllmrmzzwnnrggpfpllwhhnmnpmmdppwnnwpnnzlzggvpvgpprdrzddlmddjndjjrljlsjshhqbbhhmhvhqhzqqlbqlqnqqmhmwmhwwgbwbpbhpbblnnshnshslllrcrncctnthtllmglmlmnmfnfllrbrmdqsncjfbjnghmvpctfzttqwsfptwhfhmdhzgbmrpqvsbbwdwdnqgclrgvdbqvtnzvlzmdjzgbhbtqjhrgqqjgwcvpqhhwsmmflgsjsvbnjcjcqwqqfcbvfrllbpphfglptjfczmnfnrmqdgrhbvrddwvhnbffsnzntvldrwmgqmqdbmmpdjhpdqfsbzsfbtdzlfjwtwfpfrhzcwwrcfpdjzjzqvcnhvdvlbdbjfrnmchdjprbfpzdtmjhdjtrwfrsghngbswhfzdfvhdmnqwnpspdjgsfcjjhrlnpncfdhzfzglgvrjlzdcncbfjhpnrvwnqbzhtdfcnpnpddsnjcbgdvzlsmpnlbftgjfsjsslljppjhtjfddmwbtrvmhvshnvhhfdsqhpmgzfrcqtpdmfcwglnbjzmtmzshzsrnbzlwlzdjzzsqfmjzbnprjdzswwcgjjwslhnfhbflvcpdfzwhzzjmfvpfhzbrcdvtlmbvrqvvtlpbdnshtsvlcnlwqzcnfhzndfjbhgwpvspdvhfptnfvwznscmgthspwpjfwlbbcrzgthrhnvscvfvwrwvtzpjqmwdbjjslvzlmqstzwqtsdsqqjpqthgnshpjncwgvppngvvfrjpztftgjbbmgdddrcgjggwpfrvfqbnzvwtscftbwfssmrgrbchjmvqcdgbdmmsswvplnvnjrnqzhttbzhlthzjslgtgrnqpghtbjnbftpdttvqhrljhnfqllrhfmzcnnwrljmchsvdbgvwmrpfzjsbngpffpgntrgflntbjnjwqbdhvhssvptdvvqvtrgqhhzrcfnnmbtzqrcghggcqhndggqzzgswflhqqrgmnggbrvpwqgtlptgmzdvgztgfqdzwnbrvvqhpnmcmptqljmqqmsslfcmgpnmnjmrsngrtbsffqbwlhsrwbzdcqvpbptpjphcjnwsmrjdbbrwftsnrgfhpjvsjcwpmpvfjtjvfnnwdjdttsjrqrgsznzqwjmsvscdtpptmdhqvwngtldtsnstjmwwrwwzdzvtndrrhgsgshzlpdrmlsgvplvbfrffvgvhmncbjdlqspbpdpcdsffrjzptltsznwwqbvnrwfdtrlbbvsmzjrhhvscqfwsqtmwnpjfgnjcttwphtqqhnvgjmzvczcrlmjjgwgnprcmmprltfrgrjlhvggdwvpnrqrtfwhpsfnjfvmvgzpmhrqmggldmsvztwcwzgblfbvbfvnshrdppzcvjwbjmsncwnbnjwbmwqjjwtjmwzpdvpwrfdtrqhnltgntvglgspvnbcsvnpppwmgphhsdqtpmrzmbdqlghsgjnnbhflzwghzzhdfsvjjcjfcssmvrmfqbgzcfwmhpvjqqrrhpsffczwgbjgwqlvzrvnhvzrqnfllqtrcjhmpdjfpqnlzhdnsctbfhsbpgwqdjdjhfbqzvzpsgmqwfjsffdjcqfmjgzqzvfsbhhvnwlfjfmdnphggdbnmpznlrzbnlbvhvplhjbzdmspnnlbctgzphghpngppdpdfbcdcpfntqlrwclsvnpljdbwcbhwzzdhbhsmslvvtjsmqmtpnhmmqnhfggnlpfthdhpwmrhzgfpwsffnrdqszcttrjsqzjqgspltfzzjnzwdfzvnmncwnphmjvcwlgrzcvpzcbndvhtjnfsgjtjdqfmgcgtgppsrcqwdjcqfddlhhnlfjrnsnssqblmnjjvmfbghtwwgcpgzfddvzsrsghqfrpvdtqmjfrzpvclmsnpmrqngdwvznlhdpbrzqtswfhnblnnbwjcwwbwfrgcdgrpphnslgnwbtfcgcmvvllwgvrcrdfcfwvwmdhhzmmnzmjgmgrgpdhngjgmhlqwtgzlngrbbfwfjrpwbqjnvdggrcfdgphfctvbmjnwfbpwrvbdbjrbhtnhfvhwvrptptsdrnzmnlwgbrwcswrccwdftgvjnvhffghdvhltjwhppfwfnmmtwclzzftzwvmhpmgvdsqzrfwqsmcgswnzjcnrvdgjlqdjrczphsvldlfzwdwmpncpvgqsvgpfpsfbgbmdhnfqbhdqwwwfdgqtmjlfbztsrwjrtqnrfpfqgplznpftrnjzhzcrnngqpwbrpbhlbfsgrpwfflrpbqdrqdplgcn");
    let i = find_tag(&s, 4).unwrap();
    println!("{}", i);


    let i = find_tag(&s, 14).unwrap();
    println!("{}", i);

}

fn find_tag(s: &String, len: usize) -> Option<i32> {
    let mut r = Vec::new();
    for (i, x) in s.chars().enumerate() {
        if r.len() < len {
            r.push(x);
        } else {
            r.remove(0);
            r.push(x);
        }

        if r.len() == len {
            // println!("{:?}", r);
            let mut flag = false;

            'outer: for (j, x) in r.iter().enumerate() {
                for i in j + 1..len {
                    if r[i] == *x {
                        flag = true;
                        break 'outer;
                    }
                }
            }

            if !flag {
                return Some(i as i32 + 1);
            }
        }
    }

    None
}
