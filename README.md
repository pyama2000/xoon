XOON
===

## 構成図

```
┌──────┐
│ XOON │
├──────┴────────────────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                                                                                                   │
│  ┌────────┐     1. Request ┌─────────────┐                                                                        │
│  │        ├───────────────►│             │                                                                        │
│  │ CLIENT │                │ API GATEWAY │                                               ┌─────────────────────┐  │
│  │        │◄───────────────┤             │                                               │                     │  │
│  └────────┘ 4. Response    └────┬────────┘                                        ┌─────►│ K8S MANAGER SERVICE │  │
│                                 │       ▲ 3. Add events response                  │      │                     │  │
│                                 │       │                                         │      └─┬───────────────────┘  │
│                                 │       │                                         │        │ 6. Send Slack        │
│                                 │       │                                         │        │    message request   │
│           2. Add events request ▼       │                                         │        ▼                      │
│                                ┌────────┴───────────────────────┐                 │      ┌─────────────────────┐  │
│                                │                                │ 5. Send events  │      │                     │  │
│                                │ ASYNCHRONOUS MESSAGING SERVICE ├─────────────────┼─────►│ SLACK SERVICE       │  │
│                                │                                │                 │      │                     │  │
│                                └────────────────────────────────┘                 │      └─────────────────────┘  │
│                                                                                   │        ▲                      │
│                                                                                   │        │ 6. Send Slack        │
│                                                                                   │        │    message request   │
│                                                                                   │      ┌─┴───────────────────┐  │
│                                                                                   │      │                     │  │
│                                                                                   └─────►│ KINTAI SERVICE      │  │
│                                                                                          │                     │  │
│                                                                                          └─────────────────────┘  │
│                                                                                                                   │
└───────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘
```

### 登場人物

#### client

SlackやGoogle Homeなど, 外部から呼び出すサービス

#### API Gateway

clientのリクエストを判定して, **Asynchronus Messaging Service** にイベントの登録をリクエストする

### Services

#### Asynchronus Messaging Service

**API Gateway** からイベント登録のリクエストを受け取り, 登録できたら **API Gateway** にレスポンスを返す.
**Asynchronus Messaging Service** は一定間隔で溜まっているイベントを各サービスに依頼する. 

#### k8s Manager Service

Kubernetes のポッドなどを管理するサービス. 

#### Slack Service

Slackにメッセージを送るサービス. 

#### Kintai Service

勤怠管理サービス. 
