# Rust Learning Repository

A comprehensive Rust learning repository containing projects, programs, questions, and examples covering various Rust concepts from basics to advanced topics.

---

## 📁 Repository Structure

### **Core Learning Modules**

#### 1. **simple_data_type/** - Basic Data Types
Learning project focused on Rust's fundamental data types (integers, floats, booleans, strings, etc.)
- `Cargo.toml` - Project configuration
- `src/main.rs` - Basic data type examples

#### 2. **Compound_data_Types/** - Compound Data Types
Covers arrays, tuples, and other compound types in Rust
- `Cargo.toml` - Project configuration
- `src/main.rs` - Compound type examples and usage

#### 3. **Rust_session/** - Session Practice
Practice projects and session-based exercises
- `Cargo.toml` - Project configuration
- `src/main.rs` - Session examples

---

### **Advanced Topics**

#### 4. **declarative_macros/** - Declarative Macros
Learning declarative macros in Rust (macro_rules!)
- `Cargo.toml` - Project configuration
- `src/lib.rs` - Macro library definitions
- `src/main.rs` - Macro usage examples

#### 5. **procedural_macros/** - Procedural Macros
Advanced procedural macros for custom syntax and code generation
- `Cargo.toml` - Project configuration
- `src/lib.rs` - Procedural macro definitions
- `src/main.rs` - Macro usage examples
- `target/` - Build artifacts

#### 6. **Rust_app_cpp_lib/** - Rust with C++ Integration (Library)
FFI (Foreign Function Interface) - calling C++ functions from Rust
- `build.rs` - Build script
- `Cargo.toml` - Project configuration with C++ dependencies
- `src/main.rs` - Rust code
- `src_lib/lib.cpp` - C++ library code
- `read.txt` - Documentation

#### 7. **Rust_app_with_cpp/** - Rust with C++ Integration (Direct)
FFI implementation with direct C++ integration
- `build.rs` - Build script
- `Cargo.toml` - Project configuration
- `src/FFI.cpp` - C++ FFI code
- `src/main.rs` - Rust FFI bindings

#### 8. **rust_CPP/** - Rust-C++ Interoperability
Additional FFI and interoperability examples
- `build.rs` - Build script
- `Cargo.toml` - Project configuration
- `src/file.cpp` - C++ source
- `src/main.rs` - Rust integration code

---

### **Main Learning: Problem Questions**

#### 9. **Program_Question/** - Core Problem Set (21+ Questions)

A structured set of 21 progressive Rust questions covering essential concepts and topics:

##### **Basic Questions (1-5)**
| # | Topic | Concepts |
|---|-------|----------|
| 1 | Ownership and Borrowing | References, Borrowing, String slices |
| 2 | Mutable Variables | Mutability, Variable declaration |
| 3 | If-Else Conditions | Control flow, Comparison operators |
| 4 | Pattern Matching | Match expressions, Enum patterns |
| 5 | Struct and Method | Structs, Methods, Implementation blocks |

##### **Intermediate Questions (6-15)**
| # | Topic | Concepts |
|---|-------|----------|
| 6 | Traits and Implementation | Traits, Trait implementation, Method resolution |
| 7 | Generics | Generic functions, Generic structs, Type parameters |
| 8 | Lifetimes | Lifetime annotations, Reference lifetimes |
| 9 | Error Handling | Result, Error types, Error propagation |
| 10 | Module and Visibility | Modules, pub keyword, Module organization |
| 11 | Closures | Closure syntax, Capture mechanisms, Fn traits |
| 12 | Iterator | Iterator trait, Functional programming |
| 13 | Collection Types | Vec, HashMap, HashSet, BTreeMap |
| 14 | String Operations | String manipulation, Slicing, Methods |
| 15 | Type Conversions | From, Into, as keyword, TryFrom |

##### **Advanced Questions (16-21)**
| # | Topic | Concepts |
|---|-------|----------|
| 16 | Smart Pointers | Box, Rc, Arc, RefCell, Mutex |
| 17 | Concurrency | Threads, Message passing, Synchronization |
| 18 | Macros | Macro rules, Procedural macros |
| 19 | Associated Types | Traits with associated types |
| 20 | Trait Objects | Dynamic dispatch, dyn keyword |
| 21 | Advanced Traits | Trait bounds, Higher-ranked trait bounds |

##### **Topics and Concepts Files**
- `main.rs_Declarative_Macro` - Declarative macro examples
- `main.rs_Doubly_LinkList` - Doubly linked list implementation
- `main.rs_dynamic_dispatch` - Dynamic dispatch and trait objects
- `main.rs_file_reading` - File I/O operations
- `main.rs_macro_fn_inpu` - Macro functions and input handling
- `main.rs_RefCellPtr` - RefCell pointer usage
- `main.rs_Refrence_Pointer` - Reference and pointer patterns
- `main.rs_SlinkList_BoxPtr` - Single linked list with Box pointers
- `main.rs_Thread_local_Var` - Thread-local variables
- `main.rs_thread_Mtx_Conv` - Threading with mutex and synchronization
- `main.rs_trait_impl` - Trait implementation patterns
- `main.rs_box_ptr_Drop_trait` - Box pointers and Drop trait

##### **Advanced Question Set (Advance_que/)**
- `main.rs_enum` - Enumeration patterns and usage
- `main.rs_String` - Advanced string handling
- `main.rs_struct_Hash` - Structs with hashing
- `main.rs_Trait` - Trait advanced concepts

---

### **Coding Challenge Solutions**

#### 10. **solutions/** - Algorithm Solutions
Solutions to coding challenge problems (Exercism style)
- `rust/armstrong-numbers/` - Armstrong number solutions
- `rust/gigasecond/` - Time arithmetic solutions
- `rust/hello-world/` - Hello world implementations
- `rust/reverse-string/` - String reversal solutions

---

## 🎯 Learning Path

### Beginner (Questions 1-5)
- Understand ownership and borrowing
- Master variable mutability
- Learn control flow
- Work with basic structs

### Intermediate (Questions 6-15)
- Implement traits and generics
- Handle errors properly
- Use closures and iterators
- Organize code with modules

### Advanced (Questions 16-21)
- Master smart pointers
- Implement concurrency
- Create and use macros
- Implement advanced trait patterns

---

## 🚀 Getting Started

### Run a Specific Question
```bash
cd Program_Question
cat src/main.rs_Que_1  # View question
cargo run --bin main  # Run current main.rs
```

### Run Any Project
```bash
cd <project-folder>
cargo run
```

### Build Documentation
```bash
cargo doc --open
```

---

## 📚 Topics Covered

- **Ownership & Borrowing**
- **Pattern Matching**
- **Structs & Traits**
- **Generics & Lifetimes**
- **Collections & Iterators**
- **Error Handling**
- **Closures & Functional Programming**
- **Modules & Visibility**
- **Smart Pointers**
- **Concurrency & Threading**
- **Macros (Declarative & Procedural)**
- **FFI & C++ Integration**
- **Advanced Trait Patterns**
- **File I/O Operations**
- **Type Conversions**

---

## 📝 Notes

- Each question file in `Program_Question/src/` contains detailed problem statements and concepts
- Use `main.rs_*` files as reference implementations
- Comments in code explain key concepts
- Build and run to see working examples

---

**Last Updated:** April 2026