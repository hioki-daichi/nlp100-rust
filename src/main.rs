use std::collections::HashMap;

fn main() {
    t05();
    t04();
    t03();
    t02();
    t01();
    t00();
}

// 05. n-gram
// 与えられたシーケンス（文字列やリストなど）からn-gramを作る関数を作成せよ．この関数を用い，"I am an NLPer"という文から単語bi-gram，文字bi-gramを得よ．
fn t05() {
    let input = "I am an NLPer";

    // 文字bi-gram
    assert_eq!(
        input.chars().zip(input.chars().skip(1)).collect::<Vec<_>>(),
        [
            ('I', ' '),
            (' ', 'a'),
            ('a', 'm'),
            ('m', ' '),
            (' ', 'a'),
            ('a', 'n'),
            ('n', ' '),
            (' ', 'N'),
            ('N', 'L'),
            ('L', 'P'),
            ('P', 'e'),
            ('e', 'r')
        ]
    );

    // 単語bi-gram
    assert_eq!(
        input
            .split_ascii_whitespace()
            .zip(input.split_ascii_whitespace().skip(1))
            .collect::<Vec<_>>(),
        [("I", "am"), ("am", "an"), ("an", "NLPer")]
    );
}

// 04. 元素記号
// "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can."という文を単語に分解し，1, 5, 6, 7, 8, 9, 15, 16, 19番目の単語は先頭の1文字，それ以外の単語は先頭に2文字を取り出し，取り出した文字列から単語の位置（先頭から何番目の単語か）への連想配列（辞書型もしくはマップ型）を作成せよ．
fn t04() {
    let input = "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can.";

    let mut answer: Vec<HashMap<String, usize>> = vec![];

    let mut cs = input.chars();

    let mut index: usize = 1;

    while cs.as_str().len() > 0 {
        let s: String = cs.by_ref().take_while(|c| c.is_alphanumeric()).collect();

        if s.is_empty() {
            continue;
        }

        let num_take = match index {
            1 | 5 | 6 | 7 | 8 | 9 | 15 | 16 | 19 => 1,
            _ => 2,
        };

        let word: String = s.chars().take(num_take).collect();

        let mut map: HashMap<String, usize> = HashMap::new();
        map.insert(word, index);

        answer.push(map);

        index += 1;
    }

    assert_eq!(
        format!("{:?}", answer),
        r#"[{"H": 1}, {"He": 2}, {"Li": 3}, {"Be": 4}, {"B": 5}, {"C": 6}, {"N": 7}, {"O": 8}, {"F": 9}, {"Ne": 10}, {"Na": 11}, {"Mi": 12}, {"Al": 13}, {"Si": 14}, {"P": 15}, {"S": 16}, {"Cl": 17}, {"Ar": 18}, {"K": 19}, {"Ca": 20}]"#
    );
}

// 03. 円周率
// "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics."という文を単語に分解し，各単語の（アルファベットの）文字数を先頭から出現順に並べたリストを作成せよ．
fn t03() {
    let input = "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.";

    let mut answer: Vec<usize> = vec![];

    let mut cs = input.chars();

    while cs.as_str().len() > 0 {
        let n = cs.by_ref().take_while(|c| c.is_alphanumeric()).count();

        if n != 0 {
            answer.push(n);
        }
    }

    assert_eq!(answer, vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9]);
}

// 02. 「パトカー」＋「タクシー」＝「パタトクカシーー」
// 「パトカー」＋「タクシー」の文字を先頭から交互に連結して文字列「パタトクカシーー」を得よ．
fn t02() {
    let input1 = "パトカー";
    let input2 = "タクシー";

    let answer: String = input1
        .chars()
        .zip(input2.chars())
        .map(|(a, b)| format!("{}{}", a, b))
        .collect();

    assert_eq!(answer, "パタトクカシーー");
}

// 01. 「パタトクカシーー」
// 「パタトクカシーー」という文字列の1,3,5,7文字目を取り出して連結した文字列を得よ．
fn t01() {
    let input = "パタトクカシーー";
    let (xs, ys): (Vec<_>, Vec<_>) = input.chars().enumerate().partition(|(i, _)| i % 2 == 0);
    let answer = xs.iter().map(|(_, v)| v).collect::<String>();
    assert_eq!(answer, "パトカー");

    // オマケ
    assert_eq!(ys.iter().map(|(_, v)| v).collect::<String>(), "タクシー");
}

// 00. 文字列の逆順
// 文字列"stressed"の文字を逆に（末尾から先頭に向かって）並べた文字列を得よ．
fn t00() {
    let input = "stressed";
    let answer = input.chars().rev().collect::<String>();
    assert_eq!(answer, "desserts");
}
