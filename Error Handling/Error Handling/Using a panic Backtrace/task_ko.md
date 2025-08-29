### `panic!` 백트레이스 사용하기

다른 예제를 살펴보며, `panic!` 호출이 매크로를 직접 호출한 코드 대신, 코드의 버그로 인해 라이브러리에서 비롯된 경우 어떤 일이 발생하는지 알아보겠습니다. 아래 코드 스니펫은 벡터에서 인덱스로 요소에 접근하려고 시도하는 일부 코드를 보여줍니다.

```rust
    fn main() {
        let v = vec![1, 2, 3];

        v[99];
    }
```

##### 벡터의 끝을 초과한 요소에 접근을 시도하기, 이는 `panic!` 호출을 유발합니다.

여기에서 우리는 벡터의 100번째 요소(인덱스가 99인 요소에 접근을 시도), 하지만 벡터에는 3개의 요소만 있습니다. 이런 상황에서 Rust는 패닉합니다. `[]`를 사용하는 것은 요소를 반환하도록 설계되었지만, 잘못된 인덱스를 전달하면 Rust는 반환할 수 있는 올바른 요소가 없기 때문에 이런 일이 발생합니다.

C와 같은 다른 언어는 이 상황에서 프로그램에 요구한 그대로를 제공하려고 시도하지만, 이것은 여러분이 원하는 것이 아닐 수 있습니다. 대신 벡터에서 벗어난 메모리 위치를 반환할 수 있으며, 이는 실제로 벡터의 소유가 아닌 메모리일 수 있습니다. 이를 _버퍼 오버리드(buffer overread)_ 라고 하며 공격자가 인덱스를 조작하여 배열 뒤에 저장된 읽어서는 안 되는 데이터를 읽을 수 있는 보안 취약점을 유발할 수 있습니다.

이러한 종류의 취약점으로부터 프로그램을 보호하기 위해, 존재하지 않는 인덱스의 요소를 읽으려고 하면 Rust는 실행을 중지하고 계속 실행하지 않습니다. 이를 직접 확인해 봅시다.

```text
   Compiling test_rust_project v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/test_rust_project`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', /rustc/3c235d5600393dfe6c36eeed34042efad8d4f26e/src/libcore/slice/mod.rs:2686:10
```
이 에러는 우리가 작성하지 않은 파일, _libcore/slice/mod.rs_ 에 대해 알려줍니다. 이 파일은 Rust 소스 코드 내에서 `slice`의 구현을 포함합니다. 우리가 벡터 `v`에서 `[]`를 사용할 때 실행되는 코드는 _libcore/slice/mod.rs_ 에 있으며, 여기서 실제로 `panic!`이 발생합니다.

다음으로 나오는 주석 라인은 `RUST_BACKTRACE` 환경 변수를 설정하여 오류의 원인을 자세히 확인할 수 있는 백트레이스를 얻을 수 있음을 알려줍니다. _백트레이스(backtrace)_ 는 문제 발생 지점에 도달하기까지 호출된 모든 함수들의 리스트입니다. Rust에서 백트레이스는 다른 언어에서와 동일하게 작동합니다. 백트레이스를 읽는 핵심은 상단에서부터 시작하여 여러분이 작성한 파일들이 언급될 때까지 읽는 것입니다. 여기에서 문제가 발생한 것입니다. 여러분이 작성한 파일을 언급한 라인의 위쪽은 여러분의 코드가 호출한 코드이고, 아래쪽은 여러분의 코드를 호출한 코드입니다. 이 줄들에는 Rust의 코어 코드, 표준 라이브러리 코드, 사용 중인 크레이트가 포함될 수 있습니다. `RUST_BACKTRACE` 환경 변수를 0이 아닌 값으로 설정하면 백트레이스를 확인할 수 있습니다. 아래 코드 스니펫은 볼 수 있는 출력과 유사한 결과를 보여줍니다.

```console
$ RUST_BACKTRACE=1 cargo run
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/panicking.rs:483
   1: core::panicking::panic_fmt
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/panicking.rs:85
   2: core::panicking::panic_bounds_check
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/panicking.rs:62
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/slice/index.rs:255
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/slice/index.rs:15
   5: <alloc::vec::Vec<T> as core::ops::index::Index<I>>::index
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/alloc/src/vec.rs:1982
   6: panic::main
             at ./src/main.rs:4
   7: core::ops::function::FnOnce::call_once
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/ops/function.rs:227
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

##### 환경 변수 `RUST_BACKTRACE`가 설정되었을 때 출력된 `panic!` 호출로 생성된 백트레이스

출력이 굉장히 많습니다! 운영 체제 및 Rust 버전에 따라 정확한 출력 내용은 달라질 수 있습니다. 이러한 정보를 포함한 백트레이스를 얻으려면 디버그 심볼이 활성화되어 있어야 합니다. `cargo build` 또는 `cargo run`을 `--release` 플래그 없이 사용하면 디버그 심볼이 기본적으로 활성화됩니다.

위 코드 스니펫의 출력에서, 백트레이스의 6번 라인은 프로젝트에서 문제를 발생시키는 줄을 가리킵니다: _src/main.rs_ 파일의 4번 라인입니다. 프로그램이 패닉하지 않도록 만들고 싶다면, 여러분이 작성한 파일에서 언급된 첫 번째 줄이 문제 해결을 시작해야 하는 위치입니다. 첫 번째 코드 스니펫에서는 백트레이스를 사용하는 방법을 시연하기 위해 의도적으로 패닉을 유발하는 코드를 작성했으며, 이 문제를 해결하려면 3개의 요소만 포함된 벡터에서 인덱스 99의 요소를 요청하지 않아야 합니다. 미래에 코드가 패닉을 일으킬 경우, 코드가 어떤 값으로 어떤 작업을 수행했는지, 무엇이 오류를 유발했는지, 코드가 대신 무엇을 해야 하는지 파악해야 합니다.

우리는 이 장의 나중에 [“`panic!` 사용 여부”](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html#to-panic-or-not-to-panic) 섹션에서 `panic!`을 언제 사용하고 사용하지 말아야 할지에 대해 다시 논의할 것입니다. 다음으로, `Result`를 사용하여 오류에서 복구하는 방법을 살펴보겠습니다.

_Rust 프로그래밍 언어 책에서 다음 챕터를 참조할 수 있습니다:
[패닉과 함께하는 복구 불가능한 오류](https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html#unrecoverable-errors-with-panic)_