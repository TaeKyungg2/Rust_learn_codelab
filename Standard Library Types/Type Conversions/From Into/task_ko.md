## From Into

From 트레잇은 값-대-값 변환(value-to-value conversions)에 사용됩니다.  
From이 어떤 타입에 올바르게 구현되었다면, Into 트레잇은 반대로 동작해야 합니다.  
자세한 내용은 https://doc.rust-lang.org/std/convert/trait.From.html 에서 확인할 수 있습니다.  

우리는 Default 트레잇을 구현함으로써 제공된 문자열을 Person 객체로 변환할 수 없을 경우 이를 대체 방안으로 사용할 수 있습니다.  
당신의 작업은 이 구현을 완료하여 `let p = Person::from("Mark,20")` 줄이 컴파일되도록 만드는 것입니다.  
나이의 요소를 `usize`로 파싱해야 하며, 이때 `"4".parse::<usize>()`와 같은 방법을 사용할 수 있습니다.  
이 과정에서 나오는 결과는 적절히 처리되어야 합니다.  

단계:
1. 제공된 문자열의 길이가 0이면 Person의 기본값(default)을 반환하십시오.
2. 주어진 문자열을 포함된 쉼표(,)를 기준으로 나누십시오.
3. 나눈 결과의 첫 번째 요소를 추출하여 이름으로 사용하십시오.
4. 이름이 비어 있으면 Person의 기본값(default)을 반환하십시오.
5. 나눈 결과의 다른 요소를 추출하여 이를 나이로 `usize`로 파싱하십시오. 나이를 파싱하는 중 문제가 발생하면 Person의 기본값(default)을 반환하십시오. 그렇지 않다면 결과로 인스턴스화된 Person 객체를 반환하십시오.  

<div class="hint"> `TryFrom` 구현 직전에 제공된 단계를 따르세요.  
또한 이 <a href="https://doc.rust-lang.org/std/convert/trait.TryFrom.html">예제</a>를 사용할 수도 있습니다. </div>