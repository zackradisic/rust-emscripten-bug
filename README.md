# Rust Emscripten bug related to threads + auto-dereferencing?

When compiling Rust with emscripten I get the following error:
```
note: [parse exception: attempted pop from empty stack / beyond block start boundary at 5012790 (at 0:5012790)]
          Fatal: error in parsing input
          emcc: error: '/Users/zackradisic/Desktop/Code/emsdk/upstream/bin/wasm-emscripten-finalize --minimize-wasm-changes --dyncalls-i64 /Users/zackradisic/Desktop/Code/modfy/framex/target/wasm32-unknown-emscripten/release/deps/framex.wasm -o /Users/zackradisic/Desktop/Code/modfy/framex/target/wasm32-unknown-emscripten/release/deps/framex.wasm --detect-features' failed (returned 1)
```

<details>
  <summary>Full error from rustc</summary>

```
error: linking with `emcc` failed: exit status: 1
  |
  = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_alloc\",\"_dealloc\",\"_init\",\"_main\",\"_print_codecs\",\"_set_opt\",\"_rust_eh_personality\"]" "/Users/zackradisic/Desktop/Code/modfy/framex/target/wasm32-unknown-emscripten/release/deps/framex.framex.f5802fb7-cgu.0.rcgu.o" "/Users/zackradisic/Desktop/Code/modfy/framex/target/wasm32-unknown-emscripten/release/deps/framex.framex.f5802fb7-cgu.1.rcgu.o" "/Users/zackradisic/Desktop/Code/modfy/framex/target/wasm32-unknown-emscripten/release/deps/framex.framex.f5802fb7-cgu.10.rcgu.o" "/Users/zackradisic/Desktop/Code/modfy/framex/target/wasm32-unknown-emscripten/release/deps/framex.framex.f5802fb7-cgu.11.rcgu.o" "/Users/zackradisic/Desktop/Code/modfy/framex/target/wasm32-unknown-emscripten/release/deps/framex.framex.f5802fb7-cgu.12.rcgu.o" "/Users/zackradisic/Desktop/Code/modfy/framex/target/wasm32-unknown-emscripten/release/deps/framex.framex.f5802fb7-cgu.13.rcgu.o" "/Users/zackradisic/Desktop/Code/modfy/framex/target/wasm32-unknown-emscripten/release/deps/framex.framex.f5802fb7-cgu.14.rcgu.o" "/Users/zackradisic/Desktop/Code/modfy/framex/target/wasm32-unknown-emscripten/release/deps/framex.framex.f5802fb7-cgu.15.rcgu.o" "/Users/zackradisic/Desktop/Code/modfy/framex/target/wasm32-unknown-emscripten/release/deps/framex.framex.f5802fb7-cgu.2.rcgu.o" "/Users/zackradisic/Desktop/Code/modfy/framex/target/wasm32-unknown-emscripten/release/deps/framex.framex.f5802fb7-cgu.3.rcgu.o" "/Users/zackradisic/Desktop/Code/modfy/framex/target/wasm32-unknown-emscripten/release/deps/framex.framex.f5802fb7-cgu.4.rcgu.o" "/Users/zackradisic/Desktop/Code/modfy/framex/target/wasm32-unknown-emscripten/release/deps/framex.framex.f5802fb7-cgu.5.rcgu.o" "/Users/zackradisic/Desktop/Code/modfy/framex/target/wasm32-unknown-emscripten/release/deps/framex.framex.f5802fb7-cgu.6.rcgu.o" "/Users/zackradisic/Desktop/Code/modfy/framex/target/wasm32-unknown-emscripten/release/deps/framex.framex.f5802fb7-cgu.7.rcgu.o" "/Users/zackradisic/Desktop/Code/modfy/framex/target/wasm32-unknown-emscripten/release/deps/framex.framex.f5802fb7-cgu.8.rcgu.o" "/Users/zackradisic/Desktop/Code/modfy/framex/target/wasm32-unknown-emscripten/release/deps/framex.framex.f5802fb7-cgu.9.rcgu.o" "/Users/zackradisic/Desktop/Code/modfy/framex/target/wasm32-unknown-emscripten/release/deps/framex.3m9l8ll8ayas3g7v.rcgu.o" "-L" "/Users/zackradisic/Desktop/Code/modfy/framex/target/wasm32-unknown-emscripten/release/deps" "-L" "/Users/zackradisic/Desktop/Code/modfy/framex/target/release/deps" "-L" "./wasm-libs/lib" "-L" "/Users/zackradisic/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/wasm32-unknown-emscripten/lib" "/Users/zackradisic/Desktop/Code/modfy/framex/target/wasm32-unknown-emscripten/release/deps/libframex-7860c6c7abb794c7.rlib" "/Users/zackradisic/Desktop/Code/modfy/framex/target/wasm32-unknown-emscripten/release/deps/libanyhow-612bcb44f12e1d42.rlib" "/Users/zackradisic/Desktop/Code/modfy/framex/target/wasm32-unknown-emscripten/release/deps/libffmpeg_next-54ea0b38a1519793.rlib" "/Users/zackradisic/Desktop/Code/modfy/framex/target/wasm32-unknown-emscripten/release/deps/libffmpeg_sys_next-8de54784e88d927f.rlib" "/Users/zackradisic/Desktop/Code/modfy/framex/target/wasm32-unknown-emscripten/release/deps/liblibc-ab293fa92d812261.rlib" "/Users/zackradisic/Desktop/Code/modfy/framex/target/wasm32-unknown-emscripten/release/deps/libbitflags-f0b7a29eeb4093a5.rlib" "/Users/zackradisic/Desktop/Code/modfy/framex/target/wasm32-unknown-emscripten/release/deps/libstd-35b925955873f6bf.rlib" "/Users/zackradisic/Desktop/Code/modfy/framex/target/wasm32-unknown-emscripten/release/deps/libpanic_unwind-194edf489af57a97.rlib" "/Users/zackradisic/Desktop/Code/modfy/framex/target/wasm32-unknown-emscripten/release/deps/libstd_detect-d58dd6fb52ab4b2d.rlib" "/Users/zackradisic/Desktop/Code/modfy/framex/target/wasm32-unknown-emscripten/release/deps/librustc_demangle-a1931f836e2321a3.rlib" "/Users/zackradisic/Desktop/Code/modfy/framex/target/wasm32-unknown-emscripten/release/deps/libhashbrown-93cb9f33d545cf77.rlib" "/Users/zackradisic/Desktop/Code/modfy/framex/target/wasm32-unknown-emscripten/release/deps/librustc_std_workspace_alloc-90ae5f2c3dc1e297.rlib" "/Users/zackradisic/Desktop/Code/modfy/framex/target/wasm32-unknown-emscripten/release/deps/libunwind-888b2c34d620fb42.rlib" "/Users/zackradisic/Desktop/Code/modfy/framex/target/wasm32-unknown-emscripten/release/deps/libcfg_if-e5bd1b9f540b98e1.rlib" "/Users/zackradisic/Desktop/Code/modfy/framex/target/wasm32-unknown-emscripten/release/deps/liblibc-f003f163c4e20282.rlib" "/Users/zackradisic/Desktop/Code/modfy/framex/target/wasm32-unknown-emscripten/release/deps/liballoc-b31681533b19d24b.rlib" "/Users/zackradisic/Desktop/Code/modfy/framex/target/wasm32-unknown-emscripten/release/deps/librustc_std_workspace_core-e54aa39c126b63d9.rlib" "/Users/zackradisic/Desktop/Code/modfy/framex/target/wasm32-unknown-emscripten/release/deps/libcore-7c8d732fc023986c.rlib" "/Users/zackradisic/Desktop/Code/modfy/framex/target/wasm32-unknown-emscripten/release/deps/libcompiler_builtins-70ec426a1130b25e.rlib" "-l" "avcodec" "-l" "avformat" "-l" "avfilter" "-l" "avdevice" "-l" "swresample" "-l" "postproc" "-l" "swscale" "-l" "avutil" "-l" "m" "-l" "mp3lame" "-l" "x264" "-l" "workerfs.js" "-l" "c" "-s" "DISABLE_EXCEPTION_CATCHING=0" "-L" "/Users/zackradisic/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/Users/zackradisic/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-o" "/Users/zackradisic/Desktop/Code/modfy/framex/target/wasm32-unknown-emscripten/release/deps/framex.js" "-O3" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-O3" "-pthread" "-s" "ALLOW_MEMORY_GROWTH=0" "-s" "INVOKE_RUN=0" "-s" "INITIAL_MEMORY=2146435072" "-s" "MODULARIZE=1" "-s" "EXPORT_NAME=framex" "-s" "USE_PTHREADS=1" "-s" "PROXY_TO_PTHREAD=1" "-s" "ENVIRONMENT=worker" "-s" "PTHREAD_POOL_SIZE=navigator.hardwareConcurrency" "-s" "EXPORTED_RUNTIME_METHODS=[FS,intArrayFromString,writeArrayToMemory,_malloc]" "gxx_personality_v0_stub.o" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
  = note: [parse exception: attempted pop from empty stack / beyond block start boundary at 5012790 (at 0:5012790)]
          Fatal: error in parsing input
          emcc: error: '/Users/zackradisic/Desktop/Code/emsdk/upstream/bin/wasm-emscripten-finalize --minimize-wasm-changes --dyncalls-i64 /Users/zackradisic/Desktop/Code/modfy/framex/target/wasm32-unknown-emscripten/release/deps/framex.wasm -o /Users/zackradisic/Desktop/Code/modfy/framex/target/wasm32-unknown-emscripten/release/deps/framex.wasm --detect-features' failed (returned 1)
```
  
</details>

I narrowed down the code causing this and it seems related to Rust's auto-dereferencing. This is the section of code, and you can see the type of parameter `s` in the closure is multiple nested references (`&&&str` or `&&str`). Rust's compiler will auto-dereference them to the appropriate type `&str`. This code does not compile.

<img width="852" alt="Screen Shot 2021-12-07 at 10 46 16 AM" src="https://user-images.githubusercontent.com/56137411/145006533-da8e6e2d-6254-4d5f-b9ed-755a239ef044.png">

However, when I use a [reference pattern](https://doc.rust-lang.org/reference/patterns.html#reference-patterns) to dereference the closure parameter, the code compiles and works as expected:

<img width="861" alt="Screen Shot 2021-12-07 at 10 46 02 AM" src="https://user-images.githubusercontent.com/56137411/145006902-880176a3-0828-47e8-89ff-1e6eb018c640.png">

I also tested this theory by trying to compile the following code and it fails with the same error:
```rust
let test: Vec<&str> = vec![
    "hi",
    "hello",
    "mitochondria is the powerhouse of the cell",
];

let test = test
    .iter()
    .filter(|s| s.contains("h")) // `s` has the type &&&str
    .map(|s| s.to_string())  // `s` has the type &&str
    .collect::<Vec<String>>();

println!("Values: {:?}", test);
```
Note that this error only occurs when compiling Rust with threads.

You can view the repo with the full reproduction [here](https://github.com/zackradisic/rust-emscripten-bug).



