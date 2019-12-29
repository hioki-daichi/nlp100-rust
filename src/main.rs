fn main() {
    t02();
    t01();
    t00();
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
