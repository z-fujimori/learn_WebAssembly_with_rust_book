use clap::Parser;
use wasmtime::{component::{Component, Linker, TypedFunc},Engine, Store};

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

fn main() {
    let args = Args::parse();

    if let Err(e) = start(args) {
        println!("{:?}", e);
    }
}
