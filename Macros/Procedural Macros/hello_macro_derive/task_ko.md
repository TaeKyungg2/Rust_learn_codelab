### 2단계: 절차적 매크로 정의하기

다음 단계는 절차적 매크로를 정의하는 것입니다. 이 글을 작성하는 시점 기준으로, 절차적 매크로는 별도의 크레이트에서 정의되어야 합니다. 이 제한은 나중에 해제될 가능성이 있습니다. 크레이트와 매크로 크레이트를 구조화하는 관례는 다음과 같습니다: `foo`라는 이름의 크레이트가 있다면, 사용자 정의 `derive` 절차적 매크로 크레이트는 `foo_derive`라고 합니다.

여기서는 `Cargo.toml` 파일에 정의된 새 크레이트 `hello_macro_derive`를 사용합니다.

이 두 크레이트는 상호 밀접하게 관련되어 있으므로, `hello_macro` 크레이트 디렉터리 내에 절차적 매크로 크레이트를 생성합니다. 만약 `hello_macro`의 트레이트 정의를 변경한다면, `hello_macro_derive` 내 절차적 매크로 구현도 변경해야 합니다. 이 두 크레이트는 각각 별도로 배포되어야 하며, 이를 사용하는 프로그래머는 두 크레이트를 모두 의존성에 추가하고 각각 범위에 가져와야 합니다. 대신, `hello_macro` 크레이트가 `hello_macro_derive`를 종속성으로 사용하고 절차적 매크로 코드를 재내보내도록 할 수도 있습니다. 그러나 우리가 프로젝트를 구조화한 방식은 프로그래머가 `derive` 기능을 원하지 않더라도 `hello_macro`를 사용할 수 있도록 합니다.

`hello_macro_derive`를 절차적 매크로 크레이트로 선언해야 합니다. 바로 사용할 `syn` 및 `quote` 크레이트와 같은 기능도 필요하므로 이를 종속성에 추가해야 합니다. 아래 내용을 `hello_macro_derive` 크레이트의 _Cargo.toml_ 파일에 추가합니다:

```toml
    [lib]
    proc-macro = true

    [dependencies]
    syn = "1.0"
    quote = "1.0"
```

절차적 매크로를 정의하려면, 아래 코드 스니펫을 `hello_macro_derive` 크레이트의 _src/lib.rs_ 파일에 추가하세요. 이 코드는 `impl_hello_macro` 함수에 대한 정의를 추가하기 전까지 컴파일되지 않을 것입니다.

```rust
    extern crate proc_macro;

    use crate::proc_macro::TokenStream;
    use quote::quote;
    use syn;

    #[proc_macro_derive(HelloMacro)]
    pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
        // 구문 트리로 Rust 코드를 표현하는 구조 생성
        // 이 구문 트리를 조작할 수 있음
        let ast = syn::parse(input).unwrap();

        // 트레이트 구현 생성
        impl_hello_macro(&ast)
    }
```

##### 절차적 매크로 크레이트에서 Rust 코드를 처리하기 위해 필요한 코드

위 코드를 보면, `hello_macro_derive` 함수와 `impl_hello_macro` 함수로 코드를 나누었습니다. `hello_macro_derive` 함수는 `TokenStream`을 파싱하는 역할을 하며, `impl_hello_macro` 함수는 구문 트리를 변환하는 역할을 합니다. 이렇게 하면 절차적 매크로를 작성하는 작업이 더 간편해집니다. 외부 함수(`hello_macro_derive`의 경우) 내 코드는 여러분이 봤거나 작성할 거의 모든 절차적 매크로 크레이트에서 동일하게 사용될 것입니다. 내부 함수(`impl_hello_macro`의 경우) 본문에 작성하는 코드는 매크로의 목적에 따라 달라집니다.

여기서는 세 가지 새로운 크레이트를 소개했습니다: `proc_macro`, [`syn`](https://crates.io/crates/syn), 그리고 [`quote`](https://crates.io/crates/quote). `proc_macro` 크레이트는 Rust에 내장되어 있으므로 _Cargo.toml_에 추가하지 않아도 됩니다. `proc_macro` 크레이트는 컴파일러의 API로, 이를 통해 Rust 코드를 읽고 조작할 수 있습니다.

`syn` 크레이트는 문자열로부터 Rust 코드를 데이터 구조로 변환하여, 해당 데이터 구조에서 작업을 수행할 수 있습니다. `quote` 크레이트는 다시 `syn` 데이터 구조를 Rust 코드로 변환합니다. 이러한 크레이트를 사용하면 처리하고자 하는 Rust 코드를 파싱하는 작업이 훨씬 간단해집니다: Rust 코드를 전체적으로 파싱하는 코드를 작성하는 것은 간단한 작업이 아닙니다.

`hello_macro_derive` 함수는 라이브러리 사용자가 어떤 타입에 대해 `#[derive(HelloMacro)]`를 지정하면 호출됩니다. 이는 `hello_macro_derive` 함수에 `proc_macro_derive`로 어노테이트하고, 트레이트 이름과 일치하는 `HelloMacro`라는 이름을 지정했기 때문에 가능합니다. 이는 대부분의 절차적 매크로가 따르는 관례입니다.

`hello_macro_derive` 함수는 먼저 `input`을 `TokenStream`에서 우리가 이해하고 작업할 수 있는 데이터 구조로 변환합니다. 여기에 `syn`이 사용됩니다. `syn`의 `parse` 함수는 `TokenStream`을 받아 Rust 코드를 나타내는 `DeriveInput` 구조체를 반환합니다. 아래 예제는 `struct Pancakes;` 문자열을 파싱하여 얻은 `DeriveInput` 구조체 관련 부분을 보여줍니다:

```rust
    DeriveInput {
        // --생략--

        ident: Ident {
            ident: "Pancakes",
            span: #0 bytes(95..103)
        },
        data: Struct(
            DataStruct {
                struct_token: Struct,
                fields: Unit,
                semi_token: Some(
                    Semi
                )
            }
        )
    }
```

##### 절차적 매크로에서 어트리뷰트를 포함한 코드를 파싱할 때 얻는 DeriveInput 인스턴스

이 구조체의 필드는 우리가 파싱한 Rust 코드가 이름(식별자)이 `Pancakes`인 유닛 구조체임을 보여줍니다. 해당 구조체에는 다양한 Rust 코드를 설명하는 더 많은 필드가 있습니다. 자세한 정보는 [`syn`의 `DeriveInput` 문서](https://docs.rs/syn/0.14.4/syn/struct.DeriveInput.html)를 확인하세요.

이제 `impl_hello_macro` 함수를 정의하여 어노테이트된 타입에 대해 `HelloMacro` 트레이트를 구현하는 새 Rust 코드를 생성하겠습니다. 아래 코드 스니펫은 이를 보여줍니다.

```rust
    fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
        let name = &ast.ident;
        let gen = quote! {
            impl HelloMacro for #name {
                fn hello_macro() {
                    println!("Hello, Macro! My name is {}", stringify!(#name));
                }
            }
        };
        gen.into()
    }
```

##### 파싱된 Rust 코드를 사용하여 HelloMacro 트레이트 구현하기

`ast.ident`를 사용하여 어노테이트된 타입의 이름(식별자)을 포함하는 `Ident` 구조체 인스턴스를 얻습니다. DeriveInput 인스턴스가 있는 예제 구조체는 절차적 매크로 예제에서 코드를 실행할 때 `impl_hello_macro` 함수에 의해 반환된 `ident`가 `"Pancakes"` 값을 가지는 `ident` 필드를 포함함을 보여줍니다. 따라서 위 스니펫의 `name` 변수는 `Ident` 구조체 인스턴스를 포함하며, 이를 출력하면 절차적 매크로 예제에서 `struct`의 이름 `"Pancakes"` 문자열이 됩니다.

`quote!` 매크로를 사용하여 반환할 Rust 코드를 정의합니다. 컴파일러는 `quote!` 매크로 실행의 직접적인 결과와는 다른 것을 기대하므로, 이를 `TokenStream`으로 변환해야 합니다. 이는 `into` 메서드를 호출하여, 이 중간 표현을 소비하고 필요한 `TokenStream` 유형의 값을 반환함으로써 수행됩니다.

`quote!` 매크로는 매우 유용한 템플릿 메커니즘도 제공합니다. 예를 들어, `#name`을 입력하면 `quote!`가 해당 변수가 가진 값을 대체합니다. 심지어 일반 매크로가 작동하는 방식과 유사한 반복 작업도 가능합니다. [`quote` 크레이트 문서](https://docs.rs/quote)를 확인하여 자세한 내용을 알아보세요.

우리의 절차적 매크로는 사용자가 어노테이트한 타입에 대해 `HelloMacro` 트레이트 구현을 생성하도록 설계되었습니다. 이를 위해 `#name`을 사용하여 타입을 얻습니다. 트레이트 구현에는 함수 `hello_macro`가 있으며, 이 함수 본문은 우리가 제공하려는 기능인 "Hello, Macro! My name is"를 출력하고 어노테이트된 타입의 이름을 출력합니다.

여기 사용된 `stringify!` 매크로는 Rust에 내장되어 있습니다. 이 매크로는 `1 + 2`와 같은 Rust 표현식을 받아 컴파일 타임에 이를 `"1 + 2"`와 같은 문자열 리터럴로 변환합니다. 이는 표현식을 평가한 후 결과를 `String`으로 변환하는 `format!` 또는 `println!` 매크로와는 다릅니다. `#name` 입력이 문자 그대로 출력해야 하는 표현식일 가능성도 있으므로 `stringify!`를 사용합니다. 또한, `stringify!`를 사용하면 컴파일 타임에 `#name`을 문자열 리터럴로 변환함으로써 할당을 절약할 수 있습니다.

이제 `hello_macro`와 `hello_macro_derive`에서 `cargo build`가 성공적으로 완료되어야 합니다. 절차적 매크로를 예제로 작성한 코드에 연결하여 작동하는 모습을 보겠습니다! _projects_ 디렉터리에서 `cargo new pancakes`를 사용하여 새 바이너리 프로젝트를 만드세요. `pancakes` 크레이트의 _Cargo.toml_에 `hello_macro` 및 `hello_macro_derive`를 종속성으로 추가해야 합니다. 만약 여러분의 `hello_macro`와 `hello_macro_derive`를 _https://crates.io/_에 게시한 상태라면, 일반 종속성으로 추가해야 합니다. 그렇지 않다면, 아래와 같이 `path` 종속성으로 추가할 수 있습니다:

```toml
    [dependencies]
    hello_macro = { path = "../hello_macro" }
    hello_macro_derive = { path = "../hello_macro/hello_macro_derive" }
```

절차적 매크로 예제에 작성된 코드를 _src/main.rs_에 넣고 `cargo run`을 실행하면, `Hello, Macro! My name is Pancakes!`가 출력되어야 합니다. `pancakes` 크레이트가 `HelloMacro` 트레이트를 구현하지 않고도 절차적 매크로가 트레이트 구현을 추가하였습니다. `#[derive(HelloMacro)]`에 의해 트레이트 구현이 추가되었습니다.

다음으로, 다른 종류의 절차적 매크로가 사용자 정의 `derive` 매크로와 어떻게 다른지 살펴보겠습니다.