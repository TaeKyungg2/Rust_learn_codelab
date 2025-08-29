## IDE 마스터하기: 조건 관리하기

`main.rs` 파일에서 작성된 코드가 무엇을 출력하는지 빠르게 이해할 수 있나요?

%IDE_NAME%는 코드 가독성을 높이는 데 도움이 되는 유용한 기능을 많이 제공합니다.

예를 들어, [드모르간의 법칙](https://en.wikipedia.org/wiki/De_Morgan%27s_laws)의 **빠른 수정(quick-fix)**을 적용해 봅시다.

커서를 `&&` 위에 놓고 &shortcut:ShowIntentionActions; 바로 가기를 사용하여 사용 가능한 컨텍스트 작업을 확인하세요.

![](image.png)

이제 `main.rs` 파일에서 작성된 코드가 무엇을 출력하는지 빠르게 이해할 수 있나요?

<div class="hint">
  최종적으로 if 조건문은 다음과 같은 형태여야 합니다.<br>
  <code>(number > 4 && number <= 9) || (number > 0 && number < 10)</code>
</div>

<div class="hint">
  이 코드는 "If Branch"를 출력합니다.
</div>

*참고 1*: `&&`와 `||`는 [단락 논리 연산자](https://en.wikipedia.org/wiki/Short-circuit_evaluation)입니다.

*참고 2*: 이전 작업에서 작성한 코드의 다양한 위치에서 &shortcut:ShowIntentionActions;를 시도해 보세요.  
%IDE_NAME%는 코드 품질을 개선하는 데 유용한 제안을 자주 제공합니다.