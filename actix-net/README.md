`actix-net` 的各个部分之间有一定的依赖关系，这些模块相互协作以提供完整的网络功能。

1. **actix_rt**:
    - **依赖项**：作为运行时，它是基础设施层，许多其他模块依赖于它来执行异步任务和计时器。
    - **依赖于**：Tokio runtime。

2. **actix_service**:
    - **依赖项**：提供服务抽象的核心模块，它定义了服务和中间件的接口。
    - **依赖于**：这个模块依赖于 `actix_rt` 来运行服务和中间件。

3. **actix_codec**:
    - **依赖项**：用于消息的编码和解码，帮助处理网络协议。
    - **依赖于**：依赖 `actix_service` 提供的服务接口，并使用 `tokio_util::codec` 提供的编码器和解码器基础设施。

4. **actix_connect**:
    - **依赖项**：管理网络连接的建立和维护，特别是 TCP 连接。
    - **依赖于**：依赖 `actix_rt` 来进行异步连接管理，并使用 `actix_service` 来定义连接处理逻辑。

5. **actix_utils**:
    - **依赖项**：提供了一些通用工具和实用程序，这些工具和实用程序在整个框架中都有使用。
    - **依赖于**：通常依赖 `actix_rt` 和 `actix_service` 提供的基础设施。

6. **actix_server**:
    - **依赖项**：提供构建异步服务器的基础设施，管理服务器实例和客户端连接。
    - **依赖于**：依赖 `actix_rt` 来运行服务器循环，依赖 `actix_service` 定义的服务来处理连接，并依赖 `actix_codec` 来处理消息编码和解码。

7. **actix_tls**:
    - **依赖项**：为网络通信提供 TLS 支持。
    - **依赖于**：依赖 `actix_connect` 来管理安全连接，并使用 `actix_service` 提供的服务接口。

8. **actix_testing**:
    - **依赖项**：提供测试工具，允许模拟和测试网络服务。
    - **依赖于**：依赖 `actix_rt` 和 `actix_service` 提供的基础设施。

### 依赖关系图

以下是一个简单的依赖关系图：

```
          +---------------------+
          |     actix_rt        |
          +---------------------+
                   |
  +----------------|----------------+
  |                |                |
+-----------+  +-------------+  +-----------+
|actix_utils|  |actix_service|  |actix_codec|
+-----------+  +-------------+  +-----------+
  |                |                |
  |                |                |
+----------+   +-------------+  +------------+
|actix_tls |   |actix_connect|  |actix_server|
+----------+   +-------------+  +------------+
                   |
                   |
             +-------------+
             |actix_testing|
             +-------------+
```

1. `actix_rt` 作为基础，所有模块都依赖它来执行异步任务。
2. `actix_service` 是服务和中间件的核心模块，许多其他模块依赖它来定义和组合服务。
3. `actix_codec` 提供编码和解码功能，依赖于 `actix_service` 和 `actix_rt`。
4. `actix_connect` 和 `actix_server` 管理连接和服务器，依赖 `actix_rt` 和 `actix_service`。
5. `actix_tls` 提供 TLS 支持，依赖于 `actix_connect` 和 `actix_service`。
6. `actix_testing` 提供测试工具，依赖于 `actix_rt` 和 `actix_service`。
7. `actix_utils` 提供通用工具，依赖于 `actix_rt` 和 `actix_service`。

## Something Interesting
1. `actix-rt`依赖与tokio，从而依赖于操作系统提供的运行时。但是`actix-serice`设计为no_std，
这意味即使在嵌入式操作系统或者无操作系统时，只要提供一个异步运行时，即使时no_std的，那么`actix-service`也会正常工作。

