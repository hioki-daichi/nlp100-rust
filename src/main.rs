fn main() {
    t03();
    t02();
    t01();
    t00();
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
