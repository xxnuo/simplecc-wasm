# simplecc-wasm


> 基于 [simplecc-rs](https://github.com/sorz/simplecc-rs) 的 opencc 简单替代

_编译成 WASM 摆脱编译 Node Addon 的烦恼_

```bash
npm i simplecc-wasm
```

# Usage

```js
// Node.js
import { simplecc } from "simplecc-wasm";
// const { simplecc } = require("simplecc"); // commonjs
simplecc("发财了去植发", "s2t"); // '發財了去植髮'

// web
import init, { simplecc } from "simplecc-wasm";
await init();
simplecc("发财了去植发", "s2t"); // '發財了去植髮'
// There is a react app in demo/web path
```

## 配置 Configurations

[Opencc 配置中的一部分](https://github.com/BYVoid/OpenCC#configurations-%E9%85%8D%E7%BD%AE%E6%96%87%E4%BB%B6)

```plain
"s2t"
"t2s"
"s2tw"
"s2hk"
"hk2s"
"s2twp"
"tw2sp"
```
