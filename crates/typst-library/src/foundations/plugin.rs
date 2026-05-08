use std::fmt::{self, Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};

use ecow::{EcoString, eco_format};
use typst_syntax::Spanned;
use wasmi::Memory;

use crate::diag::{At, SourceResult, StrResult, bail};
use crate::engine::Engine;
use crate::foundations::{Binding, Bytes, Func, Module, Scope, Value, cast, func, scope};
use crate::loading::{DataSource, Load};

/// WebAssemblyモジュールを読み込みます。
///
/// 結果として得られる[モジュール]($module)には、読み込まれたWebAssemblyモジュールの
/// 各関数エクスポートに対応する[関数]($function)がTypst関数として1つずつ含まれます。
///
/// TypstのWebAssemblyプラグインは、特定の[プロトコル]($plugin/#protocol)に
/// 従う必要があります。プラグインとして実行するには、プログラムは32ビットの共有
/// WebAssemblyライブラリにコンパイルする必要があります。
/// プラグイン関数は複数の[バイトバッファ]($bytes)を引数として受け取り、
/// 単一のバイトバッファを返すことができます。
/// プラグイン関数は通常、[`str`]($str/#constructor)、[`bytes`]($bytes/#constructor)、
/// [データ読み込み関数]($reference/data-loading)を活用してネイティブのTypst型と
/// バイト列との間で必要な変換を行うイディオマティックなTypst関数でラップされます。
///
/// セキュリティ上の理由から、プラグインはシステムから隔離して実行されます。
/// つまり、出力、ファイル読み込みなどの動作はサポートされません。
///
/// # 例
/// ```example
/// #let myplugin = plugin("hello.wasm")
/// #let concat(a, b) = str(
///   myplugin.concatenate(
///     bytes(a),
///     bytes(b),
///   )
/// )
///
/// #concat("hello", "world")
/// ```
///
/// plugin関数はモジュールを返すため、import構文と組み合わせて使えます。
/// ```typ
/// #import plugin("hello.wasm"): concatenate
/// ```
///
/// # 純粋性
/// プラグイン関数は**純粋でなければなりません**。
/// プラグイン関数の呼び出しは、以後のプラグイン呼び出しに観測可能な副作用を
/// 一切持たず、同じ引数に対して常に同じ値を返さなければなりません。
///
/// これは、Typstの関数が純粋でなければならない（言語設計上きわめて基本的な要件）
/// ためであり、Typst関数はプラグイン関数を呼び出せるため、この要件は引き継がれます。
/// 特に、プラグイン関数が同じ引数で2回呼び出された場合、Typstは結果をキャッシュし、
/// その関数を一度しか呼び出さないことがあります。
/// さらに、Typstはプラグインの複数のインスタンスを複数のスレッドで実行する可能性があり、
/// それらの間で状態は共有されません。
///
/// Typstはプラグイン関数の純粋性を強制しません（効率上の理由で）が、
/// 純粋でない関数を呼び出すと予測不可能で再現性のない結果につながるため、
/// 避けなければなりません。
///
/// とはいえ、コストの高いランタイム初期化を必要とするプラグインでは、
/// 可変な操作が役立つ場合もあります。
/// 純粋性の要件のため、そうした初期化は通常の関数呼び出しでは実行できません。
/// 代わりに、Typstは[plugin transition API]($plugin.transition)を提供しています。
/// これは関数呼び出しを実行し、transition呼び出しによって生じた副作用を観測する
/// 新しい関数を持つ派生モジュールを生成します。元のプラグインは影響を受けません。
///
/// # プラグインとパッケージ
/// 任意のTypstコードは、WebAssemblyファイルを含めて読み込むだけでプラグインを
/// 利用できます。しかし、バイトベースのプラグインインターフェースはかなり低レベルなため、
/// 通常プラグインはイディオマティックなラッパー関数を含むパッケージを通して提供されます。
///
/// # WASI
/// 多くのコンパイラは、デフォルトで、または唯一の選択肢として
/// [WASI ABI](https://wasi.dev/)を使用します（例：emscripten）。
/// これは出力やファイル読み込みなどを許可します。
/// このABIはTypstでは直接動作しません。
/// 別のターゲットにコンパイルするか、
/// [全ての関数をスタブ化](https://github.com/astrale-sharp/wasm-minimal-protocol/tree/master/crates/wasi-stub)
/// する必要があります。
///
/// # プロトコル
/// プラグインとして使うには、WebAssemblyモジュールが以下のプロトコルに
/// 準拠している必要があります。
///
/// ## エクスポート
/// プラグインモジュールは、Typstから呼び出せるように関数をエクスポートできます。
/// プロトコルに準拠するために、エクスポート関数は次のようにすべきです。
///
/// - `n`個の32ビット整数引数`a_1`、`a_2`、…、`a_n`（長さとして解釈されるため、
///   `usize/size_t`が望ましい場合があります）を取り、1つの32ビット整数を返す。
///
/// - 関数はまず長さ`a_1 + a_2 + ... + a_n`のバッファ`buf`を確保し、
///   次に`wasm_minimal_protocol_write_args_to_buffer(buf.ptr)`を呼び出す。
///
/// - これでバッファの最初の`a_1`バイトが第1引数、次の`a_2`バイトが第2引数、
///   というように構成される。
///
/// - 関数は引数を用いて処理を行い、出力バッファを生成できる。
///   返却前に`wasm_minimal_protocol_send_result_to_host`を呼び出して、
///   結果をホストに送信すべきである。
///
/// - 成功を通知するには、関数は`0`を返すべきである。
///
/// - エラーを通知するには、関数は`1`を返すべきである。書き込まれたバッファは
///   UTF-8エンコードされたエラーメッセージとして解釈される。
///
/// ## インポート
/// プラグインモジュールは、ランタイムが提供する2つの関数をインポートする必要があります。
/// （型と関数はWAT構文で記述されています）
///
/// - `(import "typst_env" "wasm_minimal_protocol_write_args_to_buffer" (func
///   (param i32)))`
///
///   現在の関数の引数を、プラグインが確保したバッファに書き込みます。
///   プラグイン関数が呼び出されると、入力バッファの[長さを引数として
///   受け取ります](#exports)。
///   その後、これらの長さの合計以上の容量を持つバッファを確保すべきです。
///   そしてバッファへの`ptr`とともにこの関数を呼び出し、引数を順番に書き込ませます。
///
/// - `(import "typst_env" "wasm_minimal_protocol_send_result_to_host" (func
///   (param i32 i32)))`
///
///   現在の関数の出力をホスト（Typst）に送信します。
///   第1引数はバッファへのポインタ（`ptr`）、第2引数はそのバッファの長さ（`len`）です。
///   `ptr`が指すメモリは、この関数の戻り直後に解放可能です。
///   メッセージをエラーメッセージとして解釈する場合、UTF-8でエンコードされている必要があります。
///
/// # リソース
/// より多くのリソースについては、[wasm-minimal-protocolリポジトリ](https://github.com/astrale-sharp/wasm-minimal-protocol)
/// を参照してください。これには次のものが含まれています。
///
/// - プラグイン実装の例の一覧と、これらの例のテストランナー
/// - Rustでプラグインを書くためのラッパー（Zigラッパーは開発中）
/// - WASIのスタバー
#[func(scope)]
pub fn plugin(
    engine: &mut Engine,
    /// WebAssemblyファイルへの[パス]($syntax/#paths)、または生のWebAssemblyバイト列。
    source: Spanned<DataSource>,
) -> SourceResult<Module> {
    let loaded = source.load(engine.world)?;
    Plugin::module(loaded.data).at(source.span)
}

#[scope]
impl plugin {
    /// 副作用を持つプラグイン関数を呼び出し、可変呼び出しの結果を観測することが
    /// 保証されたプラグイン関数を持つ新しいモジュールを返します。
    ///
    /// 通常の関数呼び出しを通じて（transition APIを使わずに）純粋でない関数を呼び出すことは
    /// 禁止されており、予測不可能な挙動につながる点に注意してください。
    /// 詳細は[純粋性に関する節]($plugin/#purity)を参照してください。
    ///
    /// 以下の例では、2つの関数をエクスポートするプラグイン`hello-mut.wasm`を読み込みます。
    /// `get()`関数はグローバル配列を文字列として取得します。
    /// `add(value)`関数はグローバル配列に値を追加します。
    ///
    /// transition APIを通して`add`を呼び出します。
    /// 派生モジュールに対する`mutated.get()`の呼び出しは、追加を観測します。
    /// 一方、元のモジュールは`base.get()`の呼び出しが示すように、変更されません。
    ///
    /// _注:_ 内部のWebAssembly実装の制限により、transition APIはプラグインのメモリ内の
    /// 変更を反映することのみを保証でき、WebAssemblyグローバル変数の変更は保証しません。
    /// プラグインがtransition後にグローバル変数の変更が見えることに依存している場合、
    /// 当面はtransition APIの使用を避けるとよいでしょう。
    /// 将来的にはこの制限が解消されることを期待しています。
    ///
    /// ```typ
    /// #let base = plugin("hello-mut.wasm")
    /// #assert.eq(base.get(), "[]")
    ///
    /// #let mutated = plugin.transition(base.add, "hello")
    /// #assert.eq(base.get(), "[]")
    /// #assert.eq(mutated.get(), "[hello]")
    /// ```
    #[func]
    pub fn transition(
        /// 呼び出すプラグイン関数。
        func: PluginFunc,
        /// 関数を呼び出すバイトバッファ。
        #[variadic]
        arguments: Vec<Bytes>,
    ) -> StrResult<Module> {
        func.transition(arguments)
    }
}

/// WebAssemblyプラグインから読み込まれた関数。
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct PluginFunc {
    /// The underlying plugin, shared by this and the other functions.
    plugin: Arc<Plugin>,
    /// The name of the plugin function.
    name: EcoString,
}

impl PluginFunc {
    /// The name of the plugin function.
    pub fn name(&self) -> &EcoString {
        &self.name
    }

    /// Call the WebAssembly function with the given arguments.
    #[comemo::memoize]
    #[typst_macros::time(name = "call plugin")]
    pub fn call(&self, args: Vec<Bytes>) -> StrResult<Bytes> {
        self.plugin.call(&self.name, args)
    }

    /// Transition a plugin and turn the result into a module.
    #[comemo::memoize]
    #[typst_macros::time(name = "transition plugin")]
    pub fn transition(&self, args: Vec<Bytes>) -> StrResult<Module> {
        self.plugin.transition(&self.name, args).map(Plugin::into_module)
    }
}

cast! {
    PluginFunc,
    self => Value::Func(self.into()),
    v: Func => v.to_plugin().ok_or("expected plugin function")?.clone(),
}

/// A plugin with potentially multiple instances for multi-threaded
/// execution.
struct Plugin {
    /// Shared by all variants of the plugin.
    base: Arc<PluginBase>,
    /// A pool of plugin instances.
    ///
    /// When multiple plugin calls run concurrently due to multi-threading, we
    /// create new instances whenever we run out of ones.
    pool: Mutex<Vec<PluginInstance>>,
    /// A snapshot that new instances should be restored to.
    snapshot: Option<Snapshot>,
    /// A combined hash that incorporates all function names and arguments used
    /// in transitions of this plugin, such that this plugin has a deterministic
    /// hash and equality check that can differentiate it from "siblings" (same
    /// base, different transitions).
    fingerprint: u128,
}

impl Plugin {
    /// Create a plugin and turn it into a module.
    #[comemo::memoize]
    #[typst_macros::time(name = "load plugin")]
    fn module(bytes: Bytes) -> StrResult<Module> {
        Self::new(bytes).map(Self::into_module)
    }

    /// Create a new plugin from raw WebAssembly bytes.
    fn new(bytes: Bytes) -> StrResult<Self> {
        let mut config = wasmi::Config::default();

        // Disable relaxed SIMD as it can introduce non-determinism.
        config.wasm_relaxed_simd(false);

        let engine = wasmi::Engine::new(&config);
        let module = wasmi::Module::new(&engine, bytes.as_slice())
            .map_err(|err| format!("failed to load WebAssembly module ({err})"))?;

        // Ensure that the plugin exports its memory.
        if !matches!(module.get_export("memory"), Some(wasmi::ExternType::Memory(_))) {
            bail!("plugin does not export its memory");
        }

        let mut linker = wasmi::Linker::new(&engine);
        linker
            .func_wrap(
                "typst_env",
                "wasm_minimal_protocol_send_result_to_host",
                wasm_minimal_protocol_send_result_to_host,
            )
            .unwrap();
        linker
            .func_wrap(
                "typst_env",
                "wasm_minimal_protocol_write_args_to_buffer",
                wasm_minimal_protocol_write_args_to_buffer,
            )
            .unwrap();

        let base = Arc::new(PluginBase { bytes, linker, module });
        let instance = PluginInstance::new(&base, None)?;

        Ok(Self {
            base,
            snapshot: None,
            fingerprint: 0,
            pool: Mutex::new(vec![instance]),
        })
    }

    /// Execute a function with access to an instsance.
    fn call(&self, func: &str, args: Vec<Bytes>) -> StrResult<Bytes> {
        // Acquire an instance from the pool (potentially creating a new one).
        let mut instance = self.acquire()?;

        // Execute the call on an instance from the pool. If the call fails, we
        // return early and _don't_ return the instance to the pool as it might
        // be irrecoverably damaged.
        let output = instance.call(func, args)?;

        // Return the instance to the pool.
        self.pool.lock().unwrap().push(instance);

        Ok(output)
    }

    /// Call a mutable plugin function, producing a new mutable whose functions
    /// are guaranteed to be able to observe the mutation.
    fn transition(&self, func: &str, args: Vec<Bytes>) -> StrResult<Plugin> {
        // Derive a new transition hash from the old one and the function and arguments.
        let fingerprint = typst_utils::hash128(&(self.fingerprint, func, &args));

        // Execute the mutable call on an instance.
        let mut instance = self.acquire()?;

        // Call the function. If the call fails, we return early and _don't_
        // return the instance to the pool as it might be irrecoverably damaged.
        instance.call(func, args)?;

        // Snapshot the instance after the mutable call.
        let snapshot = instance.snapshot();

        // Create a new plugin and move (this is important!) the used instance
        // into it, so that the old plugin won't observe the mutation. Also
        // save the snapshot so that instances that are initialized for the
        // transitioned plugin's pool observe the mutation.
        Ok(Self {
            base: self.base.clone(),
            snapshot: Some(snapshot),
            fingerprint,
            pool: Mutex::new(vec![instance]),
        })
    }

    /// Acquire an instance from the pool (or create a new one).
    fn acquire(&self) -> StrResult<PluginInstance> {
        // Don't use match to ensure that the lock is released before we create
        // a new instance.
        if let Some(instance) = self.pool.lock().unwrap().pop() {
            return Ok(instance);
        }

        PluginInstance::new(&self.base, self.snapshot.as_ref())
    }

    /// Turn a plugin into a Typst module containing plugin functions.
    fn into_module(self) -> Module {
        let shared = Arc::new(self);

        // Build a scope from the collected functions.
        let mut scope = Scope::new();
        for export in shared.base.module.exports() {
            if matches!(export.ty(), wasmi::ExternType::Func(_)) {
                let name = EcoString::from(export.name());
                let func = PluginFunc { plugin: shared.clone(), name: name.clone() };
                scope.bind(name, Binding::detached(Func::from(func)));
            }
        }

        Module::anonymous(scope)
    }
}

impl Debug for Plugin {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.pad("Plugin(..)")
    }
}

impl PartialEq for Plugin {
    fn eq(&self, other: &Self) -> bool {
        self.base.bytes == other.base.bytes && self.fingerprint == other.fingerprint
    }
}

impl Hash for Plugin {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base.bytes.hash(state);
        self.fingerprint.hash(state);
    }
}

/// Shared by all pooled & transitioned variants of the plugin.
struct PluginBase {
    /// The raw WebAssembly bytes.
    bytes: Bytes,
    /// The compiled WebAssembly module.
    module: wasmi::Module,
    /// A linker used to create a `Store` for execution.
    linker: wasmi::Linker<CallData>,
}

/// An single plugin instance for single-threaded execution.
struct PluginInstance {
    /// The underlying wasmi instance.
    instance: wasmi::Instance,
    /// The execution store of this concrete plugin instance.
    store: wasmi::Store<CallData>,
}

/// A snapshot of a plugin instance.
struct Snapshot {
    /// The number of pages in the main memory.
    mem_pages: u64,
    /// The data in the main memory.
    mem_data: Vec<u8>,
}

impl PluginInstance {
    /// Create a new execution instance of a plugin, potentially restoring
    /// a snapshot.
    #[typst_macros::time(name = "create plugin instance")]
    fn new(base: &PluginBase, snapshot: Option<&Snapshot>) -> StrResult<PluginInstance> {
        let mut store = wasmi::Store::new(base.linker.engine(), CallData::default());
        let instance = base
            .linker
            .instantiate_and_start(&mut store, &base.module)
            .map_err(|e| eco_format!("{e}"))?;

        let mut instance = PluginInstance { instance, store };
        if let Some(snapshot) = snapshot {
            instance.restore(snapshot);
        }
        Ok(instance)
    }

    /// Call a plugin function with byte arguments.
    fn call(&mut self, func: &str, args: Vec<Bytes>) -> StrResult<Bytes> {
        let handle = self
            .instance
            .get_export(&self.store, func)
            .unwrap()
            .into_func()
            .unwrap();
        let ty = handle.ty(&self.store);

        // Check function signature. Do this lazily only when a function is called
        // because there might be exported functions like `_initialize` that don't
        // match the schema.
        if ty.params().iter().any(|&v| v != wasmi::core::ValType::I32) {
            bail!(
                "plugin function `{func}` has a parameter that is not a 32-bit integer"
            );
        }
        if ty.results() != [wasmi::core::ValType::I32] {
            bail!("plugin function `{func}` does not return exactly one 32-bit integer");
        }

        // Check inputs.
        let expected = ty.params().len();
        let given = args.len();
        if expected != given {
            bail!(
                "plugin function takes {expected} argument{}, but {given} {} given",
                if expected == 1 { "" } else { "s" },
                if given == 1 { "was" } else { "were" },
            );
        }

        // Collect the lengths of the argument buffers.
        let lengths = args
            .iter()
            .map(|a| wasmi::Val::I32(a.len() as i32))
            .collect::<Vec<_>>();

        // Store the input data.
        self.store.data_mut().args = args;

        // Call the function.
        let mut code = wasmi::Val::I32(-1);
        handle
            .call(&mut self.store, &lengths, std::slice::from_mut(&mut code))
            .map_err(|err| eco_format!("plugin panicked: {err}"))?;

        if let Some(MemoryError { offset, length, write }) =
            self.store.data_mut().memory_error.take()
        {
            return Err(eco_format!(
                "plugin tried to {kind} out of bounds: \
                 pointer {offset:#x} is out of bounds for {kind} of length {length}",
                kind = if write { "write" } else { "read" }
            ));
        }

        // Extract the returned data.
        let output = std::mem::take(&mut self.store.data_mut().output);

        // Parse the functions return value.
        match code {
            wasmi::Val::I32(0) => {}
            wasmi::Val::I32(1) => match std::str::from_utf8(&output) {
                Ok(message) => bail!("plugin errored with: {message}"),
                Err(_) => {
                    bail!("plugin errored, but did not return a valid error message")
                }
            },
            _ => bail!("plugin did not respect the protocol"),
        };

        Ok(Bytes::new(output))
    }

    /// Creates a snapshot of this instance from which another one can be
    /// initialized.
    #[typst_macros::time(name = "save snapshot")]
    fn snapshot(&self) -> Snapshot {
        let memory = self.memory();
        let mem_pages = memory.size(&self.store);
        let mem_data = memory.data(&self.store).to_vec();
        Snapshot { mem_pages, mem_data }
    }

    /// Restores the instance to a snapshot.
    #[typst_macros::time(name = "restore snapshot")]
    fn restore(&mut self, snapshot: &Snapshot) {
        let memory = self.memory();
        let current_size = memory.size(&self.store);
        if current_size < snapshot.mem_pages {
            memory
                .grow(&mut self.store, snapshot.mem_pages - current_size)
                .unwrap();
        }

        memory.data_mut(&mut self.store)[..snapshot.mem_data.len()]
            .copy_from_slice(&snapshot.mem_data);
    }

    /// Retrieves a handle to the plugin's main memory.
    fn memory(&self) -> Memory {
        self.instance
            .get_export(&self.store, "memory")
            .unwrap()
            .into_memory()
            .unwrap()
    }
}

/// The persistent store data used for communication between store and host.
#[derive(Default)]
struct CallData {
    /// Arguments for a current call.
    args: Vec<Bytes>,
    /// The results of the current call.
    output: Vec<u8>,
    /// A memory error that occurred during execution of the current call.
    memory_error: Option<MemoryError>,
}

/// If there was an error reading/writing memory, keep the offset + length to
/// display an error message.
struct MemoryError {
    offset: u32,
    length: u32,
    write: bool,
}

/// Write the arguments to the plugin function into the plugin's memory.
fn wasm_minimal_protocol_write_args_to_buffer(
    mut caller: wasmi::Caller<CallData>,
    ptr: u32,
) {
    let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
    let arguments = std::mem::take(&mut caller.data_mut().args);
    let mut offset = ptr as usize;
    for arg in arguments {
        if memory.write(&mut caller, offset, arg.as_slice()).is_err() {
            caller.data_mut().memory_error = Some(MemoryError {
                offset: offset as u32,
                length: arg.len() as u32,
                write: true,
            });
            return;
        }
        offset += arg.len();
    }
}

/// Extracts the output of the plugin function from the plugin's memory.
fn wasm_minimal_protocol_send_result_to_host(
    mut caller: wasmi::Caller<CallData>,
    ptr: u32,
    len: u32,
) {
    let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
    let mut buffer = std::mem::take(&mut caller.data_mut().output);
    buffer.resize(len as usize, 0);
    if memory.read(&caller, ptr as _, &mut buffer).is_err() {
        caller.data_mut().memory_error =
            Some(MemoryError { offset: ptr, length: len, write: false });
        return;
    }
    caller.data_mut().output = buffer;
}
