<!--
 * @Author: LinkyPi trouble.linky@gmail.com
 * @Date: 2024-04-06 17:26:49
 * @LastEditors: LinkyPi trouble.linky@gmail.com
 * @LastEditTime: 2024-04-06 18:10:01
 * @FilePath: /myos/readme.md
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
-->

##  1. 准备事项
[参考项目链接](https://os.phil-opp.com/zh-CN/freestanding-rust-binary/)

### 为 rsut 增加平台支持
``` shell
rustup target add thumbv7em-none-eabi
```

``` shell
cargo build --target thumbv7em-none-eabi
```

### Linux 系统下编译
``` shell
cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"

```

## 2. 开始编写最小内核
[参考](https://os.phil-opp.com/zh-CN/minimal-rust-kernel/)

``` json
"features": "-mmx,-sse,+soft-float",
```
mmx 和 sse 特征决定了是否支持单指令多数据流 （Single Instruction Multiple Data，SIMD）相关指令，这些指令常常能显著地提高程序层面的性能。然而，在内核中使用庞大的 SIMD 寄存器，可能会造成较大的性能影响：因为每次程序中断时，内核不得不储存整个庞大的 SIMD 寄存器以备恢复 —— 这意味着，对每个硬件中断或系统调用，完整的 SIMD 状态必须存到主存中。由于 SIMD 状态可能相当大（512~1600 个字节），而中断可能时常发生，这些额外的存储与恢复操作可能显著地影响效率。为解决这个问题，我们对内核禁用 SIMD（但这不意味着禁用内核之上的应用程序的 SIMD 支持）。

禁用 SIMD 产生的一个问题是，x86_64 架构的浮点数指针运算默认依赖于 SIMD 寄存器。我们的解决方法是，启用 soft-float 特征，它将使用基于整数的软件功能，模拟浮点数指针运算。