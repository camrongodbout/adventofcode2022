fn unique_substring(s: &str) -> Option<usize> {

    for i in 0..s.len() - 13 {
        let sub = &s[i..i + 14];
        if is_unique(sub) {
            return Some(i + 14)
        }
    }
    None
}

fn is_unique(s: &str) -> bool {
    let mut seen = std::collections::HashSet::new();

    for c in s.chars() {
        if seen.contains(&c) {
            return false;
        }
        seen.insert(c);
    }
    true
}

fn main() {
    let s = "grvrnvrnnjljbjqjpqjjvhhzwwrbwwbblrltrrpbbbbqnnqbbbbsvbvmbvmbbrsrqrzrllwbbbqzqrqnqrnrjnnjccdggwqqhrrjcjmjmllgrlglhlclmlvlvsshwwsggmfmdfddgdfftrrczrcczhzppgdgrdggghmmdwwqgggslglfgfcgccmjcjwjrwjrjcrjjsgjjvddpwpgpbbgwbgwwhnhfftbffhpfphhfqfrqfrfnfpprvrsrhrfrllfhhrsrhssvfsvsnvsnsswtwtlthllrjjwddtggzczgcchwcwppfbbdvdrdzrdrvrwwsbsfbssqfsfjsjcscttlztllgjjlbbdsdtssvvvwlvlqqnhqqtdqtddjcdcjjpbphhgtgtqtzqqzhqqtgtvtmvtvrvqrvvfmfmppzzbwwnddzttfpfrrlddbppfqppnwnswwdhwdwjjqljqqthtnhnddgmgcmgcmgcmmfmfttrzzfdzztllmjlllgcgbbcqcvccpnndbdjbjmmzbztzptzpprpddptpprhhvlvmlmpmmljjnnjsjfjjvgjjvzzfgfzfbftbftttgstgstgtpggflfcfqqtctltgltldlzdlzzmmlddnvddzfddppmnpptzptpvttwstwswvwrvvbfbjjjbmjjdvdvrvdddrwrhrzrqqhghhrwhwhrrmppsgpsgszzdfdfwwmtwtvwvgvffmqqqtqntqnnjcncbnbwnnzggrdrqqjbqjjwjqqqwlqwlwzlljhhfsfsqsrqqhwqqwbbqbvvlflrrlglbbjhhjmhjjcmcjcgczcfcgcqqczcnnvjnnlddmpmcppgvgjgddvrrnsnmnqmqgmmnppwgwcgwgssbddgtdgdgmdgmgvvmjmvmjmvvsfssdgdghdggbfbqbdbjbsbmmrpmrprggbllwrwpwtppzvppzsssdnsdnnvnhvvvzvfzfqqnnmlnltldtdvdbdblddsmmlccmlmvlmvmmcsctctrtsrstsbsrshsddlmddmppgsscttnrtrqqcvcwwlnlznnnvcnvvtnvnbnmbmvmppjgjdjtddmpdmdvvmgvvdqdlqlhhzccsggjdjsdsttctjctjtfjttppdzpzzbjbwwmwbblslzslzszlzrrcbrrfggvcczjjtbbdnnggbwblwlbwlwqqfvfqfddrrfccvlllhmhhhrthrthrrnbnzbbpzplphprrrnbbghhnshnhbblqqqvwwffnmnmhhtccpqpvqvbvnvvfnfsnffdjdllwffcddgcgrgjrggchcpcddtbbdtdmtdmmhhtphtpppclcpcvcjvcjjfqfzqqphpnhnrnhhpdhhtfhhbbmqmfmsmvssgqqfssqgglnnqmnmbnmbbllrdrgdrdvrdvdsvvnddgtgddcdqdsqdqbqqlhhwdhdgdcgcdchchrchhpvvpgvgrrfggwfwgmpddbhfngtrwswfszgsggnpsntjpslrpjqsffzrlnbnzdtqpqtjzwlhhgrsrbvnccnsjmzcbqgcbtbqlzhnpnhhrrvqwjwzzvrlcrmjhcscrqhpqmfzbnvcwwqhcjjlnggmpbwztzfswmsbjshnsgfmdlzvzczhrdwgwbghszpnbfpctrshbfhspsczcqcrrqcpwwpfzhjqtpqgjbztrpzrlgfdjbmlwdvlvnfmdzbwsbbhlbszvwcpztlchjrqbmsftltmqpfgdpmdgjvwqqtjsqlfqrwmsnlqgsbqfwsdnfvzthmbplvszfcmlptlcjpnfpjsphsmmjplwjqphgvzbtbjtpttqhlwtgnrjvmvsfsztmsqszzlhqqhfslsvhzgtsssfctzgsqbgdzlpwbsmpcnjqshhhcwqdsdzdhnjfqzqnqdlrpddcgrgldgqbjmdtwgppdczzrjvmcfqjbpjzbtjmgdphlbwnsnpfdqlhwvvmpwzsrztnwvtlbphljmjwsgbphgmwhdmfhpvsmvsjccjhfvqtvfmmlnggncltvtrgmbtfqsvfnlvcmjnjwzcrpjnsgntvhjbtdlptshbhhchqmsprhqzdnfpjqccdfvnzjtlbsmmwvzlwlvmsbrnhqctvtvbfhntdctjnrbcrrlmsnwbbjbcbbgrrhfqwzwwfgvsvgbwnttghtgpspzwzfhffsqjvwwttntnvlwftsfvtttgnprzrzsghvjrdtsfdvzswhmrfcdqsgvrlhzbnvbmjlqrftnnbtwqtvlvwznfbslhdqjbntdgpprfqchjvgvzjssdztjlzwfljjmfvzrbbtczggzqwrnqqgzzcbqjcpfqfrbwtdjrrvbszsjdjcpdfjscsvnltcgwvqsgnhbfgnfnddnpmbzbptrmvqzpvbdpfdvtlmgnnjwflgdbfnmvsdnmlvgcpwflwvdbtbfwtfpsmqsplnzwlwgvbjrhghwrnrswsggbqpdjcjrgbgnsqdvwzzwftvjqgjzzcdvpbbjzpphmbcqmrjvgqwfgrsnqvhwflmhgrlvbpwdcsrlqwfrwppqbrdhwqtvczpclpsbsjcptgblbbsqmbhjjgzwvlcnhnzcttmpjsgchmppgphqlzlcsqcgbbjgtjjvmttdztfdptzgvmpnqrcmpmcdlpnbztllvqbggqbqhlqvdwsrwzsjwfrqvcbvsgfdptmrzpvdfblmhlzrvpmsljlqqzrhlnmwncpfhvqlsbtrjbfcrnfvjvddrhdbbczjdsrdvzlbqrccssdzcpmdsqbprjppfzdwfdswptgzcmjqfhcwsqfqhvrslffqfbcvhdzljzrmtwmfdwzdhhjcmbjtvjhzzwfqhrcslztdbnlwmmhbbbgdscjcdzftnchqfnflnsdqjscfrqpnfbftpzvtmrwncqfqqflschpfnjsjlqcjdjgtwpqhgcnjdmnnvmmpwdspmnrgqrptqwcvbtdwpqlbtwpqgwgfrzlrhtvrvzhmhmwhfdsrhpcczqfltsgtgrfwcvlcvtlhqqwnrqgzpnzbfmzbdwqwbsfvbshrgzqdbgvrhzhzlbqsfzttmsnmrqmwgtzbvdqdrbgcpclzjrhdbjtpcdbbznjgtbwbqrnpvffdmwtrbhhstcmnjcwbbnmpbvmjprtzgcptmtrffwhvfgdljnrbbrblbfbgdwtjrtgqgrpvpgjqrjzczvvlspgdbzftqgqvgdqlglbgvgjdcztznszcwfqhmwbrbjcfstzdcmdsssqfhtzpdgmzjscvbdzgbhhgdqgvfwrzmhdrhlsvlzjjzbzdljcbhncppwrtptjgszlqsrqpzqcsgvdvzmgvwgsncnbffttslphcstqvfwbwzbflmshcbnhpljgqwmmwwzlgpbcqnrtqlwcjcrclfdrnnmvtbfdztdfvtqrsgdptfcfpzpsldhzmrngggfvdqggtlfqqwsldprcffsstnnpmsbbvghdbpprqbssnprdbqclzqtgsrczwcvqwrrfmmfwsndvtvqljwwglrgbphdvvwgctbbmtrbpzqtspgrlhmnhjcdwhwvssgspzjbcfjttjqbdpdmptfzzjcfqljpqddfssmffqprvbptfvdshsdmfmdtmlbnmbmjjjsgmlmwmgcwhbrbgchrstptvdlqgddfzddlzhwjmsvvcjwvqtzjtsctfmzchlbrvlgdzbvdlbfpvhptpltrdmcgjghcpwvwqqnrzdtnmgdncplhdpsgpnbprbgshffwwsdhpgqsbmwdtpnhhltlcqfrjtswcchzvlhdgrmjwhgwppdjqlgmdhwbllqvzrchgclmqdlghjsvmwlflmhhmdzbfjhjnvwphnjbclmdpgflqgtfsmsjslntfcmtbphnrgpdcqtjzjttdtgjmvhzsrfnrjqssvwpcslpfstbpfsrsntmftmdgsqrrsnddqfmchrhtlhmqndvvllnvltdzfphjqnvmcdsgfpcmjftgdpntjzplqljhtthvnbzbzwvfnqsjvnfwhmtbsspjslgfjvdgfjpwrsgqwntntjcqtdgnhnsfwhhqfwbwhdrftj";

    let substring = unique_substring(s);

    print!("{:?}", substring);
}
