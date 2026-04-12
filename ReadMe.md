# rbdc-xugu

[![github](https://img.shields.io/badge/Github-BrightX/rbdc--xugu-blue?logo=github)](https://github.com/BrightX/rbdc-xugu)
[![gitee](https://img.shields.io/badge/Gitee-BrightXu/rbdc--xugu-8da0cb?labelColor=C71D23&logo=gitee)](https://gitee.com/BrightXu/rbdc-xugu)
[![crate](https://img.shields.io/crates/v/rbdc-xugu.svg?logo=rust)](https://crates.io/crates/rbdc-xugu)
[![documentation](https://img.shields.io/badge/docs.rs-rbdc--xugu-66c2a5?labelColor=555555&logo=docs.rs)](https://docs.rs/rbdc-xugu)
[![minimum rustc 1.86](https://img.shields.io/badge/rustc-1.86+-red.svg?logo=rust)](https://rust-lang.github.io/rfcs/2495-min-rust-version.html)
![License](https://img.shields.io/crates/l/rbdc-xugu)

> 基于 `rbdc` 的 `rust` 虚谷数据库驱动。
>
> 支持虚谷数据库 `v11`/`v12`。

## Install

```toml
# Cargo.toml
rbdc-pool-fast = "4.8"
rbdc-xugu = "4.8"

tokio = { version = "1", features = ["full"] }
```

#### Cargo Feature Flags

- `lowercase_column_name`: 将 `MetaData` 返回的列名转为小写，默认启用。
  虚谷数据库查询结果集返回的列名和别名通常为大写，但大多数情况下 `rbatis` 的 `crud!` 注解的 struct 的字段
  编程习惯上都为小写下划线命名的。
  `serde` 进行反序列化时，需要通过 `#[serde(alias = "ID")]` 注解字段别名，来对应数据库返回的大写列名 `ID`。
  启用这个 feature 后，可免去这种大量的 `serde` 别名注解。

## Usage

### 快速入门

```rust
use rbdc_pool_fast::FastPool;
use rbdc_xugu::db::Connection;
use rbdc_xugu::pool::Pool;
use rbdc_xugu::XuguDriver;

#[tokio::main]
async fn main() -> Result<(), rbdc_xugu::Error> {
    let pool = FastPool::new_url(
        XuguDriver,
        "xugu://GUEST:GUEST@127.0.0.1:5138/SYSTEM?ssl=ssl",
    )?;
    let mut conn = pool.get().await?;
    let v = conn.exec_decode("SELECT * FROM your_tables", vec![]).await?;
    println!("{:?}", v);
    //if need decode use `let result:Vec<Table> = rbs::from_value(v)?;`
    Ok(())
}
```

rbdc-xugu 驱动url格式，支持以下几种

```
1. xugu://ip:port/databaseName[?property=value[&property=value]]
  * 例：xugu://127.0.0.1:5138/SYSTEM?user=GUEST&password=GUEST&version=301&time_zone=GMT

2. xugu://user:passwd@ip:port/databaseName[?property=value[&property=value]]
  * 例：xugu://GUEST:GUEST@127.0.0.1:5138/SYSTEM?version=301&time_zone=GMT

```

#### 连接参数

| 参数名                       | 说明                                                                                                                         | 初始值                |
|:--------------------------|:---------------------------------------------------------------------------------------------------------------------------|:-------------------|
| database                  | 数据库名                                                                                                                       |                    |
| user                      | 用户名                                                                                                                        |                    |
| password                  | 用户密码                                                                                                                       |                    |
| version                   | 服务器版本                                                                                                                      | 301                |
| encryptor                 | 数据库解密密钥                                                                                                                    |                    |
| charset                   | 客户端字符集(**utf8**或**gbk**)                                                                                                   | utf8               |
| lob_ret                   | 大对象返回方式                                                                                                                    |                    |
| time_zone                 | 客户端时区                                                                                                                      |                    |
| iso_level                 | 事务隔离级别                                                                                                                     | READ COMMITTED读已提交 |
| lock_timeout              | 最大加锁等候时间                                                                                                                   |                    |
| auto_commit               | 是否自动提交                                                                                                                     | on                 |
| return_rowid              | 是否返回**rowid**                                                                                                              | false              |
| return_schema             | 查询**SQL**是否返回模式信息（此参数存在一个疑问）                                                                                               | on                 |
| identity_mode             | 数据库服务端自增长使用模式（**DEFAULT**：**default**自增,**NULL_AS_AUTO_INCREMENT**：**NULL**自增,**ZERO_AS_AUTO_INCREMENT**：**0**和**NULL**自增） |                    |
| keyword_filter            | 数据库连接配置连接上需要开放的关键字串，已逗号分隔，例如**TABLE,FUNCTION,CONSTANT**                                                                    |                    |
| disable_binlog            | 不记载**binlog**日志                                                                                                            |                    |
| current_schema            | 指定连接的模式名                                                                                                                   |                    |
| compatible_mode           | 适配其他数据库(**MySQL/ORACLE/PostgreSQL**)                                                                                       | NONE               |
| use_ssl                   | 是否开启传输数据加密保护 `on`: 启用加密，`off`: 禁用加密                                                                                        | off                |
| ssl                       | 同上 `ssl=ssl`: 启用加密，`ssl=nssl`: 禁用加密                                                                                        | nssl               |
| statement-cache-capacity  | 单个连接会话上的最大prepared语句数（max_prepare_num） 取值范围 `[100, 2097152]`，不要超过数据库设置的值 `show max_prepare_num;`            | 100                |

### 更多请参考 `rbatis` 相关文档

* `rbatis-v4`: https://rbatis.github.io/rbatis.io/#/zh-cn/v4/
* `docs.rs`: https://docs.rs/rbatis/latest/rbatis/
* `Github`: https://github.com/rbatis/rbatis
* `crates.io`: https://crates.io/crates/rbatis

## 联系方式

- **Bug 反馈**: [GitHub Issues](https://github.com/BrightX/rbdc-xugu/issues)
- **一般讨论**: [GitHub Discussions](https://github.com/BrightX/rbdc-xugu/discussions)
- **商务合作**: [BrightXu666@163.com](mailto:BrightXu666@163.com)

## 许可证

[MIT](https://opensource.org/license/mit) or [Apache 2.0](https://opensource.org/licenses/Apache-2.0)

### 免责声明

**rbdc-xugu** 跟 **成都虚谷伟业科技有限公司** 不构成任何知识产权归属关系。这个程序不含任何担保。

软件以“现状”提供，不提供任何明示或暗示的保证，包括但不限于可销性、特定用途适用性和非侵权等保证。
无论如何，作者或版权持有人对因软件或软件使用或其他交易产生的、涉及合同、侵权或其他诉讼的索赔、损害或其他责任均不承担责任。
