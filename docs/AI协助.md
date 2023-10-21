# AI 协助

## 提供示例

```text
我希望你担任 Rust 编程语言嵌入式开发专家。指引我使用 stm32f1xx-hal="0.10.0" 嵌入式框架进行开发学习，请确保给出的示例的正确性。
请提供一个 rtic EXTI1 中断点亮LED灯的示例，请确保给出的示例的正确性。
```

## 提供示例

```
我希望你担任 Rust 编程语言嵌入式开发专家。指引我使用 stm32f1xx-hal="0.10.0" 嵌入式框架进行开发学习。
请提供一个通过中断进行对射式红外传感器计数的示例，并确保正确。
```

## 修复错误

```
我希望你担任 Rust 编程语言嵌入式开发专家。指引我使用 stm32f1xx-hal="0.10.0" 嵌入式框架进行开发学习，请确保给出的示例的正确性。
请帮我分析下面代码有什么问题，设置的窗口时间为30-50ms,然而在 println!("delay..") 位置的就已经重置了.
```

## 代码转换

```
我希望你担任 Rust 编程语言嵌入式开发专家。指引我使用 stm32f1xx-hal="0.10.0" 嵌入式框架进行开发学习，请确保给出的示例的正确性。
以下C语言嵌入式代码转换为Rust语言stm32f1xx-ha对应的代码，并确保正确。
	RCC_APB1PeriphClockCmd(RCC_APB1Periph_WWDG, ENABLE);

    RCC_GetFlagStatus(RCC_FLAG_WWDGRST)
	
	WWDG_SetPrescaler(WWDG_Prescaler_8);
	WWDG_SetWindowValue(0x40 + 21);			//30ms
	WWDG_Enable(0x40 + 54);					//50ms


    WWDG_SetCounter(0x40 + 54);
```

## 细节示例

```
我希望你担任 Rust 编程语言嵌入式开发专家。指引我使用 stm32f1xx-hal="0.10.0" 嵌入式框架进行开发学习，请确保给出的示例的正确性。
以下是RTC 告警中断的示例，请帮我补充todo信息，并确保正确。
```
