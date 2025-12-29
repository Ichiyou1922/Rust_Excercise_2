fn main() {
    println!("=== Chapter 2: 所有権、参照と借用、スライス ===");

    // 1. 所有権 (Ownership)
    practice_ownership();

    // 2. 参照と借用 (References and Borrowing)
    practice_borrowing();

    // 3. スライス (Slices)
    practice_slices();

    println!("\nChapter 2 完了！");
}

fn practice_ownership() {
    println!("\n--- 1. 所有権 ---");

    // 課題 1-1: ムーブ (Move)
    // String型は所有権の移動（ムーブ）が起こります。
    // 以下のコードはコンパイルエラーになります。s1 が s2 にムーブされた後も s1 を使おうとしているためです。
    // s1 の値を s2 に「クローン（複製）」して、エラーを解消してください。
    
    let s1 = String::from("hello");
    // TODO: ここを修正する (.clone() を使う)
    let s2 = s1; 

    // println!("s1 = {}, s2 = {}", s1, s2);


    // 課題 1-2: コピー (Copy)
    // 整数型などのシンプルな型は Stack-Only Data としてコピーされます。
    // 以下のコードは修正なしで動きます。コメントアウトを外して確認してください。
    // なぜエラーにならないのか考えてみましょう。
    
    /*
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
    */

    // 課題 1-3: 所有権と関数
    // `takes_ownership` 関数に `s` を渡すと、所有権が移動します。
    // その後 `s` を使おうとするとエラーになります。
    // `takes_ownership` の呼び出しをコメントアウトするか、
    // あるいは `s.clone()` を渡すようにして、最後の println! が動くようにしてください。

    let s = String::from("ownership");
    takes_ownership(s); // ここでムーブする

    // println!("main関数で s を使う: {}", s); // コンパイルエラーになる箇所
}

fn takes_ownership(some_string: String) {
    println!("takes_ownership: {}", some_string);
} // ここで some_string がスコープを抜け、drop される

fn practice_borrowing() {
    println!("\n--- 2. 参照と借用 ---");

    // 課題 2-1: 不変参照 (Immutable Reference)
    // `calculate_length` 関数を完成させ、所有権を渡さずに文字列の長さを計算してください。
    // ヒント: 引数の型を `&String` にします。

    let s1 = String::from("hello");
    
    // TODO: calculate_length の定義を修正し、呼び出し側も参照を渡すように修正する
    // let len = calculate_length(s1);
    
    // println!("'{}' の長さは {} です。", s1, len);


    // 課題 2-2: 可変参照 (Mutable Reference)
    // `change` 関数を使って、文字列に ", world" を追加してください。
    // 変数を可変にし、可変参照を渡す必要があります。

    // TODO: s2 を可変(mut)にする
    let s2 = String::from("hello");

    // TODO: change関数に可変参照を渡す (&mut s2)
    // change( ... );
    
    // println!("変更後の文字列: {}", s2);


    // 課題 2-3: 参照の規則
    // 同一スコープ内で、可変参照は1つしか持てません。
    // また、不変参照がある間は可変参照を持てません。
    // 以下のコードはエラーになります。
    // r1 と r2 の使用が終わった後に r3 を作成するようにコードブロック {} を使うか、
    // 単に r1, r2 の使用箇所（println!）を r3 の作成前に移動させてください（NLL: Non-Lexical Lifetimes の効果）。
    
    /*
    let mut s = String::from("hello");

    let r1 = &s; // 不変参照
    let r2 = &s; // 不変参照
    
    let r3 = &mut s; // 可変参照（ここでエラー！）

    println!("{}, {}, and {}", r1, r2, r3);
    */
}

// 課題 2-1 用
// TODO: 引数の型を参照 `&String` に変える
fn calculate_length(s: String) -> usize {
    s.len()
}

// 課題 2-2 用
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn practice_slices() {
    println!("\n--- 3. スライス ---");

    // 課題 3-1: 文字列スライス
    // 文字列 "Hello world" の中から "Hello" と "world" をスライスとして取り出してください。
    let s = String::from("Hello world");

    // TODO: 範囲を指定してスライスを作成する [start..end]
    // let hello = ...
    // let world = ...

    // println!("hello = {}, world = {}", hello, world);


    // 課題 3-2: 配列スライス
    // 配列の一部を参照するスライスを作成してください。
    let a = [1, 2, 3, 4, 5];

    // TODO: [2, 3] という部分配列（スライス）を取得する
    // let slice = ...

    // assert_eq!(slice, &[2, 3]);
    // println!("配列スライス: {:?}", slice);
}