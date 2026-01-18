# 🚀 POLYGLOT-AGENT OS

## 1. Project Vision

**Polyglot-Agent OS** là một hệ sinh thái học ngoại ngữ (**Nhật – Anh – Trung**), được thiết kế **cho người học Việt Nam**, vận hành như một **Learning Operating System** thay vì một ứng dụng học tập truyền thống.

Hệ thống vận hành dựa trên kiến trúc **Event-Driven Microservices**, nơi các **AI Agents** tự động:

* Trích xuất tri thức từ dữ liệu thô (Báo chí, Manga, Video, Audio)
* Chuyển hóa nội dung thành các *Memory Objects*
* Tối ưu hóa lộ trình ghi nhớ cá nhân hóa thông qua các thuật toán Spaced Repetition nâng cao

Mục tiêu cuối cùng là mô phỏng cách con người học ngôn ngữ trong môi trường tự nhiên, nhưng với khả năng mở rộng và tối ưu của AI.

---

## 2. Core Philosophy

* **Learning as an OS**: Học tập là một hệ điều hành với kernel, process, memory và event bus
* **Input-driven learning**: Học từ nội dung thực tế, không phụ thuộc giáo trình cố định
* **Agentic Architecture**: AI Agents hoạt động bất đồng bộ, có khả năng tự phối hợp
* **Separation of Determinism & Stochasticity**:

  * Rust: logic ngôn ngữ, thuật toán, dữ liệu chuẩn xác
  * Python: AI reasoning, LLM, workflow linh hoạt

---

## 3. System Architecture Overview

Áp dụng **Clean Architecture** kết hợp **Domain-Driven Design (DDD)** trên nền **Microservices**.

### 3.1 Core Services

#### 🧠 Core Linguistic Service (Rust)

**Vai trò:** Learning Kernel của hệ thống

Chức năng chính:

* Tokenization (JP / EN / ZH)
* Kanji analysis (radical, meaning, phonetic)
* Quản lý Spaced Repetition System (SRS)
* Vector search & semantic similarity

Mỗi đơn vị kiến thức được mô hình hóa như một **Memory Object**:

```
MemoryObject {
  linguistic_unit
  difficulty_score
  modality_weights
  last_recall_quality
  decay_curve
}
```

---

#### 🤖 AI Tutor Orchestrator (FastAPI)

**Vai trò:** Agent Runtime & Process Scheduler

Chức năng:

* Điều phối Agentic Workflows
* Prompt Engineering có trạng thái (stateful prompts)
* Tích hợp LLM (Gemini / Claude)
* Giao tiếp với Core Linguistic Service qua gRPC

Không chỉ là API server, mà là **AI Agent Operating Layer**.

---

#### 🔄 Event Backbone (Apache Kafka)

**Vai trò:** System Bus & Learning Event Log

Cung cấp:

* Xử lý bất đồng bộ các tác vụ AI nặng
* Replay & audit learning events
* Tách biệt UX latency và AI processing latency

Ví dụ luồng sự kiện:

```
INGEST_REQUEST → TOKENIZED → MEMORY_OBJECT_CREATED → SRS_UPDATED
```

---

#### 🌐 Real-time Gateway (Next.js)

**Vai trò:** Userland Interface

Chức năng:

* UI học tập & theo dõi tiến trình
* WebSocket cho phản hồi real-time
* Auth & session management

UI được định hướng như **Learning Console**, không chỉ là app học từ vựng.

---

## 4. Technology Stack

| Layer             | Technology               | Rationale                                  |
| ----------------- | ------------------------ | ------------------------------------------ |
| High-Perf Backend | Rust (Axum, Tonic)       | Tốc độ cao, an toàn bộ nhớ, xử lý ngôn ngữ |
| AI Backend        | Python (FastAPI)         | Hệ sinh thái AI mạnh, tích hợp nhanh       |
| Messaging         | Apache Kafka             | Event-driven, scalable, fault-tolerant     |
| Storage           | PostgreSQL + pgvector    | Quan hệ + semantic search                  |
| In-memory         | Redis                    | Cache & AI session state                   |
| Communication     | gRPC + Protobuf          | Giao tiếp nội bộ hiệu suất cao             |
| Frontend          | Next.js, Tailwind, Clerk | Modern UI, Auth tinh gọn                   |
| DevOps            | Docker, Kubernetes       | Chuẩn hóa & scale microservices            |

---

## 5. Key Features

> Định hướng thiết kế: **Người Việt học đa ngôn ngữ Á – Âu (JP / EN / ZH)**, tận dụng lợi thế ngôn ngữ mẹ đẻ để giảm cognitive load.

### 5.1 AI-Powered Ingestion

* Tự động đọc hiểu tài liệu người dùng cung cấp
* Tách từ vựng, cấu trúc ngữ pháp
* Sinh flashcard & bài tập ngay lập tức

---

### 5.2 Shadowing Agent

* Phân tích giọng nói người học
* So sánh pitch, accent với người bản xứ
* Feedback trực tiếp + ảnh hưởng đến SRS weighting

---

### 5.3 Cross-Language Bridge (Vietnamese-centric)

* Học tiếng Nhật / Trung / Anh thông qua **giải thích tiếng Việt**, hoặc chuyển tiếp qua Anh để tối ưu tư duy đa ngôn ngữ
* Tối ưu tư duy song ngữ, không đơn thuần là dịch

---

### 5.4 Resilient Background Processing

* Xử lý hàng trăm trang sách / video dài mà không block UX
* Toàn bộ pipeline dựa trên Kafka events

---

## 6. Strengths & Technical Advantages

### 6.1 Separation of Concerns

* Rust đảm nhiệm vai trò **Learning Kernel** (SRS, Tokenization)
* Phù hợp cho xử lý logic nặng của tiếng Nhật (Kanji) và tiếng Trung (Hanzi)
* Đảm bảo hiệu năng, an toàn bộ nhớ và tính quyết định (determinism)

### 6.2 Event-Driven Design

* Kafka giúp giải quyết triệt để vấn đề **AI latency**
* Người dùng nhận phản hồi UI ngay lập tức, AI xử lý nền bất đồng bộ

### 6.3 Vietnamese-centric USP

* Khai thác lợi thế **Hán–Việt** trong việc học JP / ZH
* Đây là **Unique Selling Point** rất mạnh cho thị trường Việt Nam

---

## 7. Refined Roadmap

### Phase 0 – The Skeleton (Week 1–2)

* Thiết lập **Monorepo** và CI/CD cơ bản
* Dựng hạ tầng bằng **Docker Compose**:

  * Kafka (ưu tiên **Redpanda** để nhẹ)
  * PostgreSQL
  * Redis
* Định nghĩa **.proto contracts** (gRPC) dùng chung giữa Rust và Python

---

### Phase 1 – Minimal Lovable Kernel (Week 3–5)

**Rust**

* Tokenizer cho **một ngôn ngữ duy nhất** (ưu tiên tiếng Nhật)
* Triển khai SRS cơ bản

Công thức Forgetting Curve (mô hình sơ bộ):

$$R = e^{-rac{t}{S}}$$

Trong đó:

* $R$: khả năng ghi nhớ
* $S$: độ bền trí nhớ (stability)
* $t$: thời gian

**Python**

* Agent đơn giản để trích xuất text từ PDF / Text thô

---

### Phase 2 – The Event Flow (Week 6–8)

* Kết nối các service qua Kafka
* Bắt đầu với **Pipeline cố định**, chưa triển khai Autonomous Agents
* Mục tiêu: kiểm soát được data flow và learning events

---

### Phase 3 – Voice & Shadowing

* Phân tích phát âm, pitch accent
* Feedback ảnh hưởng trực tiếp tới SRS weighting

---

## 8. Deep Technical Advice

### 8.1 Memory Object as a Kernel Entity

* Mỗi đơn vị kiến thức được mô hình hóa thành **Memory Object** với:

  * `decay_curve`: đường cong sụt giảm trí nhớ
  * `modality_weights`: trọng số theo giác quan (đọc / nghe / viết / nói)

Cách tiếp cận này cho phép hệ thống:

* Hiểu người học yếu ở kỹ năng nào
* Tự động điều chỉnh Agent và lịch SRS theo từng modality

Đây là nền tảng **khoa học nhận thức (cognitive science–driven)** của toàn bộ Learning OS.

---

### 8.2 Vietnamese-centric Hán–Việt Module

* Map **Kanji (JP)** và **Hanzi (ZH)** qua âm **Hán–Việt**
* Giảm 50–70% cognitive load khi ghi nhớ nghĩa từ
* Module này phải nằm trong **Core Linguistic Service (Rust)** để đảm bảo:

  * Tính quyết định
  * Khả năng tái sử dụng cho nhiều Agent

---

### 8.3 Phase 1 Focus – Internal Contract First

Trong giai đoạn **Minimal Lovable Kernel**, ưu tiên cao nhất không phải là code logic, mà là:

> **Định nghĩa Protobuf (.proto) như “luật chơi” giữa Rust và Python**

* `MemoryObject`
* `ReviewResult`
* `IngestRequest`

Việc định nghĩa chặt chẽ giúp:

* Tránh lệch dữ liệu giữa các service
* Cho phép evolve hệ thống mà không phá vỡ code

---

### 8.4 Kafka Buffer Strategy (Phase 2)

Vì AI Agent có độ trễ không xác định (stochastic latency):

**Chiến lược đề xuất**:

* Kafka dùng cho xử lý nền
* Redis Pub/Sub hoặc WebSocket để push kết quả real-time

Luồng:

1. User gửi request → nhận `task_id`
2. Kafka xử lý → ghi DB
3. Event hoàn tất → Gateway push kết quả qua WebSocket

→ UX mượt, không cần reload trang

---

### 8.5 Shadowing & Audio Processing (Phase 3)

Audio là dữ liệu nặng, **không nên đẩy trực tiếp qua Kafka**.

Giải pháp:

* Next.js Gateway upload audio lên **S3-compatible storage** (MinIO)
* Kafka chỉ mang **URL + metadata**
* Python service fetch audio từ URL để phân tích Pitch Accent

---

### 8.6 Rust Crates Recommendation (Phase 1)

* **Tokenization (JP)**: `lindera` hoặc `mecab-rs`
* **gRPC**: `tonic`
* **Database**: `sqlx` (PostgreSQL)
* **Kafka**: `rdkafka`

Các crate này đều production-grade và phù hợp với kiến trúc kernel.

---

### 8.7 Spaced Repetition Formula (Rust)

Cập nhật độ bền trí nhớ:

$$S_{new} = S_{old} \times (1 + factor \times quality_score)$$

Trong đó:

* `quality_score`: đánh giá recall (0–5)
* `factor`: hệ số điều chỉnh theo modality

Việc triển khai trong Rust cho phép:

* Xử lý hàng chục nghìn Memory Object
* Thời gian phản hồi ở mức **micro-second**

---

### 8.8 Thread-Safe SRS Engine (Phase 1 Critical)

Module **SRS** trong Rust **bắt buộc phải thread-safe**, vì:

* Review có thể chạy song song (background jobs)
* Agent Python có thể gọi gRPC đồng thời

Chiến lược đề xuất:

* Sử dụng `Arc<Mutex<T>>` cho trạng thái SRS dùng chung
* Hoặc ưu tiên **lock-free / fine-grained locking** với:

  * `DashMap`
  * `RwLock` (read-heavy workload)

Nguyên tắc thiết kế:

* **Read nhiều – Write ít**
* Không giữ lock khi tính toán dài
* Tách biệt logic tính toán SRS và persistence layer

Cách tiếp cận này cho phép:

* Scale tới **hàng triệu Memory Object**
* Tránh race condition
* Đảm bảo tính quyết định của Learning Kernel

---

## 9. Risk & Mitigation

### Over-engineering sớm

→ Luôn bắt đầu từ **Skeleton + Kernel**, tránh agent hóa quá sớm

### Agent bùng nổ không kiểm soát

→ Agent stateless, toàn bộ sự thật nằm ở Core Linguistic Service

---

## 10. Conclusion

**Polyglot-Agent OS** là một **Vietnamese-first, Agentic Learning Platform** với chiều sâu kỹ thuật cao.

Chỉ cần hiện thực hóa đúng trục kiến trúc (30–40%), hệ thống đã đạt mức **research-grade** hoặc **startup-grade**.

### Phase 1 – Learning Kernel MVP

* Rust: Tokenization + Basic SRS
* Python: Text ingestion + flashcard generation

### Phase 2 – Agentization

* Kafka
* Multiple ingestion & exercise agents

### Phase 3 – Voice & Shadowing

* Speech analysis
* Feedback loop → Memory decay optimization

---

## 8. Conclusion

**Polyglot-Agent OS** không phải một ứng dụng học ngoại ngữ, mà là một **Agentic Learning Platform** mang tính nghiên cứu và startup-grade.

Chỉ cần hiện thực hóa đúng kiến trúc ở mức 30–40%, hệ thống đã có giá trị rất lớn về cả công nghệ lẫn học thuật.
