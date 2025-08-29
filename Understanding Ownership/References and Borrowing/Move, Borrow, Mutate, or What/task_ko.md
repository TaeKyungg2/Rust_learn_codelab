## 이동, 차용, 변형, 혹은 그 외

컴파일될 수 있도록 수정하세요! 함수의 인자 외에는 아무것도 변경하지 마세요.

<div class="hint">
<code>fn get_last_char</code> 함수가 소유권을 가져가면 안 됩니다.  
이유는 <code>message</code>가 <code>main</code> 함수에서 나중에 사용되기 때문입니다.  
대신 참조를 사용하여 차용할 수 있습니다.  
</div>

<div class="hint">
<code>fn uppercase_and_print</code> 함수는 <code>main</code>에서 마지막으로 호출되기 때문에  
<code>message</code>를 안전하게 이동시킬 수 있습니다. 따라서 참조를 전달할 필요는 없습니다.  

또한, 이 함수는 `message` 변수를 변경합니다. 따라서 이 변수를 가변적으로 선언해야 합니다.  
</div>