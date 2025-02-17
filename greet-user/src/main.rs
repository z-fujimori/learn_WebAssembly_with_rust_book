use clap::Parser;
use wasmtime::{Engine, Store};
use wasmtime::component::{bindgen, Component, Linker, TypedFunc};

// ファイルの先頭でマクロを実行
// {} の間にコード生成のための設定を記述
bindgen!({
    world: "greetable-provider",  // コード生成を行うワールドの名前
    path: "../greet/wit",  // WITファイルが存在するフォルダーへのパス
});

#[derive(Parser, Debug)]
struct Args {
    wasm_file: String
}

fn start(args: Args) -> anyhow::Result<()> {
    // wasmの処理をするEngineオブジェクトを標準の設定で作成
    let engine = Engine::default();
    
    let component = Component::from_file(&engine, &args.wasm_file)?;  // ファイルをロードしてComponentオブジェクトを作成
    let linker = Linker::new(&engine);  // Engineオブジェクトへの参照を与えて、Linkerオブジェクトを作成
    // Storeオブジェクトを作成      // 第二引数はStoreオブジェクトの初期値
    // Componentオブジェクトを作成するごとにStoreオブジェクトの状態が変化するのでmut
    let mut store = Store::new(&engine, ());
    
    // Componentオブジェクトからインスタンスを作成
    let instance = linker.instantiate(&mut store, &component)?;

    // 第三引数を鍵にインスタンスがエクスポートを探索 この場合は"your-namespase:greet/greetable"の実装を探す
    // 返り値はOption<CompnentExportIndex> unwrap()でエラー処理を楽にしている
    let greetable_index = instance.get_export(
        &mut store, 
        None, 
        "your-namespace:greet/greetable"
    ).unwrap();
    // 第二引数に先ほど取得したComponentExportIndexオブジェクトを指定 greet,nameという名前のものを探す
    let greet_index = instance
    .get_export(&mut store, Some(&greetable_index), "greet")
    .unwrap();
    let name_index = instance.get_export(&mut store, Some(&greetable_index), "name").unwrap();

    // greet関数をラップしたオブジェクトを作成 ジェネリクスには関数のパラメータと返り値の方をタプルで指定
    let greet: TypedFunc<(String, ), (String, )> = instance.get_typed_func(&mut store, greet_index).unwrap();
    // greet関数と同様の処理 name関数はパラメータを持たない関数なのでジェネリクスのパラメータの部分には()を指定
    let name: TypedFunc<(), (String, )> = instance.get_typed_func(&mut store, name_index).unwrap();

    let argument = "world!".to_string();

    // 返り値はタプルで返される 関数は複数の値を返すことがあるから
    let (return_value, ) = greet.call(&mut store, (argument, ))?;
    // このメソッドの呼び出しは必須 wasmコンポーネントに返り値の変換が終わったことを伝える
    greet.post_return(&mut store)?;

    println!("{return_value}");

    let (return_name, ) = name.call(&mut store, ())?;
    name.post_return(&mut store)?;
    let (return_value, ) = greet.call(&mut store, (return_name, ))?;
    greet.post_return(&mut store)?;

    println!("{return_value}");
    
    Ok(())
}

fn start45(args: Args) -> anyhow::Result<()> {
    let engine = Engine::default();
    let component = Component::from_file(&engine, &args.wasm_file)?;
    let linker = Linker::new(&engine);
    let mut store = Store::new(&engine, ());

    // 以下からが章の変更点

    // ロードしたライブラリコンポーネントのインスタンスを作成
    let provider = GreetableProvider::instantiate(&mut store, &component, &linker)?;

    // greetableインターフェースの実装をラップしたオブジェクト
    let greetable = provider.your_namespace_greet_greetable();

    // greet関数を呼び出す &strを渡す（前章との変更点）
    let message = greetable.call_greet(&mut store, "world")?;
    println!("{message}");

    // name関数を呼び出す
    let name = greetable.call_name(&mut store)?;
    let message = greetable.call_greet(&mut store, &name)?;
    println!("{message}");

    Ok(())
}


fn main() {
    let args = Args::parse();

    // ~4.4
    // if let Err(e) = start(args) {
    //     println!("{:?}", e);
    // }

    // 4.5
    if let Err(e) = start45(args) {
        println!("{:?}",e);
    }
}
