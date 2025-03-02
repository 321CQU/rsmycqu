# rsmycqu

`rsmycqu`是重庆大学校园信息查询库`pymycqu`的rust版本，
几乎完全支持`pymycqu`中所有已经支持的API

目前已经实现以下功能

- 所有下列接口响应的数据模型
- 重庆大学单点登陆(`SSO`)
- 重庆大学教务网相关功能
    - 获取访问教务网API权限

**该库还在快速开发中，预计会在近期完成，届时我们会同步发布1.0版本至crates.io**

# 快速开始

我们封装了`reqwest`中的`Client`以保证库能正确运行并存储调用API所需的信息，因此，你需要首先创建一个
`crate::session::Session`

```rust
let session = crate::session::Session::new();
```

对于所有服务，你都需要先通过重庆大学单点登陆(SSO)验证

```rust
login(&mut session, "your_auth", "your_password", &false).await
```

值得一提的是，我们的所有API接口函数都是异步的，所以你需要一个异步框架来调用(比如`tokio`)，并记得添加`await`

所有接口都会返回某个`Result`，具体类型可以查看相应接口的文档

我们只在确保不会出现异常的地方使用`unwrap`，其余地方都使用`Result`包裹，因此你可以不用担心`panic`

**TODO: 添加成绩查询的相应示例**

# 贡献

我们非常欢迎PR，以下是一些注意事项

## 接口设计

### API接口函数

所有API接口都应当设计为函数，接受我们自定义的Session、所需参数并回传一个用Result包裹的结果。

例如，login的声明如下

```rust
pub async fn login(
    session: &mut Session,
    auth: &str,
    password: &str,
    force_relogin: &bool,
) -> SSOResult<LoginResult>
```

### 关于`Session`

因为本库需求`reqwest::Client`禁用自动重定向且启用`cookie_store`，我们通过new type实现了这点

不同模块登陆可能需要不同的`token`(如`mycqu`中需要设置http请求的`Authorization`头)，我们参照`reqwest`中的`HeaderMap`实现，设计了
`AccessInfo`，你可以查看源码了解更多信息。为了在接口调用间，这些信息被存储在`Session`中，**需要相应权限的接口应当在调用前检查相应权限，在缺少时报错并提前失败
**。

### 模块划分

我们依照网址将所有API切分为多个模块`sso`、`mycqu`、`card`、`lib`分别对应单点登陆、教务网、校园卡、图书馆，如果你想添加新的接口应当符合当前模块的设计。

同时，我们为每个模块提供了相应`feature`以支持仅使用需要的接口，默认`feature`包括`sso`、`mycqu`，在编写代码时应当考虑代码在不同
`feature`下的情况

### 错误类型

我们在`errors`中声明错误，并为每个模块声明了不同的错误类型，将常见错误提升至`Error`
枚举中，所有API接口对外暴露该类型，并通过泛型指定接口所在模块拥有的错误类型。

模块独有的错误类型会包裹在`Error::InnerError`中，常见错误则为`Error`中其他枚举项

*我们定义了内部`trait`——`error_handle_help`以简化模块间错误转换的问题(这主要出现在其他模块调用`sso`中的`access_service`
时需要进行错误转换)*

### 测试

我们为所有接口即重要或容易出现异常的工具函数编写了测试，在你编写或修改新接口时请别忘了添加/修改对应接口。

我们使用`rstest`以支持测试夹具，如果你想运行测试，你应当将`crate::utils::test_tools`中`login_data`夹具返回的账号密码修改为你自己的

**请不要在代码中上传自己的账号密码，这极容易造成数据泄漏**

# 许可

AGPL 3.0
