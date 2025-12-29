fn main() {
    println!("=== Chapter 1: Rustの基礎 ===");

    // 1. 変数と可変性 (Variables and Mutability)
    practice_variables();

    // 2. 基本的なデータ型 (Basic Data Types)
    practice_types();

    // 3. 関数 (Functions)
    practice_functions();

    // 4. 制御フロー (Control Flow)
    practice_control_flow();

    println!("\nすべて完了しました！お疲れ様でした。");
}

fn practice_variables() {
    println!("\n--- 1. 変数と可変性 ---");

    // 課題 1-1: 不変変数 (Immutable variables)
    // 以下のコードはコンパイルエラーになります。
    // コメントアウトを外し、変数 `x` に `mut` キーワードを追加して修正してください。
    
    
    let mut x = 5;
    println!("x の値: {}", x);
    x = 10;
    println!("x の変更後の値: {}", x);
    

    // 課題 1-2: 定数 (Constants)
    // 定数 `MAX_POINTS` を定義してください。値は 100,000 とし、型注釈を行ってください。
    // ヒント: const キーワードを使用します。
    
    // TODO: ここに定数を定義する
    const MAX_POINTS: i32 = 100000;
    println!("最大ポイント: {}", MAX_POINTS);

    // 課題 1-3: シャドーイング (Shadowing)
    // 以下のコードの実行結果を予想し、正しく動作するように変数 `y` を定義してください。
    let y = 5;
    let y = y + 1;
    {
        let y = &y * 2;
        println!("内側のスコープでの y の値: {}", y); // ここでは 12 になるはず
    }
    println!("外側のスコープでの y の値: {}", y); // ここでは 6 になるはず
}

fn practice_types() {
    println!("\n--- 2. 基本的なデータ型 ---");

    // 課題 2-1: 数値演算
    // 足し算、引き算、掛け算、割り算、余りの計算を行ってください。
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    println!("計算結果: {}, {}, {}, {}, {}", sum, difference, product, quotient, remainder);

    // 課題 2-2: タプル (Tuples)
    // タプル `tup` から要素を取り出し（デストラクチャリング）、x, y, z という変数に束縛してください。
    let tup = (500, 6.4, 1);
    
    // TODO: ここでデストラクチャリングを行う
    // let (x, y, z) = ...
    let (x, y, z) = tup;
    
    // println!("y の値: {}", y); // 6.4 と表示されるようにする
    println!("yの値: {}", y);

    // 課題 2-3: 配列 (Arrays)
    // 配列 `a` の最初の要素と最後の要素にアクセスして表示してください。
    let a = [1, 2, 3, 4, 5];
    
    // TODO: 配列の要素にアクセスする
    // println!("最初の要素: {}", ...);
    println!("最初の要素: {}", a[0]);
    // println!("最後の要素: {}", ...);
    println!("最後の要素: {}", a[a.len() - 1]);
}

fn practice_functions() {
    println!("\n--- 3. 関数 ---");

    // 課題 3-1: 引数のある関数
    // 下に定義されている `print_measurement` 関数を呼び出してください。
    // 引数として 5 と 'h' を渡してください。

    // TODO: print_measurement を呼び出す
    print_measurement(5, 'h');

    // 課題 3-2: 戻り値のある関数
    // 下に定義されている `plus_one` 関数を完成させ、ここで呼び出して結果を表示してください。
    
    let five = 5;
    let six = plus_one(five);
    println!("5 + 1 = {}", six);
}

// ヘルパー関数: 課題 3-1 用
fn print_measurement(value: i32, unit_label: char) {
    println!("測定値: {}{}", value, unit_label);
}

// ヘルパー関数: 課題 3-2 用
fn plus_one(x: i32) -> i32 {
    // TODO: x に 1 を足した値を返すように修正する（セミコロンをつけない、または return を使う）
    return x+1;
}

fn practice_control_flow() {
    println!("\n--- 4. 制御フロー ---");

    // 課題 4-1: if 式
    let number = 3;
    
    // number が 5 未満なら "condition was true"、そうでなければ "condition was false" と表示する
    // if 文を作成してください。
    
    // TODO: if 文を書く
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // 課題 4-2: if を使った変数束縛
    let condition = true;
    // if 式を使って、condition が true なら 5、false なら 6 を `number` に代入してください。
    // let number = ...
    let number = if condition == true { 5 } else { 6 };
    
    println!("number の値: {}", number);

    // 課題 4-3: loop ループ
    // loop を使ってカウンタをインクリメントし、10になったら break で値を返してループを終了させてください。
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            // TODO: break で counter の値を返す
            break counter;
        }
    };
    println!("ループの結果: {}", result);
}
