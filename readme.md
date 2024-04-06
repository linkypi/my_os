<!--
 * @Author: LinkyPi trouble.linky@gmail.com
 * @Date: 2024-04-06 17:26:49
 * @LastEditors: LinkyPi trouble.linky@gmail.com
 * @LastEditTime: 2024-04-06 17:45:20
 * @FilePath: /myos/readme.md
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
-->

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