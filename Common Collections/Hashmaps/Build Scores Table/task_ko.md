## 점수표 생성하기

주어진 것은 축구 경기의 점수 리스트이며, 각 줄에는 한 경기의 점수가 기록되어 있습니다. 각 줄의 형식은 다음과 같습니다:
```
<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>
```

예시: `England,France,4,2` (잉글랜드가 4골, 프랑스가 2골을 기록함).

팀의 이름, 팀이 득점한 골 수, 팀이 허용한 골 수를 포함하는 점수표를 작성해야 합니다. 점수표를 작성하는 한 가지 방법은 Hashmap을 사용하는 것입니다. 이 솔루션은 Hashmap을 사용하는 방식으로 일부 작성되어 있으니, 테스트를 통과할 수 있도록 완성하세요.

테스트를 통과하도록 만들어주세요!

<div class="hint">
점수표에서 각 팀에 해당하는 항목을 삽입하려면 <code>HashMap</code>의 <code>entry()</code> 및 <code>or_insert()</code> 메서드를 사용하세요.

자세한 내용은 [The Book](https://doc.rust-lang.org/stable/book/ch08-03-hash-maps.html#only-inserting-a-value-if-the-key-has-no-value)에서 확인하세요.
</div>

<div class="hint">
주어진 키에 해당하는 항목이 이미 있는 경우, <code>entry()</code>에서 반환된 값은 기존 값을 기반으로 업데이트될 수 있습니다.

자세한 내용은 [The Book](https://doc.rust-lang.org/book/ch08-03-hash-maps.html#updating-a-value-based-on-the-old-value)에서 확인하세요.
</div>